# MultiCopyPaste 🚀
**Cross-Platform Sequential Multi-Copy Paste Utility (macOS / Windows)**
*[繁體中文版](README.md)*

---

## 💡 Why MultiCopyPaste (Problem & Solution)

In daily data entry, spreadsheet transfers, or form filling, we often repeatedly switch windows:
`Copy A -> Switch Window -> Paste A -> Copy B -> Switch Window -> Paste B -> Copy C -> Switch Window -> Paste C...`

**MultiCopyPaste** allows you to **copy N items consecutively (A, B, C)** in the source window without switching back and forth. Then, simply switch to your target input field and press the **Global Paste Shortcut** repeatedly to sequentially release and cycle through `A -> B -> C -> A -> B -> C` automatically!

---

## 🎯 Scenarios & Use Cases

- 📊 **Spreadsheet & Data Entry**：Copy multiple columns (Name, Phone, Email, Address) in one go from Excel/CSV, then switch once to your CRM or web form to paste them sequentially.
- 📝 **Code & Config Transfers**：Copy multiple API keys, URLs, and variable snippets from docs or existing files, and paste them sequentially into your target source code.
- 📋 **Multi-Step Form Filling**：Gather multiple fields from a reference PDF or email and paste them smoothly one by one into multi-page web forms.

---

## ✨ Features & Highlights

- ⚡ **Ultra Lightweight & Efficient**：Built on Tauri v2 + Rust with a memory footprint of only ~18–25 MB.
- 🌐 **Internationalization (i18n)**：Instant switching between English & Traditional Chinese via top-right dropdown.
- 🎹 **Global Shortcut**：Press `Ctrl + Option + V` (macOS) or `Ctrl + Alt + V` (Windows) to paste sequentially across any active application.
- 🛡️ **Native OS Stability**：
  - **macOS**：Uses Apple native CoreGraphics `CGEvent` and lock-free queues for 100% thread safety without crashes.
  - **Windows**：Injects standard virtual key `VK_V` (`Key::V`) with `Control` for 100% compatibility across Win32, Office, WPF, Electron, and terminals.
- 🌟 **System Tray Resident**：Minimalist template icon on macOS adapts cleanly to light/dark menu bars; rests quietly in the Windows system tray with a quick left-click menu to show/hide.
- 🎨 **Sleek Modern UI**：Dark theme with glassmorphism styling, live cycle previews, and a glowing `NEXT` indicator badge.

---

## ⌨️ How to Use & Shortcuts

1. **Launch App**：Start `MultiCopyPaste`. It will run quietly in the background monitoring your clipboard.
2. **Copy Consecutively**：In any source window, use standard `Cmd + C` / `Ctrl + C` to copy items one after another (e.g., copy `John`, then `123456`, then `john@example.com`).
3. **Paste Sequentially**：Switch to your target input box and press **`Ctrl + Option + V` (macOS)** or **`Ctrl + Alt + V` (Windows)** repeatedly to paste the items in sequence!

| Action | Shortcut |
| :--- | :--- |
| **Paste Sequentially** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **Reset Index** | Click "Reset to Top" button in the UI |
| **Show/Hide Window** | Left-click top menu bar / system tray icon -> Select "Show Window" |

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

### Build Distribution Installer (.app / .dmg / .exe / .msi)
```bash
npm run build && npx tauri build
```

---

## 📄 License
MIT License
