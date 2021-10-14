import { FontMetrics } from "canvaskit-wasm";
import {
  EngineContext,
  TextAlign,
  TextBaseline,
  TextDrawCommand,
} from "../type";

export function drawText(
  engineContext: EngineContext,
  command: TextDrawCommand,
): void {
  const { canvasKit, canvas } = engineContext;
  const { text, font, x, y, paint, baseline, align } = command;
  if (!text.length) {
    return;
  }
  const glyphIds = font.getGlyphIDs(text);
  const widths = font.getGlyphWidths(glyphIds, paint);
  const width = widths.reduce((prev, curr) => prev + curr, 0);
  const metrics = font.getMetrics();
  const bottom = y + getBottomOfBaseline(baseline, metrics);
  const left = getLeftInAlign(x, align, width);

  const textBlob = canvasKit.TextBlob.MakeFromGlyphs(glyphIds, font);

  canvas.drawTextBlob(textBlob, left, bottom, paint);

  textBlob.delete();
}
export function getLeftInAlign(
  x: number,
  align: TextAlign,
  width: number,
): number {
  switch (align) {
    case "left":
      return x;
    case "right":
      return x - width;
    case "center":
      return x - width / 2;
    default:
      throw new Error(`Unknown align ${align}`);
  }
}
export function getBottomOfBaseline(
  baseline: TextBaseline,
  fontMetrics: FontMetrics,
) {
  const { ascent, descent } = fontMetrics;
  switch (baseline) {
    case "top":
      return -ascent;
    case "bottom":
      return -descent;
    case "middle":
      return (-ascent - descent) / 2;
    default:
      throw new Error(`Unknown baseline ${baseline}`);
  }
}
