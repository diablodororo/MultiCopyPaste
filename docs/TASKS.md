# MultiCopyPaste 開發任務待辦與里程碑清單 (Tasks & Roadmap)

本文件持續記錄 MultiCopyPaste 的開發歷程、版本迭代進度與未來規劃的任務待辦清單，作為產品演進與開發追蹤的核心參考文件。

---

## 1. 已完成里程碑 (Completed Phases)

### Phase 1：專案初始化與核心資料結構 (`v0.0.1`)
- [x] 初始化 Git 儲存庫與 `docs/PRODUCT_SPEC.md` 產品規格文件
- [x] 使用 Tauri v2 CLI 建立跨平台桌面應用程式框架 (`macOS` / `Windows`)
- [x] 建立 Rust 核心資料結構 (`ClipboardItem`, `SequenceQueue`, `AppConfig`) 與狀態管理器 (`state.rs`)

### Phase 2：系統剪貼簿監聽與全域快捷鍵注入 (`v0.0.2`)
- [x] 整合 `arboard` 系統剪貼簿事件輪詢與監聽機制
- [x] 實作剪貼歷史自動去重與 $N$ 筆序列動態維護
- [x] 實作 macOS (`CoreGraphics CGEvent`) 與 Windows (`enigo VK_V`) 全域快捷鍵監聽 (`Control+Alt+V` / `Ctrl+Alt+V`) 與鍵盤模擬貼上 (`Cmd+V` / `Ctrl+V`)

### Phase 3：現代化前端 UI 與 IPC 資料雙向綁定 (`v0.0.3`)
- [x] 打造暗黑擬態 (`Glassmorphism`) 現代化設定與隊列卡片面板
- [x] 實作序列長度 $N$ 調整滑桿與 `current_index` 當前釋出卡片即時發光指示
- [x] 實作一鍵重置循環隊列指標按鈕與狀態回報機制

### Phase 4：頂部選單列常駐與背景監聽支援 (`v0.0.4` - `v0.0.5`)
- [x] 設計並實作 macOS 頂部選單列 (System Tray / Menu Bar) 極簡幾何線條圖示 (`icon_as_template`)
- [x] 實作主視窗點擊關閉按鈕自動轉為背景隱藏常駐服務，點擊選單列圖示可隨時召喚聚焦
- [x] 新增繁體中文 (`zh.ts`) 與英文 (`en.ts`) 雙語切換與 `localStorage` 記憶保存
- [x] 改良 Tauri 打包機制 (`tauri.conf.json` 改為 `app` 目標)，防止打包 `.dmg` 時系統不斷彈出 Finder 視窗干擾

### Phase 5：卡片拖曳順序調整功能 (`v0.0.6`)
- [x] 處理 macOS `WKWebView` 中 HTML5 `draggable` 與 React State 重新渲染衝突造成的拖拽狀態機中斷問題
- [x] 全面重構為 **Pointer Events (`onPointerDown` / `pointermove` / `pointerup`)** 座標追蹤排序架構
- [x] 實作 `floating-ghost` 游標即時跟隨浮動卡片預覽層（零延遲緊貼、深層陰影與毛玻璃質感）與目標卡片高亮邊框提示
- [x] 執行單元與建置檢查 (`npm run build` + `cargo check`) 並提交至 GitHub (`c988f91`)

### Phase 6：使用者體驗優化與卡片微調操作 (`v0.0.7`)
- [x] **卡片懸停快捷工具列 (Item Action Toolbar)**：在卡片右側或懸停時，顯示微型「編輯 (`Edit`)」與「刪除 (`Trash`)」按鈕，方便即時微調單筆文字內容或剔除不需要的歷史紀錄。
- [x] **雙螢幕與 DPI 解析度適應驗證**：透過 `setPointerCapture` / `releasePointerCapture` 與 `elementFromPoint` 強化 Pointer Events 在不同 DPI 或跨多螢幕時的座標捕捉與 `floating-ghost` 渲染穩定度。
- [x] **手動單擊卡片跳轉指標 (Jump to Item)**：點擊任一卡片的序號標籤 (`index-pill`) 即可觸發 `set_sequence_index`，將下一個要貼上的目標 (`current_index`) 直接切換至該卡片。
- [x] **剪貼資料保真**：擷取、貼回與編輯流程完整保留前後空白、縮排及換行，並以多行編輯器支援程式碼與結構化文字。
- [x] **核心狀態測試與重排防護**：為擷取、去重、循環、排序、刪除、編輯及 history 上限建立 Rust 單元測試；重排只接受既有項目的完整排列，避免錯誤 IPC payload 污染內容。

---

## 2. 進行中與近期規劃待辦 (Active & Upcoming Tasks)

### Phase 7：跨平台發布與 GitHub Actions CI/CD (`v1.0.0`)
- [ ] **Release 打包發布**：建立 GitHub Release 標籤 (`v0.0.6` / `v1.0.0`)，並上傳封裝好的 macOS `.app` / `.dmg` 安裝包。
- [ ] **Windows 環境相容性最後測試**：驗證 Windows 平台下的 `enigo` 貼上模擬以及托盤常駐行為是否在最新 Win11 / Win10 環境完美運作。
- [ ] **設定匯出/匯入 (`Backup & Restore`)**：提供使用者把當前常用序列或自訂快捷鍵設定導出為 JSON 備份檔的功能。
