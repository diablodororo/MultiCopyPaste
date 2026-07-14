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
- 🌐 **Internationalization (i18n)**: Instant switching between English & Traditional Chinese.
- 🎹 **Global Shortcut**: Press `Ctrl + Option + V` (macOS) or `Ctrl + Alt + V` (Windows) to paste sequentially anywhere.
- 🛡️ **Native macOS Stability**: Uses Apple native CoreGraphics `CGEvent` to ensure 100% thread safety and zero crashes.
- 🎨 **Sleek Modern UI**: Rich dark theme with glassmorphism styling, live previews, and a glowing `NEXT` indicator.

---

## ⌨️ Shortcuts & Usage

| Action | Shortcut |
| :--- | :--- |
| **Paste Sequentially** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **Reset Index** | Click "Reset to Top" button in the UI |

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
npx tauri build
```
The output `.dmg` installer will be located in `src-tauri/target/release/bundle/dmg/`.

---

## 📄 License
MIT License
