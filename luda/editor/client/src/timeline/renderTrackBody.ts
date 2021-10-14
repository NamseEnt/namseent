import { ColorUtil, Rect, RenderingTree } from "namui";
import { Track, TimelineState } from "./type";
import { renderClip } from "./renderClip";

export function renderTrackBody(
  props: {
    width: number;
    height: number;
    track: Track;
  },
  state: TimelineState,
): RenderingTree {
  const { clips } = props.track;
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
    clips.map((clip) => {
      return renderClip(
        { height: props.height, maxRight: props.width },
        { timelineState: state, clipState: clip },
      );
    }),
  ];
}
