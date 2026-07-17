import type { CSSProperties, ReactElement } from 'react';

interface SettingSliderProps {
  min: number;
  max: number;
  value: number;
  onChange: (value: number) => void;
  ariaLabel: string;
}

export function SettingSlider({
  min,
  max,
  value,
  onChange,
  ariaLabel,
}: SettingSliderProps): ReactElement {
  const progress = ((value - min) / (max - min)) * 100;
  return (
    <input
      type="range"
      className="setting-slider"
      min={min}
      max={max}
      step={1}
      value={value}
      aria-label={ariaLabel}
      onChange={(e) => onChange(Number(e.target.value))}
      style={{ '--slider-progress': `${progress}%` } as CSSProperties}
    />
  );
}
