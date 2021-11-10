import {
  BorderPosition,
  ColorUtil,
  Rect,
  Render,
  Translate,
  Vector,
  XywhRect,
} from "namui";
import { Track } from "../../timeline/type";
import { LivePlayerState } from "../type";
import { PausedTrack } from "./PausedTrack";

export type PlayerScreenState = {};
export type PlayerScreenProps = {
  tracks: Track[];
  livePlayerState: LivePlayerState;
  layout: XywhRect;
};

export const PlayerScreen: Render<PlayerScreenState, PlayerScreenProps> = (
  state,
  props,
) => {
  const screenCenter = new Vector(
    props.layout.width / 2,
    props.layout.height / 2,
  );
  const screenWhRatio = 16 / 9;
  const screenWidth = props.layout.width;
  const screenHeight = screenWidth / screenWhRatio;
  const screenRect: XywhRect = {
    x: screenCenter.x - screenWidth / 2,
    y: screenCenter.y - screenHeight / 2,
    width: screenWidth,
    height: screenHeight,
  };

  return Translate(Vector.from(props.layout).add(Vector.from(screenRect)), [
    Rect({
      ...screenRect,
      x: 0,
      y: 0,
      style: {
        stroke: {
          borderPosition: BorderPosition.outside,
          color: ColorUtil.Black,
          width: 1,
        },
      },
    }),
    props.tracks.map((track) => {
      // if (!props.livePlayerState.isPlaying) {
      return PausedTrack(
        {},
        {
          track,
          livePlayerState: props.livePlayerState,
          screenWhSize: screenRect,
        },
      );
      // }
      // return PlayingTrack({}, { track, livePlayerState: props.livePlayerState });
    }),
  ]);
};
