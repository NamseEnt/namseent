import {
  Render,
  TextAlign,
  TextBaseline,
  FontWeight,
  Language,
  ColorUtil,
  Text,
} from "namui";

export const renderTitleHeader: Render<
  {},
  {
    centerX: number;
    centerY: number;
  }
> = (state, props) => {
  return Text({
    x: props.centerX,
    y: props.centerY,
    align: TextAlign.center,
    baseline: TextBaseline.middle,
    fontType: {
      fontWeight: FontWeight.bold,
      language: Language.ko,
      serif: false,
      size: 20,
    },
    style: {
      color: ColorUtil.Black,
    },
    text: "[Live Player]",
  });
};
