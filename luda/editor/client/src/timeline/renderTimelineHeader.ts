import {
  ColorUtil,
  Rect,
  RenderingTree,
  Translate,
  BorderPosition,
} from "namui";
import { Track } from "./type";
import { renderTrackHeader } from "./renderTrackHeader";

export function renderTimelineHeader(props: {
  width: number;
  height: number;
  tracks: Track[];
}): RenderingTree {
  const trackHeaderHeight = 80;
  const trackHeaders = props.tracks.map((track, index) => {
    const x = 0;
    const y = trackHeaderHeight * index;
    const width = props.width;
    const height = trackHeaderHeight;
    return Translate({ x, y }, renderTrackHeader({ width, height, track }));
  });
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
    trackHeaders,
  ];
}
