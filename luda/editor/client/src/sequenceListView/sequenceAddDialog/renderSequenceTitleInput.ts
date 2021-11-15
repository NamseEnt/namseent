import {
  BorderPosition,
  ColorUtil,
  FontWeight,
  Language,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  TextInput,
} from "namui";
import { SequenceListViewState } from "../type";

export const renderSequenceTitleInput: Render<
  SequenceListViewState,
  {
    width: number;
  }
> = (state, props) => {
  const { textInput } = state;
  const { width } = props;

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
        size: 20,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: "Title: ",
    }),
    TextInput({
      x: 160,
      y: 0,
      text: state.newTitle,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 20,
      },
      width: width - 160,
      style: {
        color: ColorUtil.Black,
      },
      height: 32,
      rectStyle: {
        fill: {
          color: ColorUtil.White,
        },
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
      focus: textInput.focus,
      onChange: ({ text, selection }) => {
        state.newTitle = text;
        textInput.selection = selection;
      },
      onClick() {
        if (!textInput.focus) {
          textInput.focus = true;
          textInput.selection = undefined;
        }
      },
      onClickOut() {
        if (textInput.focus) {
          textInput.focus = false;
          textInput.selection = undefined;
        }
      },
      selection: textInput.selection,
    }),
  ];
};
