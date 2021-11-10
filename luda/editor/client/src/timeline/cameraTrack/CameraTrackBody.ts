import {
  BorderPosition,
  ColorUtil,
  Mathu,
  MouseButton,
  Rect,
  Render,
} from "namui";
import { ClipComponent } from "../clip/ClipComponent";
import { refreshCameraClipPositions } from "../refreshClipPositions/refreshCameraClipPositions";
import { TimelineState, Track } from "../type";
import { CameraTrackSash } from "./CameraTrackSash";

export const CameraTrackBody: Render<
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

  refreshCameraClipPositions(state.timelineState, state.track);

  const draggingClip =
    state.timelineState.actionState?.type === "dragClip" &&
    clips.find((clip) => clip.id === state.timelineState.actionState?.clipId);

  const draggingClipLastClips = [
    ...clips.filter((clip) => clip !== draggingClip),
    ...(draggingClip ? [draggingClip] : []),
  ];

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

          const clipUnderMouse = clips.find((clip) => {
            const clipStartMs = clip.startMs;
            const clipEndMs = clip.endMs;
            return (
              Mathu.in(clickMs, clipStartMs, clipEndMs) &&
              !clip.id.startsWith("fake")
            );
          });

          if (clipUnderMouse) {
            state.timelineState.contextMenu = {
              type: "clip",
              x: event.x,
              y: event.y,
              clipId: clipUnderMouse.id,
              trackId: state.track.id,
            };
          } else {
            state.timelineState.contextMenu = {
              type: "trackBody",
              clickMs,
              x: event.x,
              y: event.y,
              trackId: state.track.id,
            };
          }
        }
      },
    }),
    draggingClipLastClips.map((clip) => {
      return ClipComponent(
        { timelineState: state.timelineState, clip },
        {
          height: props.height,
          maxRight: props.width,
          sashComponent: CameraTrackSash,
        },
      );
    }),
  ];
};
