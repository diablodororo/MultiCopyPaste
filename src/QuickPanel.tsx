import { useState, useEffect, type ReactElement } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { SlidersHorizontal, AppWindow, ClipboardList } from 'lucide-react';
import { type Language, translations } from './locales';
import { SettingSlider } from './components/SettingSlider';

interface QuickPanelItem {
  id: string;
  content: string;
}

interface QuickPanelState {
  target_length: number;
  repeat_count: number;
  current_index: number;
  items: QuickPanelItem[];
}

// Slider position 11 (far right) maps to repeat_count 0 = infinite,
// so dragging right always means "repeat more".
const INFINITE_SLIDER_POSITION = 11;

const readLang = (): Language =>
  localStorage.getItem('app_lang') === 'en' ? 'en' : 'zh';

export default function QuickPanel(): ReactElement {
  const [lang, setLang] = useState<Language>(readLang);
  const [targetLength, setTargetLength] = useState<number>(3);
  const [repeatCount, setRepeatCount] = useState<number>(1);
  const [currentIndex, setCurrentIndex] = useState<number>(0);
  const [items, setItems] = useState<QuickPanelItem[]>([]);

  const t = translations[lang];

  const applyState = (state: QuickPanelState): void => {
    setTargetLength(state.target_length);
    setRepeatCount(state.repeat_count);
    setCurrentIndex(state.current_index);
    setItems(state.items);
  };

  useEffect(() => {
    invoke<QuickPanelState>('get_sequence_state').then(applyState).catch(console.error);

    const unlistenPromise = listen<QuickPanelState>('sequence-updated', (event) => {
      applyState(event.payload);
    });

    // The main window may switch the UI language while this panel is hidden;
    // re-read it whenever the panel is shown (regains focus) or storage changes.
    const syncLang = (): void => setLang(readLang());
    window.addEventListener('focus', syncLang);
    window.addEventListener('storage', syncLang);

    const onKeyDown = (e: KeyboardEvent): void => {
      if (e.key === 'Escape') void getCurrentWebviewWindow().hide();
    };
    window.addEventListener('keydown', onKeyDown);

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
      window.removeEventListener('focus', syncLang);
      window.removeEventListener('storage', syncLang);
      window.removeEventListener('keydown', onKeyDown);
    };
  }, []);

  const handleLengthChange = (value: number): void => {
    setTargetLength(value);
    invoke('set_target_length', { length: value }).catch(console.error);
  };

  const handleRepeatChange = (sliderValue: number): void => {
    const count = sliderValue === INFINITE_SLIDER_POSITION ? 0 : sliderValue;
    setRepeatCount(count);
    invoke('set_repeat_count', { count }).catch(console.error);
  };

  const handleJump = (index: number): void => {
    invoke<QuickPanelState>('set_sequence_index', { index })
      .then(applyState)
      .catch(console.error);
  };

  const openMainWindow = (): void => {
    invoke('show_main_window').catch(console.error);
    void getCurrentWebviewWindow().hide();
  };

  const isMac = navigator.platform.toUpperCase().includes('MAC');

  return (
    <div className={`quick-panel ${isMac ? 'popover-top' : ''}`}>
      <div className="quick-panel-header">
        <SlidersHorizontal size={13} />
        <span>{t.quickPanelTitle}</span>
      </div>

      <div className="quick-setting">
        <div className="quick-setting-label">
          <span>{t.sequenceLengthTitle}</span>
          <span className="quick-setting-value">{targetLength}</span>
        </div>
        <SettingSlider
          min={1}
          max={12}
          value={targetLength}
          onChange={handleLengthChange}
          ariaLabel={t.sequenceLengthTitle}
        />
      </div>

      <div className="quick-setting">
        <div className="quick-setting-label">
          <span>{t.repeatCountTitle}</span>
          <span className="quick-setting-value">{t.repeatTimesShort(repeatCount)}</span>
        </div>
        <SettingSlider
          min={1}
          max={INFINITE_SLIDER_POSITION}
          value={repeatCount === 0 ? INFINITE_SLIDER_POSITION : repeatCount}
          onChange={handleRepeatChange}
          ariaLabel={t.repeatCountTitle}
        />
      </div>

      <div className="quick-queue">
        <div className="quick-queue-title">
          {t.queueTitle} ({items.length}/{targetLength})
        </div>
        {items.length === 0 ? (
          <div className="quick-queue-empty">
            <ClipboardList size={16} />
            <span>{t.emptyTitle}</span>
          </div>
        ) : (
          <div className="quick-queue-list">
            {items.map((item, index) => (
              <button
                key={item.id}
                className={`quick-queue-item ${index === currentIndex ? 'active' : ''}`}
                onClick={() => handleJump(index)}
                title={t.jumpToItem}
              >
                <span className="quick-queue-index">{index + 1}</span>
                <span className="quick-queue-content">{item.content}</span>
                {index === currentIndex && (
                  <span className="quick-queue-next">{t.nextBadge}</span>
                )}
              </button>
            ))}
          </div>
        )}
      </div>

      <button className="quick-open-main" onClick={openMainWindow}>
        <AppWindow size={14} />
        {t.openMainWindow}
      </button>
    </div>
  );
}
