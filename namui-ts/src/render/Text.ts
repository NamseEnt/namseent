import { Color, Font } from "canvaskit-wasm";
import { getLeftInAlign, getBottomOfBaseline } from "../draw/drawText";
import { FontType } from "../font/FontStorage";
import { Language } from "../l10n/type";
import {
  DrawCommand,
  RenderingTree,
  TextAlign,
  TextBaseline,
  TextDrawCommand,
} from "../type";
import { Rect } from "./Rect";

export type TextParam = {
  text: string;
  x: number;
  y: number;
  align: TextAlign;
  baseline: TextBaseline;
  fontType: FontType;
  style: {
    border?: {
      width: number;
      color: Color;
    };
    dropShadow?: {
      x: number;
      y: number;
      color?: Color;
    };
    color: Color;
    background?: {
      color: Color;
      margin?: {
        top?: number;
        bottom?: number;
        left?: number;
        right?: number;
      };
    };
  };
};

export function Text(param: TextParam): RenderingTree {
  const { fontType } = param;
  const font = fontStorage.getFont(fontType);

  const textHandleParam: TextHandleParam = {
    ...param,
    font,
  };

  return [
    drawBackground(textHandleParam),
    {
      drawCalls: [
        {
          commands: [
            drawShadow(textHandleParam),
            drawText(textHandleParam),
            drawBorder(textHandleParam),
          ].filter((x): x is TextDrawCommand => !!x),
        },
      ],
    },
  ];
}

type TextHandleParam = TextParam & {
  font: Font;
};

function drawShadow({
  x,
  y,
  align,
  baseline,
  text,
  style: { dropShadow, color },
  font,
}: TextHandleParam): TextDrawCommand | undefined {
  if (!dropShadow) {
    return;
  }
  const shadowPaint = new CanvasKit.Paint();
  shadowPaint.setColor(dropShadow.color || color);
  shadowPaint.setStyle(CanvasKit.PaintStyle.Fill);
  shadowPaint.setAntiAlias(true);

  return TextDrawCommand({
    text,
    font,
    x: x + dropShadow.x,
    y: y + dropShadow.y,
    paint: shadowPaint,
    align,
    baseline,
  });
}
function drawText({
  text,
  font,
  x,
  y,
  align,
  baseline,
  style,
}: TextHandleParam): TextDrawCommand | undefined {
  const textPaint = new CanvasKit.Paint();
  textPaint.setColor(style.color);
  textPaint.setStyle(CanvasKit.PaintStyle.Fill);
  textPaint.setAntiAlias(true);

  return TextDrawCommand({
    text,
    font,
    x,
    y,
    paint: textPaint,
    align,
    baseline,
  });
}
function drawBorder({
  style: { border },
  text,
  font,
  x,
  y,
  align,
  baseline,
}: TextHandleParam): TextDrawCommand | undefined {
  if (!border) {
    return;
  }
  const borderPaint = new CanvasKit.Paint();
  borderPaint.setColor(border.color);
  borderPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  borderPaint.setStrokeWidth(border.width);
  borderPaint.setStrokeJoin(CanvasKit.StrokeJoin.Miter);
  borderPaint.setAntiAlias(true);

  return TextDrawCommand({
    text,
    font,
    x,
    y,
    paint: borderPaint,
    align,
    baseline,
  });
}
function drawBackground({
  x,
  y,
  align,
  baseline,
  text,
  font,
  style: { background, dropShadow },
}: TextHandleParam): RenderingTree {
  if (!background) {
    return;
  }
  const { color, margin } = background;
  const width = getTextWidth(font, text, dropShadow?.x);
  const fontMetrics = font.getMetrics();
  const glyphIds = font.getGlyphIDs(text);
  const glyphsTopBottom = getGlyphsTopBottom(font, glyphIds);

  let height: number;
  let top: number;
  if (glyphsTopBottom) {
    const { glyphsTop, glyphsBottom } = glyphsTopBottom;
    height = glyphsBottom - glyphsTop;
    top = y + getBottomOfBaseline(baseline, fontMetrics) + glyphsTop;

    if (dropShadow) {
      height += dropShadow.y;
    }
  } else {
    height = -fontMetrics.ascent + fontMetrics.descent;
    top = y + getBottomOfBaseline(baseline, fontMetrics) + fontMetrics.ascent;
  }

  const finalX = -(margin?.left ?? 0) + getLeftInAlign(x, align, width);
  const finalY = -(margin?.top ?? 0) + top;
  const finalWidth = width + (margin?.left ?? 0) + (margin?.right ?? 0);
  const finalHeight = height + (margin?.top ?? 0) + (margin?.bottom ?? 0);

  return Rect({
    x: finalX,
    y: finalY,
    width: finalWidth,
    height: finalHeight,
    style: {
      fill: {
        color,
      },
    },
  });
}

export function getTextWidth(
  font: Font,
  text: string,
  dropShadowX?: number,
): number {
  const glyphIds = font.getGlyphIDs(text);
  const glyphWidths = font.getGlyphWidths(glyphIds);
  const width =
    glyphWidths.reduce((acc, cur) => acc + cur, 0) + (dropShadowX ?? 0);
  return width;
}

export function getGlyphsTopBottom(
  font: Font,
  glyphIds: ReturnType<Font["getGlyphIDs"]>,
):
  | {
      glyphsTop: number;
      glyphsBottom: number;
    }
  | undefined {
  if (!glyphIds.length) {
    return undefined;
  }
  const glyphBounds = font.getGlyphBounds(glyphIds);
  const glyphsTop = glyphBounds
    .filter((_, index) => index % 4 === 1)
    .reduce((prev, cur) => {
      return Math.min(prev, cur);
    });
  const glyphsBottom = glyphBounds
    .filter((_, index) => index % 4 === 3)
    .reduce((prev, cur) => {
      return Math.max(prev, cur);
    });

  return {
    glyphsTop,
    glyphsBottom,
  };
}
