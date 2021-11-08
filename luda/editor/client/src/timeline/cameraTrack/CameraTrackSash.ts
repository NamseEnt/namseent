import {
  Clip,
  ColorUtil,
  Rect,
  Translate,
  Mathu,
  engine,
  Cursor,
  AfterDraw,
  XywhRect,
  Vector,
  Convert,
} from "namui";
import { SashComponent } from "../clip/Sash";

export const CameraTrackSash: SashComponent = (state, { clipRect }) => {
  const sashWidth = 10;
  const clippingWidth = clipRect.width - 2 * sashWidth;

  function getSashSideOfMouseEvent(
    clippedLocalVector: Vector,
  ): "right" | undefined {
    if (clippedLocalVector.x < clippingWidth) {
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

  const clippedRect: XywhRect = {
    x: 0,
    y: 0,
    width: clipRect.width - sashWidth,
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
          Convert.xywhToCanvasKit(clippedRect),
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
            const globalClippedRect = Mathu.translate(clippedRect, translated);
            const isMouseInSashRect =
              Mathu.contains(globalSashRect, Vector.from(mouseEvent)) &&
              !Mathu.contains(globalClippedRect, Vector.from(mouseEvent));

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
            const sashMouseAnchorMs = mouseXMs - (startMs + durationMs);

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
