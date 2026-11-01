# 🕷️ Spider-Man Hydration Reminder (Desktop Companion)

A lightweight Windows desktop hydration companion. An animated, photorealistic Spider-Man appears smoothly across your screen from different directions to remind you to stay hydrated while you work, without interrupting your workflow.

---

## 📂 Repository Organization

This repository contains both the reference **Electron** implementation and the ongoing migration to a high-performance **Rust (Tauri / Tower)** architecture:

| Directory | Description | Status |
| :--- | :--- | :--- |
| **[`tauri/`](file:///tauri)** | High-performance desktop companion rewritten in Rust (Tauri v2 + Tower) | Production Ready ⚡ |
| **[`electron/`](file:///electron)** | Reference desktop companion built with Electron + Vite + React | Production Ready |
| **[`story/`](file:///story)** | Product specifications, user stories, and acceptance criteria | Active |
| **[`.mdox/`](file:///.mdox)** | Agent context, system architecture, migration plan & deep-dive metrics | Active (Git-Ignored) |

---

## 🚀 Key Features

- **Photorealistic Spider-Man Visuals**: High-resolution movie and comic renders with realistic webbing texture, specular lighting, and pose-aware physics.
- **8 Unique Dynamic Entrances**:
  1. **Top Upside-Down Drop**: Drops from ceiling on an elastic web line.
  2. **Left Side Wall Crawl**: Crawls along the left monitor border.
  3. **Right Side Swing**: Pendulum web swing in from the right edge.
  4. **From the Bottom**: Climbs up over the taskbar.
  5. **Top-Left Corner Swing**: Diagonally swings across the screen.
  6. **Side Peek**: Playfully peeks from the side edge.
  7. **Ceiling Crawl**: Crawls horizontally across the top border.
  8. **High-Speed Web Zip-In**: Instant web-zip with spring bounce.
- **Dynamic Click-Through**: Transparent overlay passes mouse clicks through to underlying windows during idle state; captures clicks only when Spider-Man or modals are interactive.
- **Intelligent Hydration Cycles & System Idle Tracking**: Pauses countdown during system idle (> 60s) and computer sleep.
- **Windows System Tray Companion**: Full background operation with right-click menu (**💧 Drink Water**, **🕷️ Test Reminder**, **⚙️ Settings**, **⏸ Pause / Resume**, and **🚪 Exit**).
- **Procedural Audio Synthesis**: Synthesized Web Audio web-thwips and hydration celebration chimes.

---

## ⚡ Execution Commands

### High-Performance Rust (Tauri / Tower) Implementation
```bash
# Launch Rust companion in development mode
npm run start:tauri

# Trigger immediate Spider-Man entrance test animation (Release binary)
npm run demo:tauri

# Run automated end-to-end smoke tests (Instance, Mutex, %APPDATA% persistence)
npm run test:e2e

# Run side-by-side performance benchmarks (RAM, Binary Size, Process Count)
npm run bench
```

### Reference Electron Implementation
```bash
# Start Electron development overlay
npm run start:electron

# Build production Windows binaries in electron/
cd electron && npm run dist:win
```

---

## 📊 Empirical Benchmarks: Rust (Tauri) vs. Electron

| Metric | Electron Reference | Rust (Tauri / Tower) | Efficiency Gain |
| :--- | :--- | :--- | :--- |
| **RAM Working Set** | 619.83 MB | **46.31 MB** | **92.5% less RAM** 🚀 |
| **Private Memory** | 406.86 MB | **14.36 MB** | **96.5% less memory** ⚡ |
| **Primary Executable** | 180.11 MB | **9.53 MB** | **94.7% smaller** 📦 |
| **NSIS Installer Setup**| 83.03 MB | **4.86 MB** | **94.1% smaller download** |
| **Active OS Processes** | 5 processes | **1 process** | **80% process reduction** |
| **Build Download Size** | 120.60 MB (Chromium zip)| **0 MB** (Evergreen WebView2) | **Instant builds** |

---

## 📊 Electron to Rust: Metrics & Architecture

For in-depth architectural comparisons, benchmark methodology, and the step-by-step rewrite roadmap:
- 🏛️ **System Architecture**: [`.mdox/ARCHITECTURE.md`](file:///.mdox/ARCHITECTURE.md)
- 🗺️ **Migration Roadmap**: [`.mdox/PLAN.md`](file:///.mdox/PLAN.md)
- 📊 **Deep-Dive Performance Metrics**: [`.mdox/METRICS.md`](file:///.mdox/METRICS.md)
- 🔄 **Agent Context Maintenance**: [`.mdox/CONTEXT_MAINTENANCE.md`](file:///.mdox/CONTEXT_MAINTENANCE.md)
- 📜 **User Stories**: [`story/USER_STORIES.md`](file:///story/USER_STORIES.md)
