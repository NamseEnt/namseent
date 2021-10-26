import { getTextWidth, Text, TextParam } from "../Text";
import { RenderingTree, TextAlign } from "../../type";
import { Selection } from "../../textInput/ITextInputManager";
import { ColorUtil } from "../..";

export function drawTextsDividedBySelection(
  param: {
    selection?: Selection;
    width: number;
  } & TextParam,
): RenderingTree {
  if (!param.selection || param.selection.start === param.selection.end) {
    return Text({
      ...param,
      x:
        param.align === TextAlign.left
          ? 0
          : param.align === TextAlign.center
          ? param.width / 2
          : param.width,
    });
  }
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
  const { leftTextX, selectedTextX, rightTextX } = getTextXs(param, {
    leftTextString,
    rightTextString,
    selectedTextString,
  });

  const leftText = Text({
    ...param,
    text: leftTextString,
    x: leftTextX,
  });
  const selectedText = Text({
    ...param,
    text: selectedTextString,
    x: selectedTextX,
    style: {
      color: ColorUtil.White,
    },
  });
  const rightText = Text({
    ...param,
    text: rightTextString,
    x: rightTextX,
  });

  return [leftText, selectedText, rightText];
}
function getTextXs(
  param: { selection?: Selection | undefined; width: number } & TextParam,
  {
    leftTextString,
    selectedTextString,
    rightTextString,
  }: {
    leftTextString: string;
    selectedTextString: string;
    rightTextString: string;
  },
): {
  leftTextX: number;
  selectedTextX: number;
  rightTextX: number;
} {
  const font = fontStorage.getFont(param.fontType);
  const leftTextWidth = getTextWidth(
    font,
    leftTextString,
    param.style.dropShadow?.x,
  );
  const selectedTextWidth = getTextWidth(
    font,
    selectedTextString,
    param.style.dropShadow?.x,
  );
  const rightTextWidth = getTextWidth(
    font,
    rightTextString,
    param.style.dropShadow?.x,
  );

  switch (param.align) {
    case TextAlign.left:
      return {
        leftTextX: 0,
        selectedTextX: leftTextWidth,
        rightTextX: leftTextWidth + selectedTextWidth,
      };
    case TextAlign.center:
      const center = param.width / 2;
      const totalWidth = leftTextWidth + selectedTextWidth + rightTextWidth;

      return {
        leftTextX: center - totalWidth / 2 + leftTextWidth / 2,
        selectedTextX:
          center - totalWidth / 2 + leftTextWidth + selectedTextWidth / 2,
        rightTextX: center + totalWidth / 2 - rightTextWidth / 2,
      };
    case TextAlign.right:
      return {
        rightTextX: param.width,
        selectedTextX: param.width - rightTextWidth,
        leftTextX: param.width - rightTextWidth - selectedTextWidth,
      };
  }
}
