import { ColorUtil, Rect, RenderingTree } from "namui";
import { Track } from "./type";

export function renderTrackHeader(props: {
  width: number;
  height: number;
  track: Track;
}): RenderingTree {
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
        },
      },
    }),
  ];
}
