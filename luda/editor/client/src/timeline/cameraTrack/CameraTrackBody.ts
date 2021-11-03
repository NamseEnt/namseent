import { ColorUtil, MouseButton, Rect, Render } from "namui";
import { renderClip } from "../renderClip";
import { Clip, TimelineState, Track } from "../type";

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
  pushClipsForward(clips);

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
      return renderClip(
        { height: props.height, maxRight: props.width },
        { timelineState: state.timelineState, clipState: clip },
      );
    }),
  ];
};

function pushClipsForward(clips: Clip[]) {
  clips.sort((a, b) => {
    return a.startMs - b.startMs;
  });

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
