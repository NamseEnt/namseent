import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";

export const renderSequenceName: Render<
  {},
  {
    sequenceName?: string;
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
    text: "SequenceName: ",
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
      color: props.sequenceName ? ColorUtil.Black : ColorUtil.Red,
    },
    text:
      props.sequenceName ||
      "No name has been specified. Changes will not be saved.",
  }),
];
