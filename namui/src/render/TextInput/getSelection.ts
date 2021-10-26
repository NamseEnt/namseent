import { Font } from "canvaskit-wasm";
import { Code, engine, Selection } from "../..";
import { FontType } from "../../font/FontStorage";
import { TextAlign } from "../../type";
import { getTextWidth } from "../Text";

export function getSelectionOnClick(param: {
  text: string;
  selection: Selection | undefined;
  x: number;
  width: number;
  align: TextAlign;
  fontType: FontType;
  dropShadowX: number | undefined;
}): Selection {
  const text = param.text;
  const font = fontStorage.getFont(param.fontType);

  // TODO : drag selection.
  const isShiftKeyPressed = engine.keyboard.anyCodePress([
    Code.ShiftLeft,
    Code.ShiftRight,
  ]);

  // const continouslyFastClickCount: number;

  // if (continouslyFastClickCount >= 3) {
  //   return getMoreTripleClickSelection({ text });
  // }
  // if (continouslyFastClickCount === 2) {
  //   return getDoubleClickSelection({ text, font, x: localX });
  // }
  const textWidth = getTextWidth(font, text, param.dropShadowX);

  const alignedX =
    param.align === TextAlign.left
      ? param.x
      : param.align === TextAlign.center
      ? param.x - (param.width / 2 - textWidth / 2)
      : textWidth - (param.width - param.x);

  return getOneClickSelection({
    text,
    font,
    x: alignedX,
    isShiftKeyPressed,
    lastSelection: param.selection,
  });
}

function getSelectionIndexOfX({
  font,
  text,
  x,
}: {
  text: string;
  font: Font;
  x: number;
}): number {
  const glyphIds = font.getGlyphIDs(text);
  const glyphWidths = font.getGlyphWidths(glyphIds);

  let left = 0;
  const index = glyphWidths.findIndex((width) => {
    const center = left + width / 2;
    if (x < center) {
      return true;
    }
    left += width;
    return false;
  });

  return index === -1 ? text.length : index;
}

function getMoreTripleClickSelection({ text }: { text: string }): Selection {
  return {
    start: 0,
    end: text.length,
  };
}

function getDoubleClickSelection({
  text,
  font,
  x,
}: {
  text: string;
  font: Font;
  x: number;
}): Selection {
  const selectionIndex = getSelectionIndexOfX({
    font,
    text,
    x,
  });

  const textBlockRange = getTextBlockIncludingIndex({
    text,
    index: selectionIndex,
  });

  return {
    start: textBlockRange.index,
    end: textBlockRange.index + textBlockRange.length,
  };
}
function getTextBlockIncludingIndex({
  text,
  index,
}: {
  text: string;
  index: number;
}): {
  index: number;
  length: number;
} {
  const characterOnIndex = text[index];
  const isInBlock = (character: string | undefined): boolean => {
    if (character === undefined) {
      return false;
    }
    return characterOnIndex === " " ? character === " " : character !== " ";
  };

  let left = index;
  while (left > 0 && isInBlock(text[left - 1])) {
    left--;
  }
  let right = index;
  while (right < text.length && isInBlock(text[right + 1])) {
    right++;
  }
  return {
    index: left,
    length: right - left + 1,
  };
}

function getOneClickSelection({
  text,
  font,
  x,
  isShiftKeyPressed,
  lastSelection,
}: {
  text: string;
  font: Font;
  x: number;
  isShiftKeyPressed: boolean;
  lastSelection: Selection | undefined;
}): Selection {
  const selectionIndexOfX = getSelectionIndexOfX({ font, text, x });

  const start =
    !lastSelection || !isShiftKeyPressed
      ? selectionIndexOfX
      : lastSelection.start;

  return {
    start,
    end: selectionIndexOfX,
  };
}
