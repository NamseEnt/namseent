import { ColorUtil, FontWeight, Language } from "namui";
import {
  BaseClip,
  CameraAngle,
  CameraClip,
  Clip,
  SubtitleClip,
  SubtitleFontSize,
} from "../../type";
import { TrackType } from "../type";

type BaseArgs = {
  id: string;
  startMs: number;
  endMs: number;
};
type ArgsOfTrackType = {
  [TrackType.camera]: BaseArgs;
  [TrackType.subtitle]: BaseArgs & {
    text?: string;
  };
};

export function createClip(
  args: {
    trackType: TrackType.camera;
  } & ArgsOfTrackType[TrackType.camera],
): Clip;
export function createClip(
  args: {
    trackType: TrackType.subtitle;
  } & ArgsOfTrackType[TrackType.subtitle],
): Clip;
export function createClip(
  args: {
    trackType: TrackType;
  } & BaseArgs,
): Clip;
export function createClip(
  args: {
    trackType: TrackType;
  } & BaseArgs,
): Clip {
  const trackType = args.trackType;
  switch (trackType) {
    case TrackType.camera:
      const cameraArgs = args as ArgsOfTrackType[typeof trackType];

      const cameraClip: CameraClip = {
        type: trackType,
        id: cameraArgs.id,
        startMs: cameraArgs.startMs,
        endMs: cameraArgs.endMs,
        cameraAngle: {
          // TODO: it should be empty.
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
      const subtitleArgs = args as ArgsOfTrackType[typeof trackType];

      const subtitleClip: SubtitleClip = {
        type: trackType,
        id: subtitleArgs.id,
        startMs: subtitleArgs.startMs,
        endMs: subtitleArgs.endMs,
        subtitle: {
          text: subtitleArgs.text ?? "Input text here",
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
