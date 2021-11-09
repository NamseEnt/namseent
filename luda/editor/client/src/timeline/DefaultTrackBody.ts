import { ColorUtil, MouseButton, Rect, Render, BorderPosition } from "namui";
import { Track, TimelineState } from "./type";
import { ClipComponent } from "./clip/ClipComponent";
import { Sash } from "./clip/Sash";

export const DefaultTrackBody: Render<
  {
    timelineState: TimelineState;
    track: Track;
  },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { clips } = state.track;

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
      onMouseUp(event) {
        if (event.button === MouseButton.right) {
          const clickMs =
            state.timelineState.layout.startMs +
            event.translated.x * state.timelineState.layout.msPerPixel;
          state.timelineState.contextMenu = {
            type: "trackBody",
            clickMs,
            x: event.x,
            y: event.y,
            trackId: state.track.id,
          };
        }
      },
    }),
    clips.map((clip) => {
      return ClipComponent(
        { timelineState: state.timelineState, clip },
        { height: props.height, maxRight: props.width, sashComponent: Sash },
      );
    }),
  ];
};
