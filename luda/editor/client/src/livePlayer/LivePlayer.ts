import {
  BorderPosition,
  ColorUtil,
  Rect,
  Render,
  Translate,
  XywhRect,
} from "namui";
import { Track } from "../timeline/type";
import { Buttons } from "./Buttons";
import { PlayerScreen } from "./playerScreen/PlayerScreen";
import { renderTitleHeader } from "./renderTitleHeader";
import { LivePlayerState } from "./type";

export type LivePlayerProps = {
  tracks: Track[];
  layout: XywhRect & {};
};

export const LivePlayer: Render<LivePlayerState, LivePlayerProps> = (
  state,
  props,
) => {
  const { layout } = props;
  const titleHeaderCenterY = layout.height * (0.5 / 6);
  const titleHeaderCenterX = layout.width * 0.5;

  const playerScreenRect: XywhRect = {
    x: 0,
    y: layout.height * (1 / 6),
    width: layout.width,
    height: layout.height * (4 / 6),
  };

  const buttonsLayout: XywhRect = {
    x: 0,
    y: layout.height * (5 / 6),
    width: layout.width,
    height: layout.height * (1 / 6),
  };
  return Translate(
    {
      x: layout.x,
      y: layout.y,
    },
    [
      Rect({
        x: 0,
        y: 0,
        width: layout.width,
        height: layout.height,
        style: {
          stroke: {
            color: ColorUtil.Black,
            borderPosition: BorderPosition.middle,
            width: 1,
          },
        },
      }),
      renderTitleHeader(
        {},
        {
          centerY: titleHeaderCenterY,
          centerX: titleHeaderCenterX,
        },
      ),
      PlayerScreen(
        {},
        {
          livePlayerState: state,
          tracks: props.tracks,
          layout: playerScreenRect,
        },
      ),
      Buttons(state, {
        layout: buttonsLayout,
      }),
    ],
  );
};
