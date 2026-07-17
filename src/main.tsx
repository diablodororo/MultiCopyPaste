import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import './index.css'
import App from './App.tsx'
import QuickPanel from './QuickPanel.tsx'

const isQuickPanel = getCurrentWebviewWindow().label === 'quickset'
if (isQuickPanel) {
  document.documentElement.classList.add('quickset-mode')
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    {isQuickPanel ? <QuickPanel /> : <App />}
  </StrictMode>,
)
