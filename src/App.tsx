import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  Copy,
  RotateCcw,
  Trash2,
  Keyboard,
  ClipboardList
} from 'lucide-react';

interface ClipboardItem {
  id: string;
  content: string;
  copied_at: string;
}

interface SequenceState {
  target_length: number;
  current_index: number;
  items: ClipboardItem[];
  history: ClipboardItem[];
  shortcut: string;
  is_enabled: boolean;
}

export default function App() {
  const [state, setState] = useState<SequenceState>({
    target_length: 3,
    current_index: 0,
    items: [],
    history: [],
    shortcut: 'Option+V / Alt+V',
    is_enabled: true
  });

  const fetchState = async () => {
    try {
      const res = await invoke<SequenceState>('get_sequence_state');
      setState(res);
    } catch (e) {
      console.error('Failed to fetch sequence state:', e);
    }
  };

  useEffect(() => {
    fetchState();

    const unlistenPromise = listen<SequenceState>('sequence-updated', (event) => {
      setState(event.payload);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const handleLengthChange = async (newLength: number) => {
    if (newLength < 1 || newLength > 12) return;
    try {
      const res = await invoke<SequenceState>('set_target_length', { length: newLength });
      setState(res);
    } catch (e) {
      console.error(e);
    }
  };

  const handleResetIndex = async () => {
    try {
      const res = await invoke<SequenceState>('reset_sequence_index');
      setState(res);
    } catch (e) {
      console.error(e);
    }
  };

  const handleClear = async () => {
    try {
      const res = await invoke<SequenceState>('clear_sequence');
      setState(res);
    } catch (e) {
      console.error(e);
    }
  };

  const handleManualPaste = async () => {
    try {
      await invoke('manual_paste_next');
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="app-container">
      {/* Header */}
      <header className="header">
        <div className="title-group">
          <div className="title-icon">
            <Copy color="#fff" size={20} />
          </div>
          <div className="title-text">
            <h1>MultiCopyPaste</h1>
            <span>序列循環剪貼工具 (macOS / Windows)</span>
          </div>
        </div>
        <div className="status-badge">
          <span className="status-dot"></span>
          後台監聽中
        </div>
      </header>

      {/* Settings Card */}
      <section className="config-card">
        <div className="config-row">
          <div className="config-label">
            <h3>序列循環長度 (N)</h3>
            <p>自動擷取最近連續複製的 N 筆項目進行循環</p>
          </div>
          <div className="stepper-group">
            <button
              className="stepper-btn"
              onClick={() => handleLengthChange(state.target_length - 1)}
              disabled={state.target_length <= 1}
            >
              -
            </button>
            <span className="stepper-value">{state.target_length}</span>
            <button
              className="stepper-btn"
              onClick={() => handleLengthChange(state.target_length + 1)}
              disabled={state.target_length >= 10}
            >
              +
            </button>
          </div>
        </div>

        <div className="config-row">
          <div className="config-label">
            <h3>依序貼上快捷鍵</h3>
            <p>在任意應用程式中按下即可依序貼上</p>
          </div>
          <div className="shortcut-badge">
            <Keyboard size={14} style={{ display: 'inline', marginRight: 6, verticalAlign: 'middle' }} />
            Option + V / Alt + V
          </div>
        </div>
      </section>

      {/* Queue Display Section */}
      <section className="queue-header">
        <h2>
          目前貼上佇列 ({state.items.length}/{state.target_length})
        </h2>
        <div className="action-buttons">
          <button className="btn-secondary" onClick={handleResetIndex} title="回到第 1 筆">
            <RotateCcw size={14} />
            重置回頂部
          </button>
          <button className="btn-secondary" onClick={handleClear} title="清除記錄">
            <Trash2 size={14} />
            清空
          </button>
        </div>
      </section>

      {/* Items List */}
      <section className="queue-list">
        {state.items.length === 0 ? (
          <div className="empty-state">
            <ClipboardList size={32} color="var(--text-muted)" />
            <p>佇列目前為空</p>
            <span>請在任意視窗連續複製 {state.target_length} 段文字，將會自動在此準備循環貼上！</span>
          </div>
        ) : (
          state.items.map((item, index) => {
            const isActive = index === state.current_index;
            return (
              <div
                key={item.id}
                className={`queue-item ${isActive ? 'active' : ''}`}
                onClick={isActive ? handleManualPaste : undefined}
                style={{ cursor: isActive ? 'pointer' : 'default' }}
              >
                <div className="item-left">
                  <span className="index-pill">{index + 1}</span>
                  <span className="item-content">{item.content}</span>
                </div>
                {isActive && <span className="next-badge">準備貼上 NEXT</span>}
              </div>
            );
          })
        )}
      </section>
    </div>
  );
}
