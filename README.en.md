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
- 🔄 **Repeat Paste Cycle & Auto-Clear**：Set how many loops to paste before automatically clearing the queue (default: 1 loop). For example, with `ABC`, setting 1 loop clears after one sequence (`ABC`); setting 3 loops allows pasting `ABCABCABC` before clearing. Infinite cycling is also supported.
- 🛡️ **Native OS Stability**：
  - **macOS**：Uses Apple native CoreGraphics `CGEvent` and lock-free queues for 100% thread safety without crashes.
  - **Windows**：Injects standard virtual key `VK_V` (`Key::V`) with `Control` for 100% compatibility across Win32, Office, WPF, Electron, and terminals.
- 🌟 **Native Menu Bar Popover**：Click the menu bar (macOS) / tray (Windows) icon — either button — and the quick-settings popover floats out instantly without stealing focus from your current app; clicking anywhere else dismisses it. Adjust sequence length and repeat cycles with sliders, browse the copied queue inline (with the `NEXT` badge; click any item to make it the next paste target), 📌 pin the panel to keep it always on top, or use the Open Main Window / Quit buttons at the bottom.
- 🎨 **Sleek Modern UI**：Dark theme with glassmorphism styling, live cycle previews, and a glowing `NEXT` indicator badge.

---

## ⌨️ How to Use & Shortcuts

1. **Launch App**：Start `MultiCopyPaste`. It will run quietly in the background monitoring your clipboard. On first launch, macOS asks for the Accessibility permission — sequential paste requires it.
2. **Copy Consecutively**：In any source window, use standard `Cmd + C` / `Ctrl + C` to copy items one after another (e.g., copy `John`, then `123456`, then `john@example.com`).
3. **Paste Sequentially**：Switch to your target input box and press **`Ctrl + Option + V` (macOS)** or **`Ctrl + Alt + V` (Windows)** repeatedly to paste the items in sequence!
4. **Cycle & Auto-Clear**：Once the configured repeat count limit is reached, the queue automatically clears, ready for your next set of copies.

| Action | Shortcut / Method |
| :--- | :--- |
| **Paste Sequentially** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **Quick Settings Popover (sliders + queue preview)** | Click the menu bar / tray icon (either button); click again or click outside to dismiss |
| **Pin the Popover (always on top)** | 📌 button in the popover header; outside clicks no longer dismiss it |
| **Jump to a Paste Target** | Click any queue item in the popover or the main window |
| **Reset Index** | Click "Reset to Top" button in the UI |
| **Open Main Window / Quit** | Buttons at the bottom of the popover |

---

## 📥 Download & Installation

Grab the installer for your platform from [GitHub Releases](https://github.com/diablodororo/MultiCopyPaste/releases/latest) (macOS `.dmg` / Windows `.exe`).

### macOS: ""MultiCopyPaste" is damaged and can't be opened"?

This project is not enrolled in the Apple Developer Program (no notarization), so Gatekeeper quarantines downloaded builds and reports them as "damaged". The file is **not** actually damaged — remove the quarantine attribute and it runs normally:

```bash
xattr -cr /Applications/MultiCopyPaste.app
```

(Drag the app from the `.dmg` into Applications first, run the command in Terminal, then launch the app again.)

### macOS: Grant the Accessibility permission

On first launch a system prompt appears; enable `MultiCopyPaste` under System Settings → Privacy & Security → Accessibility. Injecting the synthetic `Cmd+V` keystroke requires it — without the grant, the paste shortcut silently does nothing. After upgrading to a new version, if pasting stops working, remove the old entry from the list and add it back.

### Windows: SmartScreen warning

If "Windows protected your PC" appears on first run, click "More info → Run anyway".

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

### macOS Dev Install (keeps the Accessibility grant)
The macOS Accessibility grant is tied to the app's code signature; ad-hoc signatures change on every rebuild and silently revoke it. Signing with a fixed local development certificate keeps the grant valid across rebuilds:

```bash
APPLE_SIGNING_IDENTITY="<your local cert name>" ./scripts/macos-dev-install.sh
```

The script builds, signs with the stable identity, installs to `/Applications`, and relaunches.

### Build Distribution Installer (.app / .dmg / .exe / .msi)
```bash
npm run build && npx tauri build
```

---

## 📄 License
MIT License
