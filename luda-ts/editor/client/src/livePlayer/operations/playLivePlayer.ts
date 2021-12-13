import { LivePlayerState } from "../type";

export function playLivePlayer(state: LivePlayerState): void {
  state.isPlaying = true;
  state.playStartTimeMs = Date.now();
}
