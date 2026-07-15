# Codex Tool Output Review Guidelines (MultiCopyPaste)

## 1. Build 與驗證標準
- **Frontend 建置檢查**：每次提交或重大修改後，需執行 `npm run build` 確認 TypeScript 與 Vite bundle 編譯無報錯。
- **Backend 核心編譯檢查**：需於 `src-tauri` 目錄執行 `cargo check` 確認 Rust 模組、IPC 指令與全域快捷鍵綁定無編譯或借用權限錯誤。
- **Backend 核心測試**：凡涉及剪貼內容、佇列、history 或 `current_index` 的變更，需於 `src-tauri` 執行 `cargo test`，確認資料保真與狀態 invariants。

## 2. 提交條件
- 所有 Git 變更需附帶完整之 Commit Message 與推進業務目標的說明。
