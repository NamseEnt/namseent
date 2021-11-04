import {
  Clip,
  ColorUtil,
  Rect,
  Render,
  Translate,
  MouseEvent,
  Mathu,
  engine,
  Cursor,
  AfterDraw,
  XywhRect,
  Vector,
} from "namui";
import { TimelineState, Clip as TimelineClip } from "../type";

export const Sash: Render<
  {
    timelineState: TimelineState;
    clip: TimelineClip;
  },
  {
    clipRect: XywhRect;
  }
> = (state, { clipRect }) => {
  const sashWidth = 10;
  const clippingWidth = clipRect.width - 2 * sashWidth;

  function getSashSideOfMouseEvent(
    clippedLocalVector: Vector,
  ): "left" | "right" | undefined {
    if (Mathu.in(clippedLocalVector.x, 0, sashWidth)) {
      return "left";
    } else if (clippedLocalVector.x < clippingWidth) {
      return undefined;
    } else {
      return "right";
    }
  }

  const sashRect: XywhRect = {
    x: 0,
    y: 0,
    width: clipRect.width,
    height: clipRect.height,
  };

  return Translate(
    {
      x: clipRect.x,
      y: 1,
    },
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(
            sashWidth,
            0,
            clipRect.width - 2 * sashWidth,
            clipRect.height,
          ),
        ),
        clipOp: CanvasKit.ClipOp.Difference,
      },
      [
        Rect({
          ...sashRect,
          style: {
            fill: {
              color: ColorUtil.Red,
            },
            round: {
              radius: 5,
            },
          },
          onMouseIn: () => {
            engine.mousePointer.setCursor(Cursor.leftRightResize);
          },
        }),
        AfterDraw(({ translated }) => {
          engine.mouseEvent.onMouseDown((mouseEvent) => {
            const globalSashRect = Mathu.translate(sashRect, translated);
            const isMouseInSashRect = Mathu.contains(
              globalSashRect,
              Vector.from(mouseEvent),
            );

            if (!isMouseInSashRect) {
              return;
            }

            const side = getSashSideOfMouseEvent(
              Vector.from(mouseEvent).sub(translated),
            );
            if (!side) {
              return;
            }

            const { startMs, endMs } = state.clip;
            const { layout } = state.timelineState;

            const mouseXMs =
              (mouseEvent.x - layout.x - layout.headerWidth) *
                layout.msPerPixel +
              layout.startMs;

            const durationMs = endMs - startMs;
            const sashMouseAnchorMs =
              side === "left"
                ? mouseXMs - startMs
                : mouseXMs - (startMs + durationMs);

            state.timelineState.actionState = {
              type: "resizeClip",
              clipId: state.clip.id,
              side,
              sashMouseAnchorMs,
            };
          });
        }),
      ],
    ),
  );
};
