import {
  ColorUtil,
  FontWeight,
  Language,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  TextInput,
} from "namui";
import { TextInputState } from "../type";

export function renderNumberInput(
  props: {
    width: number;
    label: string;
    value: number;
    textInputId: string;
    onChange: (value: number) => void;
  },
  state: TextInputState,
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
      text: props.value.toString(),
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 12,
      },
      width: props.width - 150,
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
      focus: state.targetId === props.textInputId,
      onChange: ({ text, selection }) => {
        if (state.targetId !== props.textInputId) {
          return;
        }
        const parsed = parseFloat(text);
        props.onChange(isNaN(parsed) ? 0 : parsed);
        state.selection = selection;
      },
      onClick() {
        if (state.targetId !== props.textInputId) {
          state.targetId = props.textInputId;
          state.selection = undefined;
        }
      },
      onClickOut() {
        if (state.targetId === props.textInputId) {
          state.targetId = undefined;
          state.selection = undefined;
        }
      },
      selection:
        state.targetId === props.textInputId ? state.selection : undefined,
    }),
  ];
}
