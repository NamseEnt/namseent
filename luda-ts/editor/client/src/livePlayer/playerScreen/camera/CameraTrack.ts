import { Mathu, Render, WhSize } from "namui";
import { isCameraClip } from "../../../clipTypeGuard";
import { Track } from "../../../timeline/type";
import { getPlaybackTimeMs } from "../../operations/getPlaybackTimeMs";
import { LivePlayerState } from "../../type";
import { CameraAngle } from "./CameraAngle";

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
    return;
  }
  if (!isCameraClip(clip)) {
    throw new Error("clip is not camera clip");
  }

  return CameraAngle(
    {},
    {
      cameraAngle: clip.cameraAngle,
      whSize: props.screenWhSize,
    },
  );
};
