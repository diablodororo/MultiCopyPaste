# MultiCopyPaste 🚀
**Cross-Platform Sequential Multi-Copy Paste Utility (macOS / Windows)**
*[繁體中文版](README.md)*

---

## 💡 Problem & Solution

In daily data entry, spreadsheet transfers, or form filing, we often repeatedly switch windows:
`Copy A -> Paste A -> Copy B -> Paste B -> Copy C -> Paste C...`

**MultiCopyPaste** allows you to **copy N items consecutively (A, B, C)** in the source window, and then repeatedly press the **Global Paste Shortcut** in the target window to sequentially cycle paste `A -> B -> C -> A -> B -> C` automatically!

---

## ✨ Features

- ⚡ **Ultra Lightweight & Efficient**: Built on Tauri v2 + Rust with a memory footprint of only ~18–25 MB.
- 🌐 **Internationalization (i18n)**: Instant switching between English & Traditional Chinese with a top-right dropdown.
- 🎹 **Global Shortcut**: Press `Ctrl + Option + V` (macOS) or `Ctrl + Alt + V` (Windows) to paste sequentially anywhere.
- 🛡️ **Native macOS Stability**: Uses Apple native CoreGraphics `CGEvent` and lock-free queues to ensure 100% thread safety and zero crashes during rapid pasting.
- 🌟 **System Tray Resident & Quick Menu**: Features a sleek minimalist transparent geometric line icon for the macOS menu bar (adapts automatically to dark/light menu bars via template mode), allowing left-click window summoning or quitting.
- 🎨 **Sleek Modern UI & Brand Identity**: Rich dark theme with glassmorphism styling, live cycle previews, and custom magic-wand application icons.

---

## ⌨️ Shortcuts & Usage

| Action | Shortcut |
| :--- | :--- |
| **Paste Sequentially** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **Reset Index** | Click "Reset to Top" button in the UI |
| **Show/Hide Window** | Left-click top menu bar icon -> Select "Show Window / 顯示視窗" |

---

## 📦 Changelog

### v0.0.5 (Current Release)
- 🎨 **Brand Identity & Icon Upgrade**: Updated main application and DMG installer icons with custom magic-wand and letter typography artwork.
- 🌟 **Minimalist System Tray Icon**: Added dedicated pure white transparent line art icon with `icon_as_template(true)` for seamless adaptation between light and dark macOS menu bars.
- 🚀 **Silent Background Packaging**: Refactored `.dmg` creation to build purely in the background without opening or interfering with macOS Finder GUI windows.

### v0.0.4
- 🌐 **Bilingual UI (i18n)**: Added instant English and Traditional Chinese switching menu and clear version bar.
- 🖥️ **System Tray Integration**: Window close events now hide the app cleanly to the top menu bar with a left-click quick menu.

### v0.0.3
- 🎹 **Global Shortcut Update**: Changed default sequential paste shortcut to `Ctrl + Option + V` on macOS.
- 🛡️ **Fixed Paste Crash Issue**: Refactored CoreGraphics `CGEvent` and clipboard monitoring threads to resolve SIGTRAP crashes and memory access issues.

---

## 🛠️ Development & Build

### Prerequisites
- Node.js (>= 18)
- Rust (>= 1.77)

### Run in Development
```bash
npm install
npx tauri dev
```

### Build Distribution Installer (.dmg)
```bash
npm run build && npx tauri build
```
The output `.dmg` installer will be located in `src-tauri/target/release/bundle/dmg/`.

---

## 📄 License
MIT License
