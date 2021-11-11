import { Render, WhSize } from "namui";
import { Track, TrackType } from "../../timeline/type";
import { LivePlayerState } from "../type";
import { CameraTrack } from "./camera/CameraTrack";
import { SubtitleTrack } from "./subtitle/SubtitleTrack";

export const PausedTrack: Render<
  {},
  {
    track: Track;
    livePlayerState: LivePlayerState;
    screenWhSize: WhSize;
  }
> = (state, props) => {
  switch (props.track.type) {
    case TrackType.camera:
      return CameraTrack(state, props);
    case TrackType.subtitle:
      return SubtitleTrack(state, props);
  }
};
