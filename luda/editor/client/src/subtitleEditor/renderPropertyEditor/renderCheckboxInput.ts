import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";

export function renderCheckboxInput(props: {
  value: boolean;
  label: string;
  onChange: (value: boolean) => void;
}): RenderingTree {
  const strokeWidth = 1;
  const innerMargin = 4;

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
    Rect({
      x: 150 + strokeWidth,
      y: 0 + strokeWidth,
      width: 20 - 2 * strokeWidth,
      height: 20 - 2 * strokeWidth,
      style: {
        stroke: {
          width: strokeWidth,
          color: ColorUtil.Black,
        },
      },
      onClick: () => {
        props.onChange(!props.value);
      },
    }),
    Rect({
      x: 150 + innerMargin + strokeWidth,
      y: innerMargin + strokeWidth,
      width: 20 - 2 * (innerMargin + strokeWidth),
      height: 20 - 2 * (innerMargin + strokeWidth),
      style: {
        fill: {
          color: props.value ? ColorUtil.Black : ColorUtil.White,
        },
      },
    }),
  ];
}
