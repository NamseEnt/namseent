import { Color, Font } from "canvaskit-wasm";
import { getTextWidth, Text, TextParam } from "./Text";
import {
  MouseEventCallback,
  MouseEvent,
  RenderingTree,
  TextAlign,
  TextBaseline,
} from "../type";
import { Rect } from "./Rect";
import {
  OnTextInputChange,
  Selection,
} from "../textInput/ITextInputController";
import { ColorUtil, engine, Translate } from "..";
import { FontType } from "../font/FontStorage";
import { Code } from "../device/keyboard/Code";

export function TextInput(param: {
  text: string;
  focus: boolean;
  selection: Selection | undefined;
  x: number;
  y: number;
  width: number;
  height: number;
  onClick: MouseEventCallback;
  onClickOut: MouseEventCallback;
  onChange: OnTextInputChange;
  align: TextAlign;
  baseline: TextBaseline;
  fontType: FontType;
  rectStyle: Parameters<typeof Rect>[0]["style"];
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
}): RenderingTree {
  console.log("param - ", param.text, param.selection);
  // TODO : Update selection by param.
  if (param.focus) {
    globalThis.textInputController.setFocus(param.text, param.onChange);
    globalThis.textInputController.updateSelection(param.selection);
  } else {
    globalThis.textInputController.outFocus();
  }

  const onClick = (event: MouseEvent) => {
    const localX = event.x - param.x; // TODO : should make it work on TextAlign.center and TextAlign.right too.
    const selection = getSelectionOnClick({
      x: localX,
      align: param.align,
      selection: param.selection,
      fontType: param.fontType,
      text: param.text,
    });
    param.onChange({
      text: param.text,
      selection,
    });
    param.onClick(event);
  };

  return Translate({ x: param.x, y: param.y }, [
    drawSelection({
      ...param,
    }),
    drawTextsDividedBySelection({
      ...param,
      x: 0,
      y: 0,
    }),
    Rect({
      x: 0,
      y: 0,
      width: param.width,
      height: param.height,
      style: param.rectStyle,
      onClick,
      onClickOut: param.onClickOut,
    }),
  ]);
}

function drawTextsDividedBySelection(
  param: {
    selection?: Selection;
  } & TextParam,
): RenderingTree {
  if (!param.selection || param.selection.start === param.selection.end) {
    return Text(param);
  }
  const font = fontStorage.getFont(param.fontType);
  const leftSelectionIndex = Math.min(
    param.selection.start,
    param.selection.end,
  );
  const rightSelectionIndex = Math.max(
    param.selection.start,
    param.selection.end,
  );

  const leftTextString = param.text.slice(0, leftSelectionIndex);
  const selectedTextString = param.text.slice(
    leftSelectionIndex,
    rightSelectionIndex,
  );
  const rightTextString = param.text.slice(rightSelectionIndex);

  const leftText = Text({
    ...param,
    text: leftTextString,
    x: 0,
  });
  const leftTextWidth = getTextWidth(
    font,
    leftTextString,
    param.style.dropShadow?.x,
  );
  const selectedText = Text({
    ...param,
    text: selectedTextString,
    x: leftTextWidth,
    style: {
      color: ColorUtil.White,
    },
  });
  const selectedTextWidth = getTextWidth(
    font,
    selectedTextString,
    param.style.dropShadow?.x,
  );
  const rightText = Text({
    ...param,
    text: rightTextString,
    x: leftTextWidth + selectedTextWidth,
  });

  return [leftText, selectedText, rightText];
}

function drawSelection(param: {
  text: string;
  selection?: Selection;
  align: TextAlign;
  baseline: TextBaseline;
  fontType: FontType;
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
  const x = getCaretX({
    selectionIndex: minSelectionIndex,
    font,
    text: param.text,
  });
  const width = shouldDrawSelectionAsLine
    ? 1
    : getCaretX({
        selectionIndex: maxSelectionIndex,
        font,
        text: param.text,
      }) - x;

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
function getSelectionOnClick(param: {
  text: string;
  selection: Selection | undefined;
  x: number;
  align: TextAlign;
  fontType: FontType;
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
  return getOneClickSelection({
    text,
    font,
    x: param.x,
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
