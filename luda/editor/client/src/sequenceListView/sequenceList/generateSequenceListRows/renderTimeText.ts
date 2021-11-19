import {
  ColorUtil,
  FontWeight,
  Language,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";

export const renderTimeText: Render<
  {},
  {
    width: number;
    height: number;
    timeMs: number;
  }
> = (state, props) => {
  const { width, height, timeMs } = props;

  const sec = timeMs / 1000;
  const min = sec / 60;
  const hour = min / 60;

  const hh = Math.floor(hour);
  const mm = (Math.floor(min) % 60).toString().padStart(2, "0");
  const ss = (Math.floor(sec) % 60).toString().padStart(2, "0");

  return Text({
    x: width,
    y: height / 2,
    align: TextAlign.right,
    baseline: TextBaseline.middle,
    fontType: {
      language: Language.ko,
      serif: false,
      fontWeight: FontWeight.regular,
      size: 14,
    },
    style: {
      color: ColorUtil.White,
    },
    text: hh ? `${hh}:${mm}:${ss}` : `${mm}:${ss}`,
  });
};
