export function convertColorToHsl(color: Float32Array) {
  const [r, g, b, a] = color;
  const normalizedR = r || 0;
  const normalizedG = g || 0;
  const normalizedB = b || 0;
  const max = Math.max(normalizedR, normalizedG, normalizedB);
  const min = Math.min(normalizedR, normalizedG, normalizedB);
  const delta = max - min;

  let hue = 0;
  if (delta !== 0) {
    switch (max) {
      case normalizedR: {
        hue = (normalizedG - normalizedB) / delta;
        break;
      }
      case normalizedG: {
        hue = (normalizedB - normalizedR) / delta + 2;
        break;
      }
      case normalizedB: {
        hue = (normalizedR - normalizedG) / delta + 4;
        break;
      }
      default: {
        throw new Error("Can not calculate hue.");
      }
    }
  }
  hue = (hue < 0 ? hue + 6 : hue) / 6;

  const lightness = (max + min) / 2;
  const saturation =
    delta === 0 ? 0 : delta / (1 - Math.abs(2 * lightness - 1));

  return {
    hue,
    saturation,
    lightness,
    alpha: a || 0,
  };
}
