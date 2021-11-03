import {
  Clip,
  ColorUtil,
  Rect,
  Render,
  Translate,
  MouseEvent,
  Mathu,
} from "namui";
import { TimelineState, Clip as TimelineClip } from "../type";

export const Sash: Render<
  {
    timelineState: TimelineState;
    clip: TimelineClip;
  },
  {
    clipX: number;
    clipWidth: number;
    height: number;
  }
> = (state, { clipX, clipWidth, height }) => {
  const sashWidth = 10;
  const clippingWidth = clipWidth - 2 * sashWidth;

  function getSashSideOfMouseEvent(
    mouseEvent: MouseEvent,
  ): "left" | "right" | undefined {
    const translatedX = mouseEvent.translated.x;
    if (Mathu.in(mouseEvent.translated.x, 0, sashWidth)) {
      return "left";
    } else if (translatedX < clippingWidth) {
      return undefined;
    } else {
      return "right";
    }
  }

  return Translate(
    {
      x: clipX,
      y: 1,
    },
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(sashWidth, 0, clipWidth - 2 * sashWidth, height),
        ),
        clipOp: CanvasKit.ClipOp.Difference,
      },
      Rect({
        x: 0,
        y: 0,
        width: clipWidth,
        height: height - 1,
        style: {
          fill: {
            color: ColorUtil.Red,
          },
          round: {
            radius: 5,
          },
        },
        onMouseMoveIn: (mouseEvent) => {
          const side = getSashSideOfMouseEvent(mouseEvent);
          state.clip.mouseIn = side;
        },
        onMouseMoveOut: () => {
          state.clip.mouseIn = undefined;
        },
        onMouseDown: (mouseEvent) => {
          const side = getSashSideOfMouseEvent(mouseEvent);
          if (!side) {
            return;
          }
          state.timelineState.actionState = {
            type: "resizeClip",
            clipId: state.clip.id,
            side,
          };
        },
      }),
    ),
  );
};
