# MultiCopyPaste 🚀
**跨平台序列循環剪貼簿小工具 (macOS / Windows)**
*[English Version](README.en.md)*

---

## 💡 痛點與為此而生的解決方案

在日常文書處理、試算表搬移或填寫線上表單時，我們經常需要重複：
`複製A -> 貼上A -> 複製B -> 貼上B -> 複製C -> 貼上C...`

**MultiCopyPaste** 讓您可以在來源視窗**一口氣依序複製 N 筆資料 (A, B, C)**，接著在目標視窗連續按下 **全域貼上快捷鍵**，工具就會依序將 `A -> B -> C -> A -> B -> C` 自動循環貼上！

---

## ✨ 產品特色

- ⚡ **極致輕量與低耗能**：基於 Tauri v2 + Rust 打造，背景常駐記憶體僅約 18~25 MB。
- 🌐 **雙語介面 (i18n)**：支援繁體中文與 English 即時切換。
- 🎹 **全域快捷鍵**：按下 `Ctrl + Option + V` (macOS) 或 `Ctrl + Alt + V` (Windows) 即可瞬間依序貼上。
- 🛡️ **穩定防當機制**：macOS 採用 Apple 原生 CoreGraphics `CGEvent`，避免跨執行緒崩潰。
- 🎨 **現代高質感 UI**：絕美暗黑玻璃擬態介面，及時預覽當前序列與 `NEXT` 貼上指標。

---

## ⌨️ 快捷鍵與操作說明

| 操作 | 預設快捷鍵 |
| :--- | :--- |
| **依序循環貼上** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **重置循環指標** | 可於介面上點擊「重置回頂部」按鈕 |

---

## 🛠️ 開發與建置

### 前置需求
- Node.js (>= 18)
- Rust (>= 1.77)

### 啟動開發伺服器
```bash
npm install
npx tauri dev
```

### 打包正式版應用程式 (Build App / DMG)
```bash
npx tauri build
```
打包完成後產物將放置於 `src-tauri/target/release/bundle/dmg/` 目錄中。

---

## 📄 授權條款
MIT License
