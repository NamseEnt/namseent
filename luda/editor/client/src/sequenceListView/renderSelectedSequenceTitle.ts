import {
  ColorUtil,
  FontWeight,
  Language,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";

export const renderSelectedSequenceTitle: Render<
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
    text: "Selected: ",
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
    text: props.title || "Not selected",
  }),
];
