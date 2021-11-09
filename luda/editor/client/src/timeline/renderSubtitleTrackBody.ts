import {
  Render,
  Rect,
  ColorUtil,
  BorderPosition,
  MouseButton,
  Mathu,
} from "namui";
import { Clip } from "../type";
import { renderClip } from "./clip/renderClip";
import { TimelineState, Track } from "./type";

export const renderSubtitleTrackBody: Render<
  { timelineState: TimelineState; track: Track },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { clips } = state.track;

  // this should be called before constrainDraggingClipPlacement.
  const draggingFakeClip = DraggingFakeClip(state, {
    clips,
    width: props.width,
    height: props.height,
  });

  constrainDraggingClipPlacement(state.timelineState, clips);

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
      if (
        state.timelineState.actionState?.type === "dragClip" &&
        clip.id === state.timelineState.actionState.clipId
      ) {
        return;
      }
      return renderClip(
        { timelineState: state.timelineState, clip },
        {
          height: props.height,
          maxRight: props.width,
        },
      );
    }),
    draggingFakeClip,
  ];
};

function constrainDraggingClipPlacement(state: TimelineState, clips: Clip[]) {
  if (state.actionState?.type !== "dragClip") {
    return;
  }
  const draggingClipIndex = clips.findIndex(
    (clip) => clip.id === state.actionState?.clipId,
  );
  if (draggingClipIndex < 0) {
    return;
  }
  const draggingClip = clips[draggingClipIndex]!;

  clips.sort((a, b) => a.startMs - b.startMs);

  let previousClipIndex = draggingClipIndex - 1;
  let nextClipIndex = draggingClipIndex + 1;
  while (true) {
    const previousClip = clips[previousClipIndex];
    const nextClip = clips[nextClipIndex];

    const previousAvailablePoint = previousClip
      ? previousClip.startMs + 200
      : 0;
    const nextAvailablePoint = nextClip ? nextClip.startMs - 200 : Infinity;
    const availableSpace = nextAvailablePoint - previousAvailablePoint;
    if (availableSpace < 0) {
      [previousClipIndex, nextClipIndex] = [nextClipIndex, nextClipIndex + 1];
      continue;
    }

    const newStartMs = Mathu.clamp(
      draggingClip.startMs,
      previousAvailablePoint,
      nextAvailablePoint,
    );
    const offset = newStartMs - draggingClip.startMs;
    draggingClip.startMs += offset;
    draggingClip.endMs += offset;
    break;
  }
}

const DraggingFakeClip: Render<
  {
    timelineState: TimelineState;
  },
  { clips: Clip[]; width: number; height: number }
> = (state, props) => {
  if (state.timelineState.actionState?.type !== "dragClip") {
    return;
  }
  const { clipId } = state.timelineState.actionState;
  const draggingClip = props.clips.find((clip) => clip.id === clipId);
  if (!draggingClip) {
    return;
  }
  return renderClip(
    {
      timelineState: state.timelineState,
      clip: {
        ...draggingClip,
        startMs: draggingClip.startMs,
        endMs: draggingClip.endMs,
        id: `fake-${draggingClip.id}-drag-preview`,
      },
    },
    {
      height: props.height,
      maxRight: props.width,
    },
  );
};
