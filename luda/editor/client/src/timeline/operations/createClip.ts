import { ColorUtil, FontWeight, Language } from "namui";
import {
  BaseClip,
  CameraClip,
  Clip,
  SubtitleClip,
  SubtitleFontSize,
} from "../../type";
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

    case TrackType.subtitle:
      const subtitleClip: SubtitleClip = {
        id,
        type: trackType,
        startMs,
        endMs,
        subtitle: {
          text: "Input text here",
          fontType: {
            fontWeight: FontWeight.regular,
            language: Language.ko,
            serif: false,
            size: SubtitleFontSize.regular,
          },
          style: {
            background: {
              color: ColorUtil.Black,
            },
            border: {
              color: ColorUtil.Transparent,
              width: 0,
            },
            color: ColorUtil.White,
            dropShadow: {
              color: ColorUtil.Transparent,
              x: 0,
              y: 0,
            },
          },
        },
      };
      return subtitleClip;

    default:
      throw new Error(`Unknown track type ${trackType}. please implement.`);
  }
}
