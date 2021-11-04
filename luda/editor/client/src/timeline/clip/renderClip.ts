import {
  ColorUtil,
  Rect,
  RenderingTree,
  engine,
  Cursor,
  AfterDraw,
  XywhRect,
  Mathu,
  Vector,
} from "namui";
import { TimelineState, Clip } from "../type";
import { Sash } from "./Sash";

export function renderClip(
  props: { height: number; maxRight: number },
  states: {
    timelineState: TimelineState;
    clipState: Clip;
  },
): RenderingTree {
  const { height } = props;
  const { clipState, timelineState } = states;
  const { startMs: clipStartMs, endMs: clipEndMs } = clipState;
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
    timelineState.clipIdMouseIn === clipState.id ||
    timelineState.actionState?.clipId === clipState.id;

  const clipRect: XywhRect = {
    x: x + 1,
    y: 1,
    width: width - 2,
    height: height - 2,
  };

  return [
    Rect({
      ...clipRect,
      style: {
        fill: {
          color: ColorUtil.Color01(0.4, 0.4, 0.8),
        },
        stroke: shouldHighlight
          ? {
              color: ColorUtil.Red,
              width: 3,
            }
          : {
              color: ColorUtil.Black,
              width: 1,
            },
        round: {
          radius: 5,
        },
      },
      onMouseIn() {
        engine.mousePointer.setCursor(Cursor.grab);
      },
    }),
    AfterDraw(({ translated }) => {
      const { mousePosition } = engine.mousePosition;
      {
        const isMouseInClipRect = Mathu.contains(
          Mathu.translate(clipRect, translated),
          mousePosition,
        );

        if (isMouseInClipRect) {
          timelineState.clipIdMouseIn = clipState.id;
        } else if (timelineState.clipIdMouseIn === clipState.id) {
          timelineState.clipIdMouseIn = undefined;
        }
      }

      engine.mouseEvent.onMouseDown((mouseEvent) => {
        if (timelineState.actionState) {
          return;
        }

        const isMouseInRect = Mathu.contains(
          Mathu.translate(clipRect, translated),
          Vector.from(mouseEvent),
        );

        if (!isMouseInRect) {
          return;
        }
        const globalX = translated.x + clipRect.x;
        const mouseAnchorMs =
          (mouseEvent.x - globalX) * timelineState.layout.msPerPixel;

        timelineState.actionState = {
          type: "dragClip",
          clipId: clipState.id,
          mouseAnchorMs,
        };
      });
    }),
    shouldHighlight &&
      Sash(
        {
          clip: clipState,
          timelineState,
        },
        {
          clipRect,
        },
      ),
  ];
}
