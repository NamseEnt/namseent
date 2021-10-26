import {
  RenderingTree,
  TextAlign,
  TextBaseline,
  Language,
  FontWeight,
  ColorUtil,
  TextInput,
  Text,
} from "namui";
import { TextInputState } from "../type";

export function renderSingleTextEditorRow(
  props: {
    label: string;
    value: string;
    textInputId: string;
    onChange: (text: string) => void;
  },
  state: {
    textInput: TextInputState;
  },
): RenderingTree {
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
      text: props.label,
    }),
    TextInput({
      x: 150,
      y: 0,
      text: props.value,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 12,
      },
      width: 200,
      style: {
        color: ColorUtil.Black,
      },
      height: 20,
      rectStyle: {
        fill: {
          color: ColorUtil.White,
        },
        stroke: {
          color: ColorUtil.Black,
          width: 1,
        },
      },
      focus: state.textInput.targetId === props.textInputId,
      onChange: ({ text, selection }) => {
        if (state.textInput.targetId !== props.textInputId) {
          return;
        }
        props.onChange(text);
        state.textInput.selection = selection;
      },
      onClick() {
        if (state.textInput.targetId !== props.textInputId) {
          state.textInput.targetId = props.textInputId;
          state.textInput.selection = undefined;
        }
      },
      onClickOut() {
        if (state.textInput.targetId === props.textInputId) {
          state.textInput.targetId = undefined;
          state.textInput.selection = undefined;
        }
      },
      selection:
        state.textInput.targetId === props.textInputId
          ? state.textInput.selection
          : undefined,
    }),
  ];
}
