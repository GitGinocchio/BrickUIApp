

export function RGBAToHex(color: [number, number, number, number]): string {
  return (
    '#' +
    color
      .map(c => c.toString(16).padStart(2, '0'))
      .join('')
      .toUpperCase()
  );
}

function normalizeAlpha(a: string | undefined): number {
  if (a === undefined) return 255;

  if (a.includes('%')) {
    // percentuale
    return Math.round((parseFloat(a) / 100) * 255);
  }

  const num = parseFloat(a);

  if (num <= 1) {
    // decimale tra 0 e 1
    return Math.round(num * 255);
  }

  // altrimenti lo considero intero 0–255
  return Math.round(num);
}

export function colorStringToRGBA(input: string): string {
  input = input.trim();

  console.log("colorStringToRGBA input", input);

  // HEX
  if (input.startsWith('#')) {
    input = input.slice(1);
    let r = 0, g = 0, b = 0, a = 255;
    if (input.length === 6) {
      r = parseInt(input.slice(0, 2), 16);
      g = parseInt(input.slice(2, 4), 16);
      b = parseInt(input.slice(4, 6), 16);
    } else if (input.length === 8) {
      r = parseInt(input.slice(0, 2), 16);
      g = parseInt(input.slice(2, 4), 16);
      b = parseInt(input.slice(4, 6), 16);
      a = parseInt(input.slice(6, 8), 16);
    } else {
      throw new Error('Formato esadecimale non valido');
    }
    return RGBAToHex([r, g, b, a]);
  }

  // RGB / RGBA
  let rgbMatch = input.match(/rgba?\(\s*(\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.%]+))?\s*\)/i);
  if (rgbMatch) {
    const r = parseInt(rgbMatch[1]);
    const g = parseInt(rgbMatch[2]);
    const b = parseInt(rgbMatch[3]);
    const a = normalizeAlpha(rgbMatch[4]);
    return RGBAToHex([r, g, b, a]);
  }

  // HSL / HSLA
  let hslMatch = input.match(/hsla?\(\s*(\d+),\s*(\d+)%?,\s*(\d+)%?(?:,\s*([\d.%]+))?\s*\)/i);
  if (hslMatch) {
    let h = parseInt(hslMatch[1]);
    let s = parseInt(hslMatch[2]) / 100;
    let l = parseInt(hslMatch[3]) / 100;
    const alpha = normalizeAlpha(hslMatch[4]);

    const c = (1 - Math.abs(2 * l - 1)) * s;
    const x = c * (1 - Math.abs((h / 60) % 2 - 1));
    const m = l - c / 2;
    let r = 0, g = 0, b = 0;

    if (h < 60)      { r = c; g = x; b = 0; }
    else if (h < 120){ r = x; g = c; b = 0; }
    else if (h < 180){ r = 0; g = c; b = x; }
    else if (h < 240){ r = 0; g = x; b = c; }
    else if (h < 300){ r = x; g = 0; b = c; }
    else             { r = c; g = 0; b = x; }

    return RGBAToHex([
      Math.round((r + m) * 255),
      Math.round((g + m) * 255),
      Math.round((b + m) * 255),
      alpha
    ]);
  }

  throw new Error('Formato colore non supportato');
}