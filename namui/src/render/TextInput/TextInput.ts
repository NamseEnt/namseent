import { Color, Font } from "canvaskit-wasm";
import {
  MouseEventCallback,
  MouseEvent,
  RenderingTree,
  TextAlign,
  TextBaseline,
} from "../../type";
import { Rect } from "../Rect";
import {
  OnTextInputChange,
  Selection,
} from "../../textInput/ITextInputController";
import { engine, Translate } from "../..";
import { FontType } from "../../font/FontStorage";
import { drawTextsDividedBySelection } from "./drawTextsDividedBySelection";
import { getSelectionOnClick } from "./getSelection";
import { drawSelection } from "./drawSelection";

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
  // TODO : Update selection by param.
  if (param.focus) {
    globalThis.textInputController.setFocus(param.text, param.onChange);
    globalThis.textInputController.updateSelection(param.selection);
  } else {
    globalThis.textInputController.outFocus();
  }

  const onClick = (event: MouseEvent) => {
    const selection = getSelectionOnClick({
      x: event.translated.x,
      align: param.align,
      selection: param.selection,
      fontType: param.fontType,
      text: param.text,
      width: param.width,
      dropShadowX: param.style.dropShadow?.x,
    });
    param.onChange({
      text: param.text,
      selection,
    });
    param.onClick(event);
  };

  return Translate({ x: param.x, y: param.y }, [
    Rect({
      x: 0,
      y: 0,
      width: param.width,
      height: param.height,
      style: param.rectStyle,
      onClick,
      onClickOut: param.onClickOut,
    }),
    drawSelection({
      ...param,
      dropShadowX: param.style.dropShadow?.x,
    }),
    drawTextsDividedBySelection({
      ...param,
      x: 0,
      y: 0,
    }),
  ]);
}
