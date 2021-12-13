import { Render } from "namui";
import { Track } from "../../timeline/type";
import { LivePlayerState } from "../type";

export const PlayingTrack: Render<
  {},
  {
    track: Track;
    livePlayerState: LivePlayerState;
  }
> = (state, props) => {
  throw new Error("Not implemented");
};
