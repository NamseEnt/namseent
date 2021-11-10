import { LivePlayerState } from "../type";
import { pauseLivePlayer } from "./pauseLivePlayer";
import { playLivePlayer } from "./playLivePlayer";

export function changeLivePlayerPlaybackTime(
  state: LivePlayerState,
  playbackTimeMs: number,
): void {
  if (!state.isPlaying) {
    state.anchorMs = playbackTimeMs;
    return;
  }
  pauseLivePlayer(state);
  state.anchorMs = playbackTimeMs;
  playLivePlayer(state);
}
