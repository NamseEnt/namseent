import { ColorUtil, Rect, BorderPosition, Render } from "namui";
import { Track } from "./type";

export const renderTrackHeader: Render<
  {},
  {
    width: number;
    height: number;
    track: Track;
  }
> = (state, props) => {
  return [
    Rect({
      x: 0,
      y: 0,
      width: props.width,
      height: props.height,
      style: {
        fill: {
          color: ColorUtil.Color01(0.4, 0.4, 0.4),
        },
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
  ];
};
