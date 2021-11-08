import { CameraClip, Clip } from "../../type";
import { TrackType } from "../type";

export function createClip({
  trackType,
  id,
  startMs,
  endMs,
}: {
  trackType: TrackType;
  id: string;
  startMs: number;
  endMs: number;
}): Clip {
  switch (trackType) {
    case TrackType.camera:
      const cameraClip: CameraClip = {
        id,
        type: trackType,
        startMs,
        endMs,
        cameraAngle: {
          imageSourceUrl: "resources/images/피디-기본-미소.png",
          source01Rect: {
            x: 0.25,
            y: 0.25,
            width: 0,
            height: 0.5,
          },
          dest01Rect: {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
          },
        },
      };
      return cameraClip;
    default:
      throw new Error(`Unknown track type ${trackType}. please implement.`);
  }
}
