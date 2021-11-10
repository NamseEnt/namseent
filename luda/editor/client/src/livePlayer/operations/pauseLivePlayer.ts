import { LivePlayerState } from "../type";
import { getPlaybackTimeMs } from "./getPlaybackTimeMs";

export function pauseLivePlayer(state: LivePlayerState): void {
  state.anchorMs = getPlaybackTimeMs(state);
  state.isPlaying = false;
}
