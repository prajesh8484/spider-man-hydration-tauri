use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, WebviewWindow,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub drinks_today: u32,
    pub reminders_today: u32,
    pub streak_days: u32,
    pub last_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub reminder_interval_minutes: u32,
    pub after_drink_minutes: u32,
    pub snooze_minutes: u32,
    pub scale: f64,
    pub enable_sound: bool,
    pub volume: f64,
    pub pause_when_idle: bool,
    pub start_with_windows: bool,
    pub is_dark_mode: bool,
    pub first_launch: bool,
    pub stats: Stats,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            reminder_interval_minutes: 25,
            after_drink_minutes: 20,
            snooze_minutes: 5,
            scale: 1.0,
            enable_sound: true,
            volume: 0.6,
            pause_when_idle: true,
            start_with_windows: false,
            is_dark_mode: false,
            first_launch: true,
            stats: Stats {
                drinks_today: 0,
                reminders_today: 0,
                streak_days: 1,
                last_date: "Today".to_string(),
            },
        }
    }
}

fn get_app_dir_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "spiderman", "spider-man-hydration-reminder") {
        let dir = proj_dirs.data_dir().to_path_buf();
        let _ = fs::create_dir_all(&dir);
        dir
    } else {
        PathBuf::from(".")
    }
}

fn get_settings_file_path() -> PathBuf {
    get_app_dir_path().join("settings.json")
}

fn get_sounds_dir_path() -> PathBuf {
    let dir = get_app_dir_path().join("sounds");
    let _ = fs::create_dir_all(&dir);
    dir
}

// -----------------------------------------------------------------------------
// Windows Single-Instance Lock
// -----------------------------------------------------------------------------

#[cfg(target_os = "windows")]
fn ensure_single_instance() -> bool {
    use windows::core::w;
    use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS};
    use windows::Win32::System::Threading::CreateMutexW;
    unsafe {
        let _ = CreateMutexW(None, true, w!("Global\\SpiderManHydrationSingleInstanceMutex"));
        GetLastError() != ERROR_ALREADY_EXISTS
    }
}

#[cfg(not(target_os = "windows"))]
fn ensure_single_instance() -> bool {
    true
}

// -----------------------------------------------------------------------------
// Windows Auto-Start via Registry (HKCU\Software\Microsoft\Windows\CurrentVersion\Run)
// -----------------------------------------------------------------------------

