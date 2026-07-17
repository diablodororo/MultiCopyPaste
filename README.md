# MultiCopyPaste 🚀
**跨平台序列循環剪貼簿小工具 (macOS / Windows)**
*[English Version](README.en.md)*

---

## 💡 原因與痛點 (Why MultiCopyPaste)

在日常文書處理、試算表搬移或填寫線上表單時，我們經常需要重複執行：
`複製 A -> 切換視窗 -> 貼上 A -> 複製 B -> 切換視窗 -> 貼上 B -> 複製 C -> 切換視窗 -> 貼上 C...`

**MultiCopyPaste** 讓您可以在來源視窗**一口氣依序連續複製 N 筆資料 (A, B, C)**，接著在目標視窗連續按下**全域貼上快捷鍵**，工具就會自動以 `A -> B -> C -> A -> B -> C` 的順序依序釋出並循環貼上，為您大幅節省往返視窗切換的時間與精力！

---

## 🎯 用途與使用情境 (Scenarios & Use Cases)

- 📊 **Excel / 試算表資料整理**：一次複製姓名、電話、地址、Email 等多個欄位，再切換到 CRM 系統或報表一次依序填完。
- 📝 **程式碼與設定檔搬移**：在閱讀 API 文件或舊專案時，連續複製多段關鍵變數、網址與 Token，再到新檔案中依序貼上組裝。
- 📋 **批次線上表單填寫**：填寫多步驟問卷或政府線上申請單，從原始文件一次複製多段資訊後流暢連續貼上。

---

## ✨ 產品特色與亮點 (Features)

- ⚡ **極致輕量與低耗能**：基於 Tauri v2 + Rust 打造，背景常駐記憶體僅約 18~25 MB。
- 🌐 **雙語介面 (i18n)**：右上角一鍵即時切換繁體中文與 English 介面。
- 🎹 **全域快捷鍵**：按下 `Ctrl + Option + V` (macOS) 或 `Ctrl + Alt + V` (Windows) 即可在任何應用程式中瞬間依序貼上。
- 🔄 **重複貼上循環與自動清空機制**：支援設定循環輪次（預設 1 輪）。例如連續複製 `ABC`，設定 1 輪時貼完一輪 (`ABC`) 會自動清空剪貼簿記憶；設定 3 輪則可連續貼出 `ABCABCABC` 後自動清空，亦可設定無限循環。
- 🛡️ **跨平台原生穩定機制**：
  - **macOS**：採用 Apple 原生 CoreGraphics `CGEvent` 與無鎖佇列，確保高頻操作 100% 執行緒安全。
  - **Windows**：發送真實虛擬鍵碼 `VK_V` (`Key::V`) 搭配 `Control`，支援 Win32、Office、Electron 與 Terminal 等全系列程式。
- 🌟 **選單列原生 Popover (Quick Settings Popover)**：點擊 macOS 選單列 / Windows 托盤圖示（左右鍵皆可），原生 Popover 立即浮出——不搶走當前應用程式的焦點，點擊面板外自動收起。以「拉桿」即時調整序列循環長度與重複輪次，直接顯示目前複製佇列（含 `NEXT` 指標，點擊任一筆即跳轉為下一個貼上目標），並提供「📌 釘選」按鈕讓面板常駐最上層；「開啟主視窗」與「離開」也在面板中。
- 🎨 **現代高質感 UI**：絕美暗黑玻璃擬態主視窗，即時預覽序列狀態與 `NEXT` 指標。

---

## ⌨️ 用法與快捷鍵 (How to Use)

1. **開啟應用**：啟動 `MultiCopyPaste` 後，應用會於背景自動監聽您的系統剪貼簿。macOS 首次啟動會要求「輔助使用 (Accessibility)」權限——依序貼上功能必須授權才能運作。
2. **連續複製**：在任何來源視窗，依序按標準快捷鍵 (`Cmd + C` / `Ctrl + C`) 複製您要的資料（例如依序複製 `John`、`123456`、`john@example.com`）。
3. **依序貼上**：切換到目標輸入框，每按一次 **`Ctrl + Option + V` (macOS)** 或 **`Ctrl + Alt + V` (Windows)**，工具就會依序貼出下一筆資料！
4. **循環與清空**：達到預設或設定的「重複循環次數」時，佇列會自動清空，準備接收下一次的全新複製任務。

| 操作 | 預設快捷鍵 / 操作方式 |
| :--- | :--- |
| **依序循環貼上** | `Ctrl + Option + V` (macOS) / `Ctrl + Alt + V` (Windows) |
| **快速設定 Popover（拉桿 + 佇列預覽）** | 點擊選單列 / 托盤圖示（左右鍵皆可），再點一次或點面板外即收起 |
| **釘選面板（常駐最上層）** | Popover 標題列的 📌 按鈕，啟用後點擊外部不會關閉 |
| **跳轉下一個貼上目標** | Popover 或主視窗中點擊任一佇列項目 |
| **重置循環指標** | 可於介面上點擊「重置回頂部」按鈕 |
| **開啟主視窗 / 離開** | Popover 底部的按鈕 |

---

## 📥 下載與安裝 (Installation)

前往 [GitHub Releases](https://github.com/diablodororo/MultiCopyPaste/releases/latest) 下載對應平台的安裝包（macOS `.dmg` / Windows `.exe`）。

### macOS：出現「App 已損壞，無法打開」怎麼辦？

本專案未加入 Apple 開發者計畫（未經 notarization 公證），macOS Gatekeeper 會攔截從網路下載的安裝包並顯示「已損壞」。這**不是檔案真的損壞**，執行以下指令移除隔離標記即可正常使用：

```bash
xattr -cr /Applications/MultiCopyPaste.app
```

（將 `.dmg` 內的 App 拖入「應用程式」資料夾後，打開「終端機」貼上執行，再重新啟動 App。）

### macOS：授權「輔助使用」權限

首次啟動會跳出系統提示，請前往「系統設定 → 隱私權與安全性 → 輔助使用」開啟 `MultiCopyPaste`——模擬 `Cmd+V` 按鍵注入需要此權限，未授權時快捷鍵貼上不會有任何效果。升級新版本後若貼上失效，請將清單中的舊項目移除後重新加入。

### Windows：SmartScreen 提示

首次執行若出現「Windows 已保護您的電腦」，點擊「其他資訊 → 仍要執行」即可。

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

### 打包正式版應用程式 (.app / .dmg / .exe / .msi)
```bash
npm run build && npx tauri build
```

### macOS 開發期安裝（保留輔助使用授權）
macOS 的「輔助使用」授權綁定 App 簽章，ad-hoc 簽章每次重建都會改變並讓授權失效。使用固定的本機開發憑證簽章即可讓授權跨重建持續有效：

```bash
APPLE_SIGNING_IDENTITY="<你的本機憑證名稱>" ./scripts/macos-dev-install.sh
```

腳本會自動建置、以固定憑證簽章、部署至 `/Applications` 並重新啟動。

---

## 📄 授權條款
MIT License
