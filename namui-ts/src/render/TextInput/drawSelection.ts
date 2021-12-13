import { RenderingTree, TextAlign, TextBaseline } from "../../type";
import { Rect } from "../Rect";
import { Selection } from "../../textInput/ITextInputManager";
import { ColorUtil } from "../..";
import { FontType } from "../../font/FontStorage";
import { Font } from "canvaskit-wasm";
import { getTextWidth } from "../Text";

export function drawSelection(param: {
  text: string;
  selection?: Selection;
  align: TextAlign;
  width: number;
  baseline: TextBaseline;
  fontType: FontType;
  dropShadowX: number | undefined;
}): RenderingTree {
  if (!param.selection) {
    return;
  }

  const font = fontStorage.getFont(param.fontType);
  const fontMetrics = font.getMetrics();

  const y = 0; // TODO: this is only for TextBaseline.top. support middle and bottom.
  const height = -fontMetrics.ascent + fontMetrics.descent;

  const minSelectionIndex = Math.min(
    param.selection.start,
    param.selection.end,
  );
  const maxSelectionIndex = Math.max(
    param.selection.start,
    param.selection.end,
  );

  const shouldDrawSelectionAsLine = minSelectionIndex === maxSelectionIndex;

  const alignBaseX =
    param.align === TextAlign.left
      ? 0
      : param.align === TextAlign.center
      ? param.width / 2 - getTextWidth(font, param.text, param.dropShadowX) / 2
      : param.width - getTextWidth(font, param.text, param.dropShadowX);

  const minCaretX = getCaretX({
    selectionIndex: minSelectionIndex,
    font,
    text: param.text,
  });
  const maxCaretX = getCaretX({
    selectionIndex: maxSelectionIndex,
    font,
    text: param.text,
  });

  const x = alignBaseX + minCaretX;

  const width = shouldDrawSelectionAsLine ? 1 : maxCaretX - minCaretX;

  return Rect({
    x,
    y,
    width,
    height,
    style: {
      fill: {
        color: ColorUtil.Blue,
      },
    },
  });
}

function getCaretX({
  selectionIndex,
  font,
  text,
}: {
  selectionIndex: number;
  font: Font;
  text: string;
}): number {
  if (!text.length) {
    return 0;
  }
  const glyphIds = font.getGlyphIDs(text);
  const glyphWidths = font.getGlyphWidths(glyphIds);
  const x = glyphWidths
    .slice(0, selectionIndex)
    .reduce((acc, width) => acc + width, 0);

  return x;
}
