import { Clip } from "../../type";
import { TimelineState, Track } from "../type";

export function refreshCameraClipPositions(
  timelineState: TimelineState,
  track: Track,
) {
  pushClipsForward(timelineState, track.clips);
}

function pushClipsForward(state: TimelineState, clips: Clip[]) {
  clips.sort((a, b) => a.startMs - b.startMs);

  const draggingClip =
    state.actionState?.type === "dragClip" &&
    clips.find((clip) => clip.id === state.actionState?.clipId);
  if (draggingClip) {
    changeOrderByDragging(clips, draggingClip);
  }

  let nextStartMs = 0;
  for (let index = 0; index < clips.length; index++) {
    const clip = clips[index];
    if (!clip) {
      continue;
    }

    const durationMs = clip.endMs - clip.startMs;

    if (clip !== draggingClip) {
      clip.startMs = nextStartMs;
      clip.endMs = clip.startMs + durationMs;
    }
    nextStartMs = nextStartMs + durationMs;
  }
}

function changeOrderByDragging(clips: Clip[], draggingClip: Clip) {
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
