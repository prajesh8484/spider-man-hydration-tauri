# 🕷️ Spider-Man Hydration Reminder (Desktop Companion)

A lightweight Windows desktop hydration companion. An animated, photorealistic Spider-Man appears smoothly across your screen from different directions to remind you to stay hydrated while you work, without interrupting your workflow.

---

## 📂 Repository Organization

This repository contains both the reference **Electron** implementation and the ongoing migration to a high-performance **Rust (Tauri / Tower)** architecture:

| Directory | Description | Status |
| :--- | :--- | :--- |
| **[`electron/`](file:///electron)** | Reference desktop app built with Electron + Vite + React | Production Ready |
| **[`story/`](file:///story)** | Product specifications, user stories, and acceptance criteria | Active |
| **[`.mdox/`](file:///.mdox)** | Agent context, system architecture, migration plan & deep-dive metrics | Active (Git-Ignored) |
| **`tauri/`** | Target high-performance desktop companion rewritten in Rust & Tauri | Scaffolding / In Progress |

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

## ⚡ Running the Reference App (Electron)

```bash
# Navigate to electron workspace
cd electron

# Install dependencies
npm install

# Start development overlay with hot reload
npm start

# Build production Windows binaries (NSIS installer & Portable EXE)
npm run dist:win
```

---

## 📊 Electron to Rust: Metrics & Architecture

For in-depth architectural comparisons, benchmark methodology, and the step-by-step rewrite roadmap:
- 🏛️ **System Architecture**: [`.mdox/ARCHITECTURE.md`](file:///.mdox/ARCHITECTURE.md)
- 🗺️ **Migration Roadmap**: [`.mdox/PLAN.md`](file:///.mdox/PLAN.md)
- 📊 **Deep-Dive Performance Metrics**: [`.mdox/METRICS.md`](file:///.mdox/METRICS.md)
- 🔄 **Agent Context Maintenance**: [`.mdox/CONTEXT_MAINTENANCE.md`](file:///.mdox/CONTEXT_MAINTENANCE.md)
- 📜 **User Stories**: [`story/USER_STORIES.md`](file:///story/USER_STORIES.md)
