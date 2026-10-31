# 🕷️ User Stories & Functional Blueprint: Spider-Man Hydration Reminder

This directory defines the functional requirements, user journeys, and acceptance criteria for the Spider-Man Hydration Reminder desktop companion. Both the **Electron** reference app and the **Rust (Tauri / Tower)** implementation adhere to these specifications.

---

## Story Index

| Story ID | Title | Priority | Target Implementations |
| :--- | :--- | :--- | :--- |
| **US-001** | Fullscreen Transparent Desktop Overlay | Critical | Electron & Rust / Tauri |
| **US-002** | Dynamic Mouse Event Click-Through | Critical | Electron & Rust / Tauri |
| **US-003** | 8 Dynamic Choreographed Spider-Man Entrances | High | Electron & Rust / Tauri |
| **US-004** | Non-Repeating Asset Queue Shuffling | High | Electron & Rust / Tauri |
| **US-005** | Windows System Tray Companion & Context Menu | High | Electron & Rust / Tauri |
| **US-006** | Procedural Web Audio & Custom Sound Uploads | Medium | Electron & Rust / Tauri |
| **US-007** | System Idle & Computer Sleep Compensation | High | Electron & Rust / Tauri |
| **US-008** | Hydration Tracking, Confetti & Streak Analytics | Medium | Electron & Rust / Tauri |
| **US-009** | Windows Auto-Start on Boot | Medium | Electron & Rust / Tauri |

---

## Detailed Specifications

### US-001: Fullscreen Transparent Desktop Overlay
- **As a** busy desktop user,
- **I want** Spider-Man to appear smoothly directly over my active screen without an application window frame or background color,
- **So that** the reminder feels like an organic companion residing in my desktop environment.
- **Acceptance Criteria**:
  1. Window matches the primary display's work area resolution (`screen.getPrimaryDisplay().workAreaSize`).
  2. Background is 100% alpha transparent (`rgba(0,0,0,0)`).
  3. Window stays always-on-top above normal windows (`'screen-saver'` z-level priority).
  4. App icon does not clutter the Windows Taskbar (`skipTaskbar: true`), operating quietly via the System Tray.

---

### US-002: Dynamic Mouse Event Click-Through (Hit-Testing)
- **As a** user working in other applications (coding, typing, gaming),
- **I want** mouse clicks to pass straight through the transparent overlay when Spider-Man is not prompting me,
- **So that** the application never interferes with my normal workflow.
- **Acceptance Criteria**:
  1. In `IDLE_COUNTING` state, overlay window passes all mouse clicks through to underlying applications.
  2. When Spider-Man arrives (`ENTRANCE`, `REMINDER_VISIBLE`), or when Settings Modal is open, the overlay captures mouse clicks so the user can click **"Drink Water"**, **"Snooze"**, or close the modal.
  3. As soon as Spider-Man exits (`EXIT`), click-through passthrough is immediately re-engaged.

---

### US-003: 8 Dynamic Choreographed Spider-Man Entrances
- **As a** Marvel / Spider-Man fan,
- **I want** variety in how Spider-Man enters my screen,
- **So that** reminders remain delightful and never feel repetitive.
- **Supported Poses & Trajectories**:
  1. **Top Upside-Down Drop**: Hangs upside down on an elastic SVG web line from top screen edge.
  2. **Left Side Wall Crawl**: Clings to and crawls horizontally from the left monitor border.
  3. **Right Side Swing**: Pendulum web swing entering from the right display border.
  4. **Bottom Climb Up**: Crouches and climbs up from behind the Windows taskbar.
  5. **Top-Left Corner Swing**: Diagonal web glide across screen from upper left.
  6. **Side Peek**: Playful peek around the side bezel.
  7. **Ceiling Crawl**: Crawls along the upper screen edge.
  8. **High-Speed Web Zip-In**: Instant snap zip with spring bounce physics.

---

### US-004: Non-Repeating Asset Queue Shuffling
- **As a** user,
- **I want** the reminder poses and quotes to cycle intelligently,
- **So that** I don't see the exact same entrance animation two times in a row.
- **Acceptance Criteria**:
  1. All 8 poses are shuffled into a randomized queue.
  2. A pose is only reused after all 8 have been displayed.
  3. Reshuffling ensures the last pose of the previous queue is not identical to the first pose of the next queue.

---

### US-005: Windows System Tray Companion
- **As a** user,
- **I want** quick access to hydration controls from my Windows system tray notification area,
- **So that** I can log water, test reminders, or adjust settings without opening a bulky window.
- **Context Menu Structure**:
  - `🕷️ Spider-Man Hydration` (Disabled title header)
  - `────────────────────`
  - `💧 Drink Water (+1 💧)` (Logs water, plays celebratory synth, triggers confetti)
  - `🕷️ Test Reminder` (Instantly triggers next Spider-Man entrance)
  - `⚙️ Settings` (Opens configuration modal)
  - `────────────────────`
  - `⏸ Pause Reminders` / `▶ Resume Reminders` (Toggles pause state)
  - `────────────────────`
  - `🚪 Exit` (Graceful shutdown)

---

### US-006: Procedural Web Audio & Custom Sounds
- **As a** user who loves audio feedback,
- **I want** clean, stylized sound effects accompanying Spider-Man's movements,
- **So that** I receive an auditory cue even if I am not looking directly at the screen.
- **Audio Palette**:
  - **Web Thwip**: White-noise frequency sweep mimicking pressurized web-shooter release.
  - **Reminder Chime**: Soft major-pentatonic chord chime.
  - **Hydration Celebration**: Joyful ascending multi-tone arpeggio.
  - **Snooze Click**: Soft descending low-tone acknowledgement.
  - **Custom Sounds**: Option to upload personal `.mp3` or `.wav` sound files stored in `%APPDATA%`.

---

### US-007: System Idle & Computer Sleep Compensation
- **As a** user who leaves my computer to attend meetings or take breaks,
- **I want** the reminder timer to pause when I am idle (> 60s) or when my computer goes to sleep,
- **So that** I am not overwhelmed with reminders when I return.
- **Acceptance Criteria**:
  1. Queries Windows idle time every 2 seconds (`GetLastInputInfo` / `powerMonitor.getSystemIdleTime`).
  2. If idle time exceeds 60 seconds, countdown timer pauses.
  3. On OS suspend/sleep, active countdown is frozen and zero timers fire during sleep.
  4. On OS resume/wake, countdown smoothly continues without delivering stale backlog alerts.

---

### US-008: Hydration Tracking, Confetti & Streak Analytics
- **As a** health-conscious individual,
- **I want** daily tracking of how many glasses of water I've drank and my consecutive day streak,
- **So that** I stay motivated to build a healthy hydration habit.
- **Acceptance Criteria**:
  1. Clicking "I Drank Water!" triggers celebratory confetti physics (`canvas-confetti`).
  2. Daily glass counter increments by 1.
  3. Day streak increments if user drank water on consecutive days; resets if a day is skipped.
  4. Statistics persist across application restarts.
