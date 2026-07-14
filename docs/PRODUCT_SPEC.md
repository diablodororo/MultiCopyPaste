# MultiCopyPaste 序列循環剪貼簿小工具（產品規格與架構白皮書）

## 1. 產品目標與定位
**MultiCopyPaste** 是一款專為跨視窗高頻資料搬移、線上表單填寫與試算表重組而設計的跨平台（macOS / Windows）序列循環剪貼小工具。
當使用者依序從來源視窗複製 $A, B, C$ （共 $N$ 筆資料）後，只需在目標視窗連續按下專屬全域貼上快捷鍵，工具將依序以 $A \rightarrow B \rightarrow C \rightarrow A \rightarrow B \rightarrow C$ 的順序循環釋出文字內容，省去傳統「複製一次、切換視窗、貼上一次」的往返疲勞。

---

## 2. 產品功能規格與體驗亮點 (Feature Specs)

### 2.1 全域連續貼上快捷鍵與重置
- **macOS 預設快捷鍵**：`Ctrl + Option + V` (即 `Control + Alt + V`)
- **Windows 預設快捷鍵**：`Ctrl + Alt + V`
- **循環指標重置**：介面提供「重置回頂部」按鈕，隨時將下一個貼上指標歸零至序列第一項。
- **自訂序列長度 (N)**：可於介面即時調整暫存循環筆數上限（預設 5 筆，最大支援多筆動態延伸）。

### 2.2 系統選單列常駐與快選單 (System Tray / Menu Bar Resident)
- **macOS 頂部選單列**：
  - 獨立封裝純黑底白線條極簡幾何圖示 (`src-tauri/icons/tray_icon.png`)。
  - 程式中啟用 `icon_as_template(true)`，自動對應 macOS 系統淺色/深色主題與 Retina 螢幕高解析度適應。
  - 點擊選單列圖示彈出左鍵快選單：支援「顯示視窗 / Show Window」與「離開應用 / Quit」。
- **Windows 工作列右下角托盤**：
  - 常駐系統托盤區，支援雙擊/右鍵選單呼叫主視窗與快速隱藏。
- **關閉視窗邏輯**：主視窗點擊關閉時，預設隱藏至系統選單列後台持續監聽與服務，不直接強制終止進程。

### 2.3 中英雙語介面與品牌視覺 (i18n & Brand Identity)
- **多語系支援**：內建繁體中文 (`zh.ts`) 與 English (`en.ts`)，右上角一鍵即時切換語系，無須重啟應用。
- **品牌圖示 (App Icon)**：使用專屬「魔杖施法讓字母 $A, B, C$ 從紙上騰空飛舞」的高質感客製化主視覺圖示。
- **現代化介面體驗**：暗黑玻璃擬態 (`Glassmorphism`) 設計，具備狀態指示燈、序列卡片高亮及動態 `NEXT` 發光徽章。

---

## 3. 系統與底層架構工程規範 (Architecture & Backend Engine)

本專案採用 **Tauri v2 + Rust** 作為核心驅動層，搭配 **React + TypeScript** 作為 UI 互動層，確保極致輕量（常駐記憶體維持在 18~25 MB 區間）。

### 3.1 跨平台鍵盤模擬與穩定防當機制
針對作業系統剪貼簿與鍵盤注入的差異，實作獨立的平台隔離工程規範：

#### **macOS 穩定性與安全注入機制 (`#[cfg(target_os = "macos")]`)**
1. **Apple CoreGraphics CGEvent 直接發送**：
   不依賴第三方可能引發競爭條件的封裝，直接呼叫原生 `CGEventSource::new(HIDSystemState)` 與 `CGEvent::new_keyboard_event` 發送虛擬鍵碼 `9`（代表標準鍵盤 `'v'`）並附加 `CGEventFlagCommand`（代表 `Cmd` 鍵）。
2. **防當隔離 (`panic::catch_unwind`)**：
   將剪貼簿監聽與按鍵注入封裝於獨立背景執行緒與例外捕捉區塊中，徹底消除因跨視窗高頻點擊導致的 `SIGTRAP` 異常與記憶體非法存取崩潰。

#### **Windows 穩定性與按鍵注入規範 (`#[cfg(target_os = "windows")]`)**
1. **Enigo 虛擬鍵碼 (`Key::V`) 實作**：
   使用 `enigo` 模擬按鍵時，明確指定發送 `Key::V`（對應 Windows Win32 API 的虛擬鍵 `VK_V` / `0x56`）與 `Key::Control` 組合，而非字元封包 `Key::Unicode('v')`。
2. **全應用程式相容**：
   確保 `VK_V` 能夠於 Windows Win32 傳統程式、Microsoft Office、Electron 應用、WPF / UWP 以及命令提示字元/終端機中 100% 穩定被識別為 `Ctrl + V` 貼上指令。

---

## 4. 自動化 CI/CD 與建置發布規範 (Build & Release Specification)

### 4.1 多目標打包設定 (`tauri.conf.json`)
- `bundle.targets` 設定為 `"all"`，確保單一配置檔即可支援編譯全平台安裝產物。

### 4.2 macOS 靜默封裝流程 (.dmg)
- 採用靜默命令列進行 DMG 映像檔建立，避免產生視窗或干擾系統操作：
  ```bash
  npm run build && npx tauri build
  rm -f src-tauri/target/release/bundle/dmg/MultiCopyPaste_0.0.5_aarch64.dmg
  mkdir -p src-tauri/target/release/bundle/dmg
  hdiutil create -volname MultiCopyPaste -srcfolder src-tauri/target/release/bundle/macos/MultiCopyPaste.app -ov -format UDZO src-tauri/target/release/bundle/dmg/MultiCopyPaste_0.0.5_aarch64.dmg
  ```

### 4.3 跨平台自動化 CI/CD (GitHub Actions)
- 專案內建 `.github/workflows/build-and-release.yml` 跨平台發布工作流。
- **觸發條件**：推送 `v*` 標籤（如 `git push origin v0.0.5`）或透過手動 `workflow_dispatch` 觸發。
- **雙平台矩陣建置 (Matrix Build)**：
  - `macos-latest`：自動構建 macOS `.app` 與 `.dmg` 安裝包。
  - `windows-latest`：自動構建 Windows `.exe` (`NSIS setup` 安裝程式) 與 `.msi` 安裝包。
- **發布聚合**：雲端建置完成後，會自動將 macOS 與 Windows 雙平台的完整安裝檔附著上傳至該版本的 GitHub Release 頁面中，供不同系統的使用者直接下載安裝。
