import { Render } from "namui";
import { Track } from "../../../timeline/type";
import { LivePlayerState } from "../../type";

export const SubtitleTrack: Render<
  {},
  {
    track: Track;
    livePlayerState: LivePlayerState;
  }
> = (state, props) => {
  return undefined;
  // throw new Error("Method not implemented.");
};