#[cfg(target_os = "windows")]
fn is_autostart_enabled() -> bool {
    use windows::core::w;
    use windows::Win32::System::Registry::{RegOpenKeyExW, RegQueryValueExW, HKEY_CURRENT_USER, KEY_READ};
    unsafe {
        let mut key = Default::default();
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        if RegOpenKeyExW(HKEY_CURRENT_USER, subkey, 0, KEY_READ, &mut key).is_ok() {
            let app_name = w!("SpiderManHydration");
            let mut buf = [0u16; 512];
            let mut buf_len = (buf.len() * 2) as u32;
            let status = RegQueryValueExW(
                key,
                app_name,
                None,
                None,
                Some(buf.as_mut_ptr() as *mut _),
                Some(&mut buf_len),
            );
            status.is_ok()
        } else {
            false
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn is_autostart_enabled() -> bool {
    false
}

#[cfg(target_os = "windows")]
fn set_autostart_enabled(enabled: bool) -> bool {
    use windows::core::w;
    use windows::Win32::System::Registry::{
        RegDeleteValueW, RegOpenKeyExW, RegSetValueExW, HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ,
    };
    unsafe {
        let mut key = Default::default();
        let subkey = w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        if RegOpenKeyExW(HKEY_CURRENT_USER, subkey, 0, KEY_SET_VALUE, &mut key).is_ok() {
            let app_name = w!("SpiderManHydration");
            if enabled {
                if let Ok(exe_path) = std::env::current_exe() {
                    let exe_str = format!("\"{}\" --hidden\0", exe_path.display());
                    let wide: Vec<u16> = exe_str.encode_utf16().collect();
                    let wide_bytes: &[u8] = std::slice::from_raw_parts(wide.as_ptr() as *const u8, wide.len() * 2);
                    let _ = RegSetValueExW(
                        key,
                        app_name,
                        0,
                        REG_SZ,
                        Some(wide_bytes),
                    );
                }
            } else {
                let _ = RegDeleteValueW(key, app_name);
            }
            true
        } else {
            false
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn set_autostart_enabled(_enabled: bool) -> bool {
    false
}

// -----------------------------------------------------------------------------
// Tauri IPC Commands
// -----------------------------------------------------------------------------

#[tauri::command]
fn get_settings() -> AppSettings {
    let path = get_settings_file_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                return settings;
            }
        }
    }
    let default_settings = AppSettings::default();
    let _ = fs::write(path, serde_json::to_string_pretty(&default_settings).unwrap_or_default());
    default_settings
}

#[tauri::command]
fn save_settings(settings: AppSettings) -> AppSettings {
    let path = get_settings_file_path();
    let _ = fs::write(path, serde_json::to_string_pretty(&settings).unwrap_or_default());
    settings
}

#[tauri::command]
fn get_login_item_settings() -> bool {
    is_autostart_enabled()
}

#[tauri::command]
fn set_login_item_settings(enabled: bool) -> bool {
    set_autostart_enabled(enabled)
}

#[tauri::command]
fn save_custom_audio(sound_type: String, base64_data: String, extension: Option<String>) -> Result<String, String> {
    let ext = extension.unwrap_or_else(|| "mp3".to_string());
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let file_name = format!("{}_{}.{}", sound_type, now, ext);
    let sounds_dir = get_sounds_dir_path();
    let target_path = sounds_dir.join(file_name);

    use base64::Engine;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&base64_data)
        .map_err(|e| format!("Base64 decode error: {}", e))?;
    fs::write(&target_path, decoded).map_err(|e| format!("File write error: {}", e))?;

    Ok(format!("asset://{}", target_path.to_string_lossy().replace('\\', "/")))
}

#[tauri::command]
fn set_ignore_mouse_events(window: WebviewWindow, ignore: bool) -> Result<(), String> {
    window.set_ignore_cursor_events(ignore).map_err(|e| e.to_string())
}

#[tauri::command]
fn close_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

// -----------------------------------------------------------------------------
// Windows Idle Monitoring (Win32 GetLastInputInfo)
// -----------------------------------------------------------------------------

#[cfg(target_os = "windows")]
fn get_system_idle_seconds() -> u32 {
    use windows::Win32::System::SystemInformation::GetTickCount64;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    unsafe {
        let mut lii = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if GetLastInputInfo(&mut lii).as_bool() {
            let tick = GetTickCount64();
            let elapsed_ms = tick.saturating_sub(lii.dwTime as u64);
            (elapsed_ms / 1000) as u32
        } else {
            0
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn get_system_idle_seconds() -> u32 {
    0
}

// -----------------------------------------------------------------------------
// Tower Service Integration
// -----------------------------------------------------------------------------

pub struct HydrationReminderService;

impl tower::Service<String> for HydrationReminderService {
    type Response = String;
    type Error = std::convert::Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        log::info!("Hydration Service processed request: {}", req);
        std::future::ready(Ok(format!("Acknowledged: {}", req)))
    }
}

// -----------------------------------------------------------------------------
// Application Entry Point & Tray Setup
// -----------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Single instance mutex guard
    if !ensure_single_instance() {
        log::warn!("Another instance is already running. Exiting secondary instance.");
        return;
    }

    let is_paused = Arc::new(AtomicBool::new(false));
    let is_paused_for_tray = is_paused.clone();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_login_item_settings,
            set_login_item_settings,
            save_custom_audio,
            set_ignore_mouse_events,
            close_app
        ])
        .setup(move |app| {
            let handle = app.handle();

            // Set up System Tray
            let header = MenuItemBuilder::with_id("header", "🕷️ Spider-Man Hydration")
                .enabled(false)
                .build(handle)?;
            let drink_water = MenuItemBuilder::with_id("drink", "💧 Drink Water (+1 💧)").build(handle)?;
            let test_reminder = MenuItemBuilder::with_id("test", "🕷️ Test Reminder").build(handle)?;
            let settings = MenuItemBuilder::with_id("settings", "⚙️ Settings").build(handle)?;
            let pause_toggle = MenuItemBuilder::with_id("pause", "⏸ Pause Reminders").build(handle)?;
            let exit = MenuItemBuilder::with_id("exit", "🚪 Exit").build(handle)?;
            let sep1 = PredefinedMenuItem::separator(handle)?;
            let sep2 = PredefinedMenuItem::separator(handle)?;
            let sep3 = PredefinedMenuItem::separator(handle)?;

            let menu = MenuBuilder::new(handle)
                .item(&header)
                .item(&sep1)
                .item(&drink_water)
                .item(&test_reminder)
                .item(&settings)
                .item(&sep2)
                .item(&pause_toggle)
                .item(&sep3)
                .item(&exit)
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Spider-Man Hydration Reminder 🕷️💧")
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "drink" => {
                        let _ = app.emit("action-drink", ());
                    }
                    "test" => {
                        let _ = app.emit("trigger-reminder", ());
                    }
                    "settings" => {
                        let _ = app.emit("open-settings", ());
                    }
                    "pause" => {
                        let current = is_paused_for_tray.load(Ordering::Relaxed);
                        let next = !current;
                        is_paused_for_tray.store(next, Ordering::Relaxed);
                        let _ = app.emit("toggle-pause", next);
                    }
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let _ = app.emit("open-settings", ());
                    }
                })
                .build(handle)?;

            // Background Windows Idle Monitoring & Sleep/Wake Detection Loop
            let app_handle_for_idle = handle.clone();
            std::thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let now = std::time::Instant::now();
                    let elapsed = now.duration_since(last_tick);
                    last_tick = now;

                    // If more than 8 seconds elapsed across a 2-second sleep, system resumed from sleep
                    if elapsed.as_secs() > 8 {
                        log::info!("System woke from sleep/suspend (elapsed: {}s)", elapsed.as_secs());
                        let _ = app_handle_for_idle.emit("power-resume", ());
                    }

                    let idle_sec = get_system_idle_seconds();
                    let _ = app_handle_for_idle.emit("idle-status", idle_sec);
                }
            });

            // CLI Flag Support: --test / --demo / --settings
            let cli_args: Vec<String> = std::env::args().collect();
            let handle_for_cli = handle.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1200));
                if cli_args.iter().any(|a| a == "--test" || a == "--demo") {
                    log::info!("CLI flag detected: triggering Spider-Man reminder animation");
                    let _ = handle_for_cli.emit("trigger-reminder", ());
                } else if cli_args.iter().any(|a| a == "--settings") {
                    log::info!("CLI flag detected: opening settings modal");
                    let _ = handle_for_cli.emit("open-settings", ());
                }
            });

            log::info!("Spider-Man Hydration Tauri Core successfully initialized");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
