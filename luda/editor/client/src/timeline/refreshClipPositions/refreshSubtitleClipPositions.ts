import { Clip } from "../../type";
import { TimelineState, Track } from "../type";

export function refreshSubtitleClipPositions(
  timelineState: TimelineState,
  track: Track,
) {
  constrainClipsPlacement(timelineState, track.clips);
}

const constraintMs = 200;

function constrainClipsPlacement(state: TimelineState, clips: Clip[]) {
  clips.sort((a, b) => a.startMs - b.startMs);

  const draggingClip = clips.find(
    (clip) =>
      state.actionState?.type === "dragClip" &&
      clip.id === state.actionState.clipId,
  );

  constrainNotDraggingClipsPlacement(clips, draggingClip);
  if (draggingClip) {
    constrainDraggingClipPlacement(clips, draggingClip);
  }
}

function constrainNotDraggingClipsPlacement(
  clips: Clip[],
  draggingClip: Clip | undefined,
): void {
  clips.forEach((clip, index) => {
    const nextClip = clips[index + 1];
    if (!nextClip || clip === draggingClip || nextClip === draggingClip) {
      return;
    }

    if (clip.startMs + constraintMs <= nextClip.startMs) {
      return;
    }
    const durationMs = nextClip.endMs - nextClip.startMs;
    nextClip.startMs = clip.startMs + constraintMs;

    nextClip.endMs = nextClip.startMs + durationMs;
  });
}
function constrainDraggingClipPlacement(
  clips: Clip[],
  draggingClip: Clip,
): void {
  const clipsWithoutDraggingClip = clips.filter(
    (clip) => clip.id !== draggingClip.id,
  );
  const isOkNow = canPutClipAtStartMs(
    clipsWithoutDraggingClip,
    draggingClip.startMs,
  );
  if (isOkNow) {
    return;
  }

  clipsWithoutDraggingClip.sort(
    (a, b) =>
      Math.abs(a.startMs - draggingClip.startMs) -
      Math.abs(b.startMs - draggingClip.startMs),
  );

  for (const clip of clipsWithoutDraggingClip) {
    const otherClips = clipsWithoutDraggingClip.filter((x) => x.id !== clip.id);
    const leftAvailableStartMs = clip.startMs - constraintMs;
    const rightAvailableStartMs = clip.startMs + constraintMs;
    const startMsList = [leftAvailableStartMs, rightAvailableStartMs];
    startMsList.sort(
      (a, b) =>
        Math.abs(a - draggingClip.startMs) - Math.abs(b - draggingClip.startMs),
    );

    for (const startMs of startMsList) {
      if (canPutClipAtStartMs(otherClips, startMs)) {
        const durationMs = draggingClip.endMs - draggingClip.startMs;
        draggingClip.startMs = startMs;
        draggingClip.endMs = draggingClip.startMs + durationMs;
        return;
      }
    }
  }

  throw new Error("you should not come here.");
}

function canPutClipAtStartMs(clips: Clip[], startMs: number): boolean {
  return clips.every(
    (clip) => Math.abs(clip.startMs - startMs) >= constraintMs,
  );
}
