import {
  Render,
  ColorUtil,
  Text,
  TextAlign,
  TextBaseline,
  FontWeight,
  Language,
} from "namui";
import { TimeRulerProps } from "./type";

export const TimeTexts: Render<
  {},
  TimeRulerProps & {
    bigGradationXs: number[];
  }
> = (state, props) => {
  const leftMarginPx = 5;
  const textSize = 10;
  return props.bigGradationXs.map((x) => {
    const ms = Math.round(props.startMs + x * props.msPerPixel);

    const minutes = Math.floor(ms / 60000);
    const seconds = Math.floor((ms % 60000) / 1000);
    const milliseconds = Math.floor(ms % 1000);

    const MM = minutes.toString().padStart(2, "0");
    const ss = seconds.toString().padStart(2, "0");
    const mmm = milliseconds.toString().padStart(3, "0");

    return Text({
      x: x + leftMarginPx,
      y: props.layout.height / 2,
      align: TextAlign.left,
      baseline: TextBaseline.middle,
      fontType: {
        size: textSize,
        fontWeight: FontWeight.regular,
        language: Language.ko,
        serif: false,
      },
      style: {
        color: ColorUtil.Grayscale01(0.5),
      },
      text: `${MM}:${ss}:${mmm}`,
    });
  });
};
