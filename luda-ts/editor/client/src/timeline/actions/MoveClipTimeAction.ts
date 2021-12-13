import { Draft } from "history";
import { Result } from "types";
import { Clip } from "../../type";
import { TimelineHistoryState, Track } from "../type";

export type MoveClipTimeAction = {
  targetClipId?: string;
  targetStartTimeMs?: number;
};

export enum MoveClipTimeActionError {
  notCompleted = "notCompleted",
  clipNotFound = "clipNotFound",
}

export function updateStateMoveClipTimeAction(
  state: Draft<TimelineHistoryState>,
  action: MoveClipTimeAction,
): Result<void, MoveClipTimeActionError> {
  if (
    action.targetClipId === undefined ||
    action.targetStartTimeMs === undefined
  ) {
    console.error(
      "[MoveClipTimeAction] targetClipId or targetStartTimeMs is undefined",
      action,
    );
    return {
      isSuccessful: false,
      error: MoveClipTimeActionError.notCompleted,
    };
  }

  const targetClip = findClip(state.tracks, action.targetClipId);
  if (targetClip === undefined) {
    return {
      isSuccessful: false,
      error: MoveClipTimeActionError.clipNotFound,
    };
  }

  const duration = targetClip.endMs - targetClip.startMs;
  targetClip.startMs = action.targetStartTimeMs;
  targetClip.endMs = targetClip.startMs + duration;

  return {
    isSuccessful: true,
  };
}

function findClip(tracks: Track[], clipId: string): Clip | undefined {
  for (const track of tracks) {
    for (const clip of track.clips) {
      if (clip.id === clipId) {
        return clip;
      }
    }
  }
}
