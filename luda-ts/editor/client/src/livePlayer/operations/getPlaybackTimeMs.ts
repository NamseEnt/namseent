import { LivePlayerState } from "../type";

export function getPlaybackTimeMs(livePlayerState: LivePlayerState): number {
  if (livePlayerState.isPlaying) {
    return (
      livePlayerState.anchorMs + Date.now() - livePlayerState.playStartTimeMs
    );
  } else {
    return livePlayerState.anchorMs;
  }
}
