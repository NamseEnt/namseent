import {
  Render,
  Rect,
  ColorUtil,
  BorderPosition,
  MouseButton,
  Clip,
} from "namui";
import { renderClip } from "./clip/renderClip";
import { refreshSubtitleClipPositions } from "./refreshClipPositions/refreshSubtitleClipPositions";
import { TimelineState, Track } from "./type";

export const renderSubtitleTrackBody: Render<
  { timelineState: TimelineState; track: Track },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { clips } = state.track;
  const selectedSubtitleClip = clips.find(
    (clip) => clip === state.timelineState.selectedClip,
  );

  refreshSubtitleClipPositions(state.timelineState, state.track);

  const draggingClip =
    state.timelineState.actionState?.type === "dragClip" &&
    clips.find((clip) => clip.id === state.timelineState.actionState?.clipId);

  const draggingOrSelectedClipLastClips = [
    ...clips.filter((clip) => clip !== draggingClip),
    ...(draggingClip ? [draggingClip] : []),
    ...(selectedSubtitleClip ? [selectedSubtitleClip] : []),
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

          const clipUnderMouse = state.timelineState.clipIdMouseIn
            ? clips.find(
                (clip) => clip.id === state.timelineState.clipIdMouseIn,
              )
            : undefined;

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
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(
            0,
            props.height / 3,
            props.width,
            props.height / 3,
          ),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      Rect({
        x: 0,
        y: props.height / 3,
        width: props.width,
        height: props.height / 3,
        style: {
          stroke: {
            color: ColorUtil.Color01(0.25, 0.25, 0.25),
            width: 1,
            borderPosition: BorderPosition.outside,
          },
        },
      }),
    ),
    draggingOrSelectedClipLastClips.map((clip) => {
      return renderClip(
        { timelineState: state.timelineState, clip },
        {
          height: props.height,
          maxRight: props.width,
        },
      );
    }),
  ];
};
