import { Mathu, Render, WhSize } from "namui";
import { isSubtitleClip } from "../../../clipTypeGuard";
import { Track } from "../../../timeline/type";
import { Clip } from "../../../type";
import { getPlaybackTimeMs } from "../../operations/getPlaybackTimeMs";
import { LivePlayerState } from "../../type";
import { Subtitle } from "./Subtitle";

export const SubtitleTrack: Render<
  {},
  {
    track: Track;
    livePlayerState: LivePlayerState;
    screenWhSize: WhSize;
  }
> = (state, props) => {
  const playbackTimeMs = getPlaybackTimeMs(props.livePlayerState);
  const clipsInPlaybackTime = props.track.clips.filter((clip) =>
    Mathu.in(playbackTimeMs, clip.startMs, clip.endMs),
  );

  clipsInPlaybackTime.sort((a, b) => a.startMs - b.startMs);

  return clipsInPlaybackTime.map((clip) => {
    if (!isSubtitleClip(clip)) {
      throw new Error("clip is not subtitle clip");
    }
    const lineIndex = getLineIndexPushUp(
      props.track.clips,
      playbackTimeMs,
      clip,
    );

    return Subtitle(
      {},
      {
        subtitle: clip.subtitle,
        whSize: props.screenWhSize,
        lineIndex,
      },
    );
  });
};

// NOTE : Below functions are in research.
// https://docs.google.com/document/d/1MBLlg_g72LxW5TTknX-AX3CVlLwKjYsH-Yj0UBO9VVk/edit#

function getLineIndexPushUp(
  clips: ReadonlyArray<Clip>,
  playbackTimeMs: number,
  targetClip: Clip,
): number {
  const clipsComeAfterTarget = clips.filter(
    (clip) => targetClip.startMs < clip.startMs,
  );
  clipsComeAfterTarget.sort((a, b) => a.startMs - b.startMs);

  let lineIndex = 0;
  for (const clip of clipsComeAfterTarget) {
    if (playbackTimeMs < clip.startMs) {
      break;
    }
    const numberOfClipsAtStart = clipsComeAfterTarget.reduce((prev, clipA) => {
      return (
        prev + (Mathu.in(clip.startMs, clipA.startMs, clipA.endMs) ? 1 : 0)
      );
    }, 0);

    lineIndex = Math.max(lineIndex, numberOfClipsAtStart);
  }

  return lineIndex;
}

function getLineIndexKeepingSlot(
  clips: ReadonlyArray<Clip>,
  playbackTimeMs: number,
  targetClip: Clip,
): number {
  type ClipTime = {
    ms: number;
    timeType: "start" | "end";
    clip: Clip;
  };
  const clipTimes: ClipTime[] = clips
    .map((clip) => {
      return [
        {
          ms: clip.startMs,
          timeType: "start",
          clip,
        },
        {
          ms: clip.endMs,
          timeType: "end",
          clip,
        },
      ] as const;
    })
    .flat()
    .filter((time) => time.ms <= playbackTimeMs);

  clipTimes.sort((a, b) => a.ms - b.ms);

  const slots: (ClipTime | undefined)[] = [];

  for (const clipTime of clipTimes) {
    if (clipTime.timeType === "start") {
      let emptySlotIndex = slots.findIndex((slot) => !slot);
      if (emptySlotIndex === -1) {
        emptySlotIndex = slots.length;
      }
      if (clipTime.clip === targetClip) {
        return emptySlotIndex;
      }
      slots[emptySlotIndex] = clipTime;
    } else {
      const slotIndex = slots.findIndex((slot) => slot?.clip === clipTime.clip);
      if (slotIndex >= 0) {
        slots[slotIndex] = undefined;
      }
    }
  }

  throw new Error("you should not come here");
}
