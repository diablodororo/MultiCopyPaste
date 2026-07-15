import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  Copy,
  RotateCcw,
  Trash2,
  Keyboard,
  ClipboardList,
  GripVertical,
  Pencil,
  Check,
  X
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
  const [dragCoords, setDragCoords] = useState<{ x: number; y: number } | null>(null);
  const [dragOffset, setDragOffset] = useState<{ x: number; y: number }>({ x: 0, y: 0 });
  const [draggedWidth, setDraggedWidth] = useState<number>(300);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editContent, setEditContent] = useState<string>('');
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
      invoke('log_debug', { msg: `handleReorder called with ${newItems.length} items: ${newItems.map(i => i.content).join(' -> ')}` });
      const res = await invoke<SequenceState>('update_sequence_items', { items: newItems });
      setState(res);
    } catch (e) {
      console.error(e);
      invoke('log_debug', { msg: `handleReorder error: ${e}` });
    }
  };

  const handleDeleteItem = async (id: string, e?: React.MouseEvent) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }
    try {
      const res = await invoke<SequenceState>('delete_sequence_item', { id });
      setState(res);
      if (editingId === id) setEditingId(null);
    } catch (err) {
      console.error(err);
    }
  };

  const handleStartEdit = (item: ClipboardItem, e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setEditingId(item.id);
    setEditContent(item.content);
  };

  const handleCancelEdit = (e?: React.MouseEvent | React.KeyboardEvent) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }
    setEditingId(null);
    setEditContent('');
  };

  const handleSaveEdit = async (id: string, e?: React.MouseEvent | React.KeyboardEvent) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }
    if (!editContent.trim()) return;
    try {
      const res = await invoke<SequenceState>('update_sequence_item', { id, content: editContent });
      setState(res);
      setEditingId(null);
    } catch (err) {
      console.error(err);
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
            const isEditing = editingId === item.id;

            return (
              <div
                key={item.id}
                data-index={index}
                className={`queue-item ${isActive ? 'active' : ''} ${isDragging ? 'dragging' : ''} ${isDragOver ? 'drag-over' : ''} ${isEditing ? 'editing' : ''}`}
                onClick={(e) => {
                  if (isEditing || draggedIndexRef.current !== null || Date.now() - lastDragEndTimeRef.current < 300) {
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
                style={{ cursor: isEditing ? 'default' : 'pointer' }}
              >
                <div className="item-left">
                  <span
                    className="drag-handle"
                    title={t.dragHint}
                    onPointerDown={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      if (editingId !== null) return;

                      const handleEl = e.currentTarget as HTMLElement;
                      handleEl.setPointerCapture(e.pointerId);

                      draggedIndexRef.current = index;
                      setDraggedIndex(index);

                      const cardEl = handleEl.closest('.queue-item') as HTMLElement | null;
                      if (cardEl) {
                        const rect = cardEl.getBoundingClientRect();
                        setDragOffset({ x: e.clientX - rect.left, y: e.clientY - rect.top });
                        setDragCoords({ x: e.clientX, y: e.clientY });
                        setDraggedWidth(rect.width);
                      }

                      invoke('log_debug', { msg: `PointerDown on handle: index=${index}` });

                      const handlePointerMove = (moveEvent: PointerEvent) => {
                        setDragCoords({ x: moveEvent.clientX, y: moveEvent.clientY });
                        const elements = document.elementsFromPoint(moveEvent.clientX, moveEvent.clientY);
                        for (const el of elements) {
                          const itemEl = el.closest('.queue-item');
                          if (itemEl && !itemEl.classList.contains('floating-ghost') && !itemEl.classList.contains('dragging')) {
                            const idxStr = itemEl.getAttribute('data-index');
                            if (idxStr !== null) {
                              const targetIdx = parseInt(idxStr, 10);
                              if (!isNaN(targetIdx) && targetIdx !== draggedIndexRef.current) {
                                setDragOverIndex(targetIdx);
                                break;
                              }
                            }
                          }
                        }
                      };

                      const handlePointerEnd = async (upEvent: PointerEvent) => {
                        try {
                          handleEl.releasePointerCapture(upEvent.pointerId);
                        } catch (err) {
                          // Ignore if capture was already lost
                        }
                        window.removeEventListener('pointermove', handlePointerMove);
                        window.removeEventListener('pointerup', handlePointerEnd);
                        window.removeEventListener('pointercancel', handlePointerEnd);

                        const fromIdx = draggedIndexRef.current;
                        let toIdx = dragOverIndex;
                        const elements = document.elementsFromPoint(upEvent.clientX, upEvent.clientY);
                        for (const el of elements) {
                          const itemEl = el.closest('.queue-item');
                          if (itemEl && !itemEl.classList.contains('floating-ghost') && !itemEl.classList.contains('dragging')) {
                            const idxStr = itemEl.getAttribute('data-index');
                            if (idxStr !== null && !isNaN(parseInt(idxStr, 10))) {
                              const targetIdx = parseInt(idxStr, 10);
                              if (targetIdx !== fromIdx) {
                                toIdx = targetIdx;
                                break;
                              }
                            }
                          }
                        }

                        invoke('log_debug', { msg: `PointerEnd: fromIdx=${fromIdx}, toIdx=${toIdx}` });

                        lastDragEndTimeRef.current = Date.now();
                        draggedIndexRef.current = null;
                        setDraggedIndex(null);
                        setDragOverIndex(null);
                        setDragCoords(null);

                        if (fromIdx !== null && toIdx !== null && fromIdx !== toIdx && !isNaN(fromIdx) && !isNaN(toIdx)) {
                          const newItems = [...state.items];
                          const [movedItem] = newItems.splice(fromIdx, 1);
                          newItems.splice(toIdx, 0, movedItem);

                          invoke('log_debug', { msg: `Pointer reordering: ${newItems.map(i => i.content).join(' -> ')}` });
                          setState((prev) => ({ ...prev, items: newItems }));
                          await handleReorder(newItems);
                        }
                      };

                      window.addEventListener('pointermove', handlePointerMove);
                      window.addEventListener('pointerup', handlePointerEnd);
                      window.addEventListener('pointercancel', handlePointerEnd);
                    }}
                  >
                    <GripVertical size={16} />
                  </span>
                  <span
                    className="index-pill clickable-pill"
                    title={t.jumpToItem}
                    onClick={(e) => {
                      e.stopPropagation();
                      if (draggedIndexRef.current !== null) return;
                      handleSetIndex(index);
                    }}
                  >
                    {index + 1}
                  </span>
                  {isEditing ? (
                    <div className="edit-input-group" onClick={(e) => e.stopPropagation()}>
                      <input
                        type="text"
                        className="edit-input"
                        value={editContent}
                        onChange={(e) => setEditContent(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') handleSaveEdit(item.id, e as any);
                          if (e.key === 'Escape') handleCancelEdit(e as any);
                        }}
                        autoFocus
                      />
                      <button className="action-btn save-btn" title={t.saveEdit} onClick={(e) => handleSaveEdit(item.id, e)}>
                        <Check size={14} />
                      </button>
                      <button className="action-btn cancel-btn" title={t.cancelEdit} onClick={(e) => handleCancelEdit(e)}>
                        <X size={14} />
                      </button>
                    </div>
                  ) : (
                    <span className="item-content" title={item.content}>{item.content}</span>
                  )}
                </div>

                {!isEditing && (
                  <div className="item-right">
                    {isActive && <span className="next-badge">{t.nextBadge}</span>}
                    <div className="item-actions" onClick={(e) => e.stopPropagation()}>
                      <button
                        className="action-btn edit-btn"
                        title={t.editItem}
                        onClick={(e) => handleStartEdit(item, e)}
                      >
                        <Pencil size={14} />
                      </button>
                      <button
                        className="action-btn delete-btn"
                        title={t.deleteItem}
                        onClick={(e) => handleDeleteItem(item.id, e)}
                      >
                        <Trash2 size={14} />
                      </button>
                    </div>
                  </div>
                )}
              </div>
            );
          })
        )}
      </section>

      {/* Floating Drag Ghost Overlay */}
      {draggedIndex !== null && dragCoords !== null && state.items[draggedIndex] && (
        <div
          className="queue-item floating-ghost active"
          style={{
            position: 'fixed',
            left: `${dragCoords.x - dragOffset.x}px`,
            top: `${dragCoords.y - dragOffset.y}px`,
            width: `${draggedWidth}px`,
            pointerEvents: 'none',
            zIndex: 99999,
          }}
        >
          <div className="item-left">
            <span className="drag-handle" style={{ color: 'var(--text-primary)' }}>
              <GripVertical size={16} />
            </span>
            <span className="index-pill">{draggedIndex + 1}</span>
            <span className="item-content">{state.items[draggedIndex].content}</span>
          </div>
          {draggedIndex === state.current_index && <span className="next-badge">{t.nextBadge}</span>}
        </div>
      )}
    </div>
  );
}
