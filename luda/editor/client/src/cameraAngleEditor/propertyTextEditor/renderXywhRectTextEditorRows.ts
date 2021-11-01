import {
  ColorUtil,
  FontWeight,
  Language,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  TextInput,
  XywhRect,
} from "namui";
import { TextInputState } from "../type";
import { Row } from "./renderPropertyTextEditor";
import { renderSingleTextEditorRow } from "./renderSingleTextEditorRow";

export function renderXywhRectTextEditorRows(
  props: {
    label: string;
  },
  state: {
    rect: XywhRect;
    textInput: TextInputState;
  },
): Row[] {
  const rectKeys = ["x", "y", "width", "height"] as const;
  return [
    Text({
      x: 0,
      y: 0,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 12,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: `- ${props.label}`,
    }),
    ...rectKeys.map((key) => {
      return renderXywhRectTextEditorRow(
        {
          key,
          textInputId: `${props.label}.${key}`,
        },
        {
          rect: state.rect,
          textInput: state.textInput,
        },
      );
    }),
  ];
}

export function renderXywhRectTextEditorRow(
  props: {
    key: keyof XywhRect;
    textInputId: string;
  },
  state: {
    rect: XywhRect;
    textInput: TextInputState;
  },
): RenderingTree {
  return renderSingleTextEditorRow(
    {
      label: `  - ${props.key}`,
      value: state.rect[props.key].toString(),
      textInputId: props.textInputId,
      onChange: (text) => {
        // TODO : Do not update data before finish editing
        const value = parseFloat(text) || 0;
        state.rect[props.key] = value;
      },
    },
    {
      textInput: state.textInput,
    },
  );
}
