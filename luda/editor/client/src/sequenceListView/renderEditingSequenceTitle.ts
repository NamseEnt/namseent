import {
  ColorUtil,
  FontWeight,
  Language,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";

export const renderEditingSequenceTitle: Render<
  {},
  {
    title?: string;
  }
> = (state, props) => [
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
    text: "Now Editing: ",
  }),
  Text({
    x: 160,
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
      color: props.title ? ColorUtil.Black : ColorUtil.Red,
    },
    text:
      props.title || "No name has been specified. Changes will not be saved.",
  }),
];
