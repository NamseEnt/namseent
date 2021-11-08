import {
  ColorUtil,
  engine,
  Cursor,
  AfterDraw,
  Translate,
  Render,
  Clip,
} from "namui";
import { SubtitleClip } from "../../type";
import { brighterColor01 } from "../../util/color/brighterColor";
import { TimelineState } from "../type";

export const SubtitleClipComponent: Render<
  {
    timelineState: TimelineState;
    clip: SubtitleClip;
  },
  { height: number; maxRight: number }
> = (states, props) => {
  const { height } = props;
  const { clip, timelineState } = states;
  const { startMs: clipStartMs, endMs: clipEndMs } = clip;
  const x =
    (clipStartMs - timelineState.layout.startMs) /
    timelineState.layout.msPerPixel;
  const durationMs = clipEndMs - clipStartMs;
  const width = durationMs / timelineState.layout.msPerPixel;

  const isOutOfBounds = x + width < 0 || x > props.maxRight;
  if (isOutOfBounds) {
    return;
  }

  const shouldHighlight =
    timelineState.selectedClip?.id === clip.id ||
    timelineState.clipIdMouseIn === clip.id ||
    timelineState.actionState?.clipId === clip.id;

  const borderWidth = (shouldHighlight ? 3 : 1) * 2;
  const componentWidth = 200 / timelineState.layout.msPerPixel;
  const componentHeight = height / 3;
  const headPosition = {
    x: 0,
    y: 0,
  };
  const tailPosition = {
    x: width - componentWidth,
    y: height - componentHeight,
  };
  const color =
    timelineState.selectedClip?.id === clip.id
      ? ColorUtil.Green
      : ColorUtil.Color0255(34, 167, 240);
  const brighterColor = brighterColor01(color, 0.2);

  const strokePath = new CanvasKit.Path()
    .moveTo(
      headPosition.x + componentWidth / 2,
      headPosition.y + componentHeight,
    )
    .lineTo(tailPosition.x + componentWidth / 2, tailPosition.y);

  const headPath = new CanvasKit.Path()
    .moveTo(0, 0)
    .lineTo(0, componentHeight)
    .lineTo(componentWidth, componentHeight)
    .lineTo(componentWidth, componentHeight / 3)
    .close();

  const tailPath = new CanvasKit.Path()
    .moveTo(0, 0)
    .lineTo(0, (componentHeight * 2) / 3)
    .lineTo(componentWidth, componentHeight)
    .lineTo(componentWidth, 0)
    .close();

  const fillPaint = new CanvasKit.Paint();
  fillPaint.setAntiAlias(true);
  fillPaint.setColor(color);

  const borderPaint = new CanvasKit.Paint();
  borderPaint.setAntiAlias(true);
  borderPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  borderPaint.setStrokeWidth(borderWidth);
  borderPaint.setColor(brighterColor);

  const strokeFillPaint = new CanvasKit.Paint();
  strokeFillPaint.setAntiAlias(true);
  strokeFillPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  strokeFillPaint.setStrokeWidth(borderWidth);
  strokeFillPaint.setStrokeCap(CanvasKit.StrokeCap.Round);
  strokeFillPaint.setColor(color);

  const strokeBorderPaint = new CanvasKit.Paint();
  strokeBorderPaint.setAntiAlias(true);
  strokeBorderPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  strokeBorderPaint.setStrokeWidth(borderWidth + borderWidth);
  strokeBorderPaint.setStrokeCap(CanvasKit.StrokeCap.Round);
  strokeBorderPaint.setColor(brighterColor);

  return Translate(
    {
      x,
      y: 0,
    },
    [
      {
        drawCalls: [
          {
            commands: [
              {
                type: "path",
                path: strokePath,
                paint: strokeBorderPaint,
              },
            ],
          },
        ],
      },

      Translate(
        headPosition,
        Clip(
          {
            path: headPath,
            clipOp: CanvasKit.ClipOp.Intersect,
          },
          {
            drawCalls: [
              {
                commands: [
                  {
                    type: "path",
                    path: headPath,
                    paint: fillPaint,
                  },
                  {
                    type: "path",
                    path: headPath,
                    paint: borderPaint,
                  },
                ],
              },
            ],
          },
        ),
      ),

      Translate(
        tailPosition,
        Clip(
          {
            path: tailPath,
            clipOp: CanvasKit.ClipOp.Intersect,
          },
          {
            drawCalls: [
              {
                commands: [
                  {
                    type: "path",
                    path: tailPath,
                    paint: fillPaint,
                  },
                  {
                    type: "path",
                    path: tailPath,
                    paint: borderPaint,
                  },
                ],
              },
            ],
          },
        ),
      ),

      {
        drawCalls: [
          {
            commands: [
              {
                type: "path",
                path: strokePath,
                paint: strokeFillPaint,
              },
            ],
          },
        ],
      },

      AfterDraw(({ translated }) => {
        const mouse = engine.mousePosition.mousePosition;

        const mouseInHead = headPath.contains(
          mouse.x - translated.x - headPosition.x,
          mouse.y - translated.y - headPosition.y,
        );

        const mouseInTail = tailPath.contains(
          mouse.x - translated.x - tailPosition.x,
          mouse.y - translated.y - tailPosition.y,
        );

        if (mouseInHead) {
          engine.mousePointer.setCursor(Cursor.grab);
        }

        if (mouseInTail) {
          engine.mousePointer.setCursor(Cursor.leftRightResize);
        }

        const mouseIn = mouseInHead || mouseInTail;

        if (mouseIn) {
          timelineState.clipIdMouseIn = clip.id;
        }

        if (timelineState.clipIdMouseIn === clip.id && !mouseIn) {
          timelineState.clipIdMouseIn = undefined;
        }

        engine.mouseEvent.onMouseDown((mouseEvent) => {
          if (mouseIn) {
            timelineState.selectedClip = clip;
          } else if (timelineState.selectedClip?.id === clip.id) {
            timelineState.selectedClip = undefined;
          }

          if (timelineState.actionState?.type) {
            return;
          }

          if (mouseInHead) {
            const mouseAnchorMs =
              (mouseEvent.x - translated.x) * timelineState.layout.msPerPixel;

            timelineState.actionState = {
              type: "dragClip",
              clipId: clip.id,
              mouseAnchorMs,
            };
          }

          if (mouseInTail) {
            timelineState.actionState = {
              type: "resizeClip",
              clipId: clip.id,
              side: "right",
              sashMouseAnchorMs:
                (mouseEvent.x - translated.x - width) *
                timelineState.layout.msPerPixel,
            };
          }
        });
      }),
    ],
  );
};
