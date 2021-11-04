import { ColorUtil, MouseButton, Rect, Render } from "namui";
import { ClipComponent } from "../clip/ClipComponent";
import { Clip, TimelineState, Track } from "../type";
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

  // this should be called before pushClipsForward.
  const draggingFakeClip = DraggingFakeClip(state, {
    clips,
    width: props.width,
    height: props.height,
  });

  pushClipsForward(state.timelineState, clips);

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
      if (
        state.timelineState.actionState?.type === "dragClip" &&
        clip.id === state.timelineState.actionState.clipId
      ) {
        return;
      }
      return ClipComponent(
        { timelineState: state.timelineState, clip },
        {
          height: props.height,
          maxRight: props.width,
          sashComponent: CameraTrackSash,
        },
      );
    }),
    draggingFakeClip,
  ];
};

function pushClipsForward(state: TimelineState, clips: Clip[]) {
  clips.sort((a, b) => a.startMs - b.startMs);

  if (state.actionState?.type === "dragClip") {
    const { clipId } = state.actionState;
    const draggingClip = clips.find((clip) => clip.id === clipId);
    if (!draggingClip) {
      throw new Error("clip not found");
    }

    const draggingClipIndex = clips.indexOf(draggingClip);
    let changingClipIndex = draggingClipIndex;
    for (let index = 0; index < clips.length; index++) {
      if (index === draggingClipIndex) {
        continue;
      }
      const clip = clips[index]!;
      const clipCenterMs = (clip.startMs + clip.endMs) / 2;
      if (index < draggingClipIndex) {
        if (draggingClip.startMs < clipCenterMs) {
          changingClipIndex = index;
          break;
        }
      } else {
        if (clipCenterMs < draggingClip.endMs) {
          changingClipIndex = index;
        }
      }
    }
    const temp = clips[draggingClipIndex]!;
    clips[draggingClipIndex] = clips[changingClipIndex]!;
    clips[changingClipIndex] = temp;
  }

  const firstClip = clips[0];
  if (firstClip) {
    const duration = firstClip.endMs - firstClip.startMs;
    firstClip.startMs = 0;
    firstClip.endMs = duration;
  }
  for (let index = 0; index < clips.length; index++) {
    const clip = clips[index]!;
    const nextClip = clips[index + 1];
    if (!nextClip) {
      continue;
    }

    const nextClipDurationMs = nextClip.endMs - nextClip.startMs;

    nextClip.startMs = clip.endMs;
    nextClip.endMs = nextClip.startMs + nextClipDurationMs;
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
  const clip = props.clips.find((clip) => clip.id === clipId);
  if (!clip) {
    throw new Error("clip not found");
  }
  return ClipComponent(
    {
      timelineState: state.timelineState,
      clip: {
        startMs: clip.startMs,
        endMs: clip.endMs,
        id: "camera-track-drag-preview",
      },
    },
    {
      height: props.height,
      maxRight: props.width,
      sashComponent: CameraTrackSash,
    },
  );
};
