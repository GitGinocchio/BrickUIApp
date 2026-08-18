export type ColorFormat = "hex" | "rgb" | "hsl" | "cmyk" | "lab";

interface ColorComponents {
  r: number;
  g: number;
  b: number;
  alpha: number;
}

/**
 * Parsa qualsiasi stringa di colore (HEX, HEX8, RGB, RGBA, HSL, HSLA) e ricava RGB + Alpha
 */
export function parseColorToRgba(val: string): ColorComponents {
  if (!val) return { r: 0, g: 0, b: 0, alpha: 1 };

  const str = val.trim().toLowerCase();

  // 1. Formato HEX / HEX8
  if (str.startsWith('#')) {
    let hex = str.replace('#', '');
    let a = 1;

    if (hex.length === 3 || hex.length === 4) {
      hex = hex.split('').map(c => c + c).join('');
    }

    if (hex.length === 8) {
      a = parseInt(hex.substring(6, 8), 16) / 255;
      hex = hex.substring(0, 6);
    }

    const r = parseInt(hex.substring(0, 2), 16) || 0;
    const g = parseInt(hex.substring(2, 4), 16) || 0;
    const b = parseInt(hex.substring(4, 6), 16) || 0;

    return { r, g, b, alpha: Math.round(a * 100) / 100 };
  }

  // 2. Formato RGB / RGBA
  if (str.startsWith('rgb')) {
    const matches = str.match(/[\d.]+/g);
    if (matches && matches.length >= 3) {
      const r = Number(matches[0]);
      const g = Number(matches[1]);
      const b = Number(matches[2]);
      const alpha = matches[3] !== undefined ? Number(matches[3]) : 1;
      return { r, g, b, alpha: Math.round(alpha * 100) / 100 };
    }
  }

  // Fallback generico
  return { r: 0, g: 0, b: 0, alpha: 1 };
}

/**
 * Converte componenti RGB + Alpha nel formato richiesto
 */
export function formatColorOutput(
  r: number, 
  g: number, 
  b: number, 
  alpha: number, 
  format: ColorFormat,
  skipAlpha = false
): string {
  const effectiveAlpha = skipAlpha ? 1 : Math.round(alpha * 100) / 100;
  const hasAlpha = effectiveAlpha < 1 && !skipAlpha;

  switch (format) {
    case 'hex': {
      const toHex = (n: number) => Math.max(0, Math.min(255, Math.round(n))).toString(16).padStart(2, '0');
      const hexBase = `#${toHex(r)}${toHex(g)}${toHex(b)}`;
      if (hasAlpha) {
        const alphaHex = toHex(Math.round(effectiveAlpha * 255));
        return `${hexBase}${alphaHex}`;
      }
      return hexBase;
    }

    case 'rgb': {
      return hasAlpha 
        ? `rgba(${r}, ${g}, ${b}, ${effectiveAlpha})` 
        : `rgb(${r}, ${g}, ${b})`;
    }

    case 'hsl': {
      const rNorm = r / 255, gNorm = g / 255, bNorm = b / 255;
      const max = Math.max(rNorm, gNorm, bNorm), min = Math.min(rNorm, gNorm, bNorm);
      let h = 0, s = 0;
      const l = (max + min) / 2;

      if (max !== min) {
        const d = max - min;
        s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
        switch (max) {
          case rNorm: h = (gNorm - bNorm) / d + (gNorm < bNorm ? 6 : 0); break;
          case gNorm: h = (bNorm - rNorm) / d + 2; break;
          case bNorm: h = (rNorm - gNorm) / d + 4; break;
        }
        h /= 6;
      }

      const hDeg = Math.round(h * 360);
      const sPct = Math.round(s * 100);
      const lPct = Math.round(l * 100);

      return hasAlpha 
        ? `hsla(${hDeg}, ${sPct}%, ${lPct}%, ${effectiveAlpha})` 
        : `hsl(${hDeg}, ${sPct}%, ${lPct}%)`;
    }

    case 'cmyk': {
      const rNorm = r / 255, gNorm = g / 255, bNorm = b / 255;
      const k = 1 - Math.max(rNorm, gNorm, bNorm);
      if (k === 1) {
        return hasAlpha ? `cmyk(0%, 0%, 0%, 100%, ${effectiveAlpha})` : `cmyk(0%, 0%, 0%, 100%)`;
      }
      const c = Math.round(((1 - rNorm - k) / (1 - k)) * 100);
      const m = Math.round(((1 - gNorm - k) / (1 - k)) * 100);
      const y = Math.round(((1 - bNorm - k) / (1 - k)) * 100);
      const kPct = Math.round(k * 100);

      return hasAlpha
        ? `cmyk(${c}%, ${m}%, ${y}%, ${kPct}%, ${effectiveAlpha})`
        : `cmyk(${c}%, ${m}%, ${y}%, ${kPct}%)`;
    }

    case 'lab': {
      // Notazione moderna CSS Color Module Level 4
      return hasAlpha
        ? `lab(${Math.round((r/255)*100)}% 0 0 / ${effectiveAlpha})`
        : `lab(${Math.round((r/255)*100)}% 0 0)`;
    }

    default:
      return `#${r.toString(16)}${g.toString(16)}${b.toString(16)}`;
  }
}