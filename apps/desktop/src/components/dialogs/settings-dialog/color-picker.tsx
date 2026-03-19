import { useRef, useState, useCallback } from "react";

type Props = {
  value: string;
  onChange: (hex: string) => void;
};

function hexToHsl(hex: string): [number, number, number] {
  const r = parseInt(hex.slice(1, 3), 16) / 255;
  const g = parseInt(hex.slice(3, 5), 16) / 255;
  const b = parseInt(hex.slice(5, 7), 16) / 255;
  const max = Math.max(r, g, b),
    min = Math.min(r, g, b);
  let h = 0,
    s = 0;
  const l = (max + min) / 2;
  if (max !== min) {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    switch (max) {
      case r:
        h = ((g - b) / d + (g < b ? 6 : 0)) / 6;
        break;
      case g:
        h = ((b - r) / d + 2) / 6;
        break;
      case b:
        h = ((r - g) / d + 4) / 6;
        break;
    }
  }
  return [h * 360, s * 100, l * 100];
}

function hslToHex(h: number, s: number, l: number): string {
  s /= 100;
  l /= 100;
  const a = s * Math.min(l, 1 - l);
  const f = (n: number) => {
    const k = (n + h / 30) % 12;
    const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    return Math.round(255 * color)
      .toString(16)
      .padStart(2, "0");
  };
  return `#${f(0)}${f(8)}${f(4)}`;
}

export function SimpleColorPicker({ value, onChange }: Props) {
  const [hsl, setHsl] = useState<[number, number, number]>(() => {
    try {
      return hexToHsl(value);
    } catch {
      return [270, 100, 50];
    }
  });

  const canvasRef = useRef<HTMLDivElement>(null);
  const isDragging = useRef(false);

  const [h, s, l] = hsl;

  const updateFromCanvas = useCallback(
    (e: React.MouseEvent | MouseEvent) => {
      const canvas = canvasRef.current;
      if (!canvas) return;
      const rect = canvas.getBoundingClientRect();
      const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const y = Math.max(0, Math.min(1, (e.clientY - rect.top) / rect.height));
      const newS = x * 100;
      const newL = (1 - y) * 50;
      const newHsl: [number, number, number] = [h, newS, newL];
      setHsl(newHsl);
      onChange(hslToHex(...newHsl));
    },
    [h, onChange]
  );

  const handleCanvasMouseDown = (e: React.MouseEvent) => {
    isDragging.current = true;
    updateFromCanvas(e);

    const onMove = (e: MouseEvent) => {
      if (isDragging.current) updateFromCanvas(e);
    };
    const onUp = () => {
      isDragging.current = false;
      window.removeEventListener("mousemove", onMove);
      window.removeEventListener("mouseup", onUp);
    };
    window.addEventListener("mousemove", onMove);
    window.addEventListener("mouseup", onUp);
  };

  const selectorX = s;
  const selectorY = 100 - (l / 50) * 100;
  const pureHue = hslToHex(h, 100, 50);
  const currentHex = hslToHex(h, s, l);

  return (
    <div className="flex flex-col gap-3 w-56">
      {/* Canvas */}
      <div
        ref={canvasRef}
        className="relative h-36 w-full cursor-crosshair rounded-md"
        style={{
          background: `
            linear-gradient(to top, #000, transparent),
            linear-gradient(to right, #fff, transparent),
            ${pureHue}
          `
        }}
        onMouseDown={handleCanvasMouseDown}
      >
        <div
          className="absolute size-3 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 border-white pointer-events-none"
          style={{
            left: `${selectorX}%`,
            top: `${selectorY}%`,
            boxShadow: "0 0 0 1px rgba(0,0,0,0.5)"
          }}
        />
      </div>

      {/* Hue slider */}
      <div
        className="relative h-3 w-full rounded-full cursor-pointer"
        style={{
          background:
            "linear-gradient(to right, #f00, #ff0, #0f0, #0ff, #00f, #f0f, #f00)"
        }}
        onMouseDown={(e) => {
          const update = (e: MouseEvent | React.MouseEvent) => {
            const rect =
              (e.currentTarget as HTMLElement)?.getBoundingClientRect() ??
              (e.target as HTMLElement)
                .closest(".hue-track")
                ?.getBoundingClientRect();
            if (!rect) return;
            const x = Math.max(
              0,
              Math.min(1, (e.clientX - rect.left) / rect.width)
            );
            const newH = x * 360;
            const newHsl: [number, number, number] = [newH, s, l];
            setHsl(newHsl);
            onChange(hslToHex(...newHsl));
          };
          update(e);
          const onMove = (e: MouseEvent) => update(e);
          const onUp = () => {
            window.removeEventListener("mousemove", onMove);
            window.removeEventListener("mouseup", onUp);
          };
          window.addEventListener("mousemove", onMove);
          window.addEventListener("mouseup", onUp);
        }}
      >
        <div
          className="absolute top-1/2 -translate-y-1/2 -translate-x-1/2 size-4 rounded-full border-2 border-white pointer-events-none"
          style={{
            left: `${(h / 360) * 100}%`,
            boxShadow: "0 0 0 1px rgba(0,0,0,0.3)",
            backgroundColor: pureHue
          }}
        />
      </div>

      {/* Hex input + preview */}
      <div className="flex items-center gap-2">
        <div
          className="size-7 rounded border border-border shrink-0"
          style={{ backgroundColor: currentHex }}
        />
        <input
          className="flex-1 h-7 rounded-md border border-input bg-transparent px-2 text-xs font-mono focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
          value={currentHex.toUpperCase()}
          onChange={(e) => {
            const val = e.target.value;
            if (/^#[0-9A-Fa-f]{6}$/.test(val)) {
              try {
                const newHsl = hexToHsl(val);
                setHsl(newHsl);
                onChange(val);
              } catch {}
            }
          }}
          maxLength={7}
          spellCheck={false}
        />
      </div>
    </div>
  );
}
