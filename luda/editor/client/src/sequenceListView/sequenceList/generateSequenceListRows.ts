import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";
import { renderRows } from "../../common/renderRows";

export function generateSequenceListRows(props: {
  sequenceTitles: string[];
  width: number;
}): Parameters<typeof renderRows>[0] {
  const { sequenceTitles, width } = props;

  return sequenceTitles.map((title) => ({
    height: 128,
    renderingData: [
      Rect({
        x: 0,
        y: 0,
        width,
        height: 128,
        style: {
          fill: {
            color: ColorUtil.Grayscale01(0.3),
          },
        },
      }),
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
          color: ColorUtil.White,
        },
        text: title,
      }),
    ],
  }));
}
