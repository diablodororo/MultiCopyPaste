import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  Copy,
  RotateCcw,
  Trash2,
  Keyboard,
  ClipboardList,
  GripVertical
} from 'lucide-react';
import { type Language, translations } from './locales';

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
  const [lang, setLang] = useState<Language>(() => {
    const saved = localStorage.getItem('app_lang');
    return saved === 'en' ? 'en' : 'zh';
  });

  const t = translations[lang];

  const [state, setState] = useState<SequenceState>({
    target_length: 3,
    current_index: 0,
    items: [],
    history: [],
    shortcut: 'Ctrl+Option+V / Ctrl+Alt+V',
    is_enabled: true
  });

  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const draggedIndexRef = useRef<number | null>(null);
  const lastDragEndTimeRef = useRef<number>(0);

  const handleLanguageChange = (newLang: Language) => {
    setLang(newLang);
    localStorage.setItem('app_lang', newLang);
  };

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

  const handleSetIndex = async (index: number) => {
    try {
      const res = await invoke<SequenceState>('set_sequence_index', { index });
      setState(res);
    } catch (e) {
      console.error(e);
    }
  };

  const handleReorder = async (newItems: ClipboardItem[]) => {
    try {
      const res = await invoke<SequenceState>('update_sequence_items', { items: newItems });
      setState(res);
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
            <h1>{t.appTitle}</h1>
            <span>{t.appSubtitle}</span>
          </div>
        </div>

        <div className="header-right">
          <span className="version-pill">{t.appVersion}</span>
          <select
            className="lang-select"
            value={lang}
            onChange={(e) => handleLanguageChange(e.target.value as Language)}
          >
            <option value="zh">{t.langZh}</option>
            <option value="en">{t.langEn}</option>
          </select>
          <div className="status-badge">
            <span className="status-dot"></span>
            {t.statusListening}
          </div>
        </div>
      </header>

      {/* Settings Card */}
      <section className="config-card">
        <div className="config-row">
          <div className="config-label">
            <h3>{t.sequenceLengthTitle}</h3>
            <p>{t.sequenceLengthDesc}</p>
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
            <h3>{t.shortcutTitle}</h3>
            <p>{t.shortcutDesc}</p>
          </div>
          <div className="shortcut-badge">
            <Keyboard size={14} style={{ display: 'inline', marginRight: 6, verticalAlign: 'middle' }} />
            {t.shortcutBadge}
          </div>
        </div>
      </section>

      {/* Queue Display Section */}
      <section className="queue-header">
        <h2>
          {t.queueTitle} ({state.items.length}/{state.target_length})
        </h2>
        <div className="action-buttons">
          <button className="btn-secondary" onClick={handleResetIndex} title={t.resetTop}>
            <RotateCcw size={14} />
            {t.resetTop}
          </button>
          <button className="btn-secondary" onClick={handleClear} title={t.clearAll}>
            <Trash2 size={14} />
            {t.clearAll}
          </button>
        </div>
      </section>

      {state.items.length > 0 && (
        <div className="drag-hint">
          {t.dragHint}
        </div>
      )}

      {/* Items List */}
      <section className="queue-list">
        {state.items.length === 0 ? (
          <div className="empty-state">
            <ClipboardList size={32} color="var(--text-muted)" />
            <p>{t.emptyTitle}</p>
            <span>{t.emptyDesc(state.target_length)}</span>
          </div>
        ) : (
          state.items.map((item, index) => {
            const isActive = index === state.current_index;
            const isDragging = draggedIndex === index;
            const isDragOver = dragOverIndex === index && draggedIndex !== index;

            return (
              <div
                key={item.id}
                draggable={true}
                onDragStart={(e) => {
                  draggedIndexRef.current = index;
                  setDraggedIndex(index);
                  e.dataTransfer.effectAllowed = 'move';
                  e.dataTransfer.setData('text/plain', index.toString());
                }}
                onDragEnter={(e) => {
                  e.preventDefault();
                  e.dataTransfer.dropEffect = 'move';
                  if (dragOverIndex !== index) {
                    setDragOverIndex(index);
                  }
                }}
                onDragOver={(e) => {
                  e.preventDefault();
                  e.dataTransfer.dropEffect = 'move';
                  if (dragOverIndex !== index) {
                    setDragOverIndex(index);
                  }
                }}
                onDragEnd={() => {
                  lastDragEndTimeRef.current = Date.now();
                  setDraggedIndex(null);
                  setDragOverIndex(null);
                  setTimeout(() => {
                    draggedIndexRef.current = null;
                  }, 50);
                }}
                onDrop={async (e) => {
                  e.preventDefault();
                  lastDragEndTimeRef.current = Date.now();
                  const dataStr = e.dataTransfer.getData('text/plain');
                  const fromIdx = (dataStr && !isNaN(parseInt(dataStr, 10)))
                    ? parseInt(dataStr, 10)
                    : draggedIndexRef.current;
                  draggedIndexRef.current = null;
                  setDraggedIndex(null);
                  setDragOverIndex(null);

                  if (fromIdx === null || fromIdx === undefined || isNaN(fromIdx) || fromIdx === index) return;
                  const newItems = [...state.items];
                  const [movedItem] = newItems.splice(fromIdx, 1);
                  newItems.splice(index, 0, movedItem);

                  setState((prev) => ({ ...prev, items: newItems }));
                  await handleReorder(newItems);
                }}
                className={`queue-item ${isActive ? 'active' : ''} ${isDragging ? 'dragging' : ''} ${isDragOver ? 'drag-over' : ''}`}
                onClick={(e) => {
                  if (draggedIndexRef.current !== null || Date.now() - lastDragEndTimeRef.current < 300) {
                    e.preventDefault();
                    e.stopPropagation();
                    return;
                  }
                  if (isActive) {
                    handleManualPaste();
                  } else {
                    handleSetIndex(index);
                  }
                }}
                style={{ cursor: 'pointer' }}
              >
                <div className="item-left">
                  <span className="drag-handle" title="拖曳調整順序 / Drag to reorder">
                    <GripVertical size={16} />
                  </span>
                  <span className="index-pill">{index + 1}</span>
                  <span className="item-content">{item.content}</span>
                </div>
                {isActive && <span className="next-badge">{t.nextBadge}</span>}
              </div>
            );
          })
        )}
      </section>
    </div>
  );
}
