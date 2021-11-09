import { Render } from "namui";
import { isCameraClip, isSubtitleClip } from "../../clipTypeGuard";
import { Clip } from "../../type";
import { CameraTrackSash } from "../cameraTrack/CameraTrackSash";
import { TimelineState } from "../type";
import { ClipComponent } from "./ClipComponent";
import { SubtitleClipComponent } from "./SubtitleClipComponent";

export const renderClip: Render<
  {
    timelineState: TimelineState;
    clip: Clip;
  },
  {
    height: number;
    maxRight: number;
  }
> = (state, props) => {
  const { clip } = state;

  if (isCameraClip(clip)) {
    console.log("camera");
    return ClipComponent(state, {
      ...props,
      sashComponent: CameraTrackSash,
    });
  }

  if (isSubtitleClip(clip)) {
    return SubtitleClipComponent(
      {
        ...state,
        clip,
      },
      props,
    );
  }

  throw new Error(
    "No rendering implementation for this clip type. Please implement it.",
  );
};
