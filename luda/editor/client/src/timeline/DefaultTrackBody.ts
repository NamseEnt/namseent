import { ColorUtil, MouseButton, Rect, Render, BorderPosition } from "namui";
import { Track, TimelineState } from "./type";
import { ClipComponent } from "./clip/ClipComponent";
import { Sash } from "./clip/Sash";
import { Clip } from "../type";

export const DefaultTrackBody: Render<
  TimelineState,
  {
    width: number;
    height: number;
    track: Track;
  }
> = (state, props) => {
  const { clips } = props.track;
  let selectedClip: Clip | undefined = undefined;

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
            state.layout.startMs + event.translated.x * state.layout.msPerPixel;
          state.contextMenu = {
            type: "trackBody",
            clickMs,
            x: event.x,
            y: event.y,
            trackId: props.track.id,
          };
        }
      },
    }),
    Rect({
      x: 0,
      y: props.height / 3,
      width: props.width,
      height: props.height / 3,
      style: {
        stroke: {
          color: ColorUtil.Color01(0.25, 0.25, 0.25),
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
    Rect({
      x: 0,
      y: 0,
      width: props.width,
      height: props.height,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
    clips.map((clip) => {
      return ClipComponent(
        { timelineState: state, clip },
        { height: props.height, maxRight: props.width, sashComponent: Sash },
      );
    }),
    selectedClip
      ? ClipComponent(
          { timelineState: state, clip: selectedClip },
          { height: props.height, maxRight: props.width, sashComponent: Sash },
        )
      : undefined,
  ];
};
