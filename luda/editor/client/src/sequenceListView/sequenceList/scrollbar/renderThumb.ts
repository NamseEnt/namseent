import { ColorUtil, Rect, Render } from "namui";

export const renderThumb: Render<
  {},
  { width: number; height: number; y: number }
> = (state, props) => {
  const { y, width, height } = props;
  return Rect({
    x: 0,
    y,
    width,
    height,
    style: {
      fill: {
        color: ColorUtil.Grayscale01(0.8),
      },
      round: {
        radius: width / 2,
      },
    },
  });
};
