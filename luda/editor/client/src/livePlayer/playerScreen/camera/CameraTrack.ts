import {
  engine,
  Image,
  ImageFit,
  Mathu,
  Render,
  WhSize,
  XywhRect,
} from "namui";
import { isCameraClip } from "../../../clipTypeGuard";
import { Track } from "../../../timeline/type";
import { getPlaybackTimeMs } from "../../operations/getPlaybackTimeMs";
import { LivePlayerState } from "../../type";
import { CameraClip } from "./CameraClip";

export const CameraTrack: Render<
  {},
  {
    track: Track;
    livePlayerState: LivePlayerState;
    screenWhSize: WhSize;
  }
> = (state, props) => {
  const playbackTimeMs = getPlaybackTimeMs(props.livePlayerState);
  const clip = props.track.clips.find((clip) =>
    Mathu.in(playbackTimeMs, clip.startMs, clip.endMs),
  );
  if (!clip) {
    console.log("no clip");
    return;
  }
  if (!isCameraClip(clip)) {
    throw new Error("clip is not camera clip");
  }

  return CameraClip(
    {},
    {
      cameraAngle: clip.cameraAngle,
      whSize: props.screenWhSize,
    },
  );
};
