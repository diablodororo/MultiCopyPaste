# MultiCopyPaste 序列循環剪貼簿小工具 (產品規格書)

## 產品目標
提供 macOS 與 Windows 雙平台通用的輕量級剪貼簿小工具，支援設定循環次數 $N$。
當使用者依序複製 $A, B, C$ （共 $N=3$ 筆）後，利用專屬快捷鍵（預設 `Option + V` / `Alt + V`）即可依序將 $A \rightarrow B \rightarrow C \rightarrow A \rightarrow B \rightarrow C$ 依序循環貼上至目前聚焦的應用程式，省去往返切換多重視窗的時間。

## 技術架構 (Tauri v2)
- **核心與系統整合 (Rust)**：
  - `arboard`：跨平台背景監聽系統剪貼簿事件。
  - `enigo`：跨平台模擬鍵盤貼上事件 (`Cmd + V` / `Ctrl + V`)。
  - `tauri-plugin-global-shortcut`：全域熱鍵監聽 (`Option + V` / `Alt + V`)。
- **介面層 (React + Vite + TypeScript)**：
  - 現代化玻璃擬態暗黑主題。
  - 及時預覽目前序列內容與下一個準備貼上的項目徽章 (`NEXT`)。
  - 支援調整 $N$ 數量與隨時一鍵重置指標回到第 1 筆。
