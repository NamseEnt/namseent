import { TimelineState, Track, TrackType } from "../type";
import { refreshCameraClipPositions } from "./refreshCameraClipPositions";
import { refreshSubtitleClipPositions } from "./refreshSubtitleClipPositions";

export function refreshClipPositions(
  timelineState: TimelineState,
  track: Track,
): void {
  switch (track.type) {
    case TrackType.camera:
      refreshCameraClipPositions(timelineState, track);
      break;
    case TrackType.subtitle:
      refreshSubtitleClipPositions(timelineState, track);
      break;
    default:
      break;
  }
}
