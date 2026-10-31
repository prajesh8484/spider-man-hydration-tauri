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
                last_date: chrono_fallback_today(),
            },
        }
    }
}

fn chrono_fallback_today() -> String {
    "Today".to_string()
}

fn get_settings_file_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("com", "spiderman", "spider-man-hydration-reminder") {
        let dir = proj_dirs.data_dir();
        let _ = fs::create_dir_all(dir);
        dir.join("settings.json")
    } else {
        PathBuf::from("settings.json")
    }
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
    // Windows Run registry key or placeholder for desktop toggle
    false
}

#[tauri::command]
fn set_login_item_settings(enabled: bool) -> bool {
    log::info!("Auto-start with Windows toggled: {}", enabled);
    enabled
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

            // Background Windows Idle Monitoring Loop (Native Thread for Zero Overhead)
            let app_handle_for_idle = handle.clone();
            std::thread::spawn(move || {
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    let idle_sec = get_system_idle_seconds();
                    let _ = app_handle_for_idle.emit("idle-status", idle_sec);
                }
            });

            log::info!("Spider-Man Hydration Tauri Core successfully initialized");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
