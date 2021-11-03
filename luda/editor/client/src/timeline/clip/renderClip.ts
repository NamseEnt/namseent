import {
  ColorUtil,
  Rect,
  RenderingTree,
  engine,
  Cursor,
  AfterDraw,
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

  if (clipState.mouseIn) {
    engine.mousePointer.setCursor(Cursor.leftRightResize);
  } else if (timelineState.clipIdMouseIn === clipState.id) {
    engine.mousePointer.setCursor(Cursor.grab);
  }

  const shouldHighlight =
    clipState.mouseIn ||
    timelineState.clipIdMouseIn === clipState.id ||
    timelineState.actionState?.clipId === clipState.id;

  return [
    Rect({
      x: x + 1,
      y: 1,
      width: width - 2,
      height: height - 2,
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
      onMouseMoveIn: () => {
        timelineState.clipIdMouseIn = clipState.id;
      },
      onMouseMoveOut: () => {
        if (timelineState.clipIdMouseIn === clipState.id) {
          timelineState.clipIdMouseIn = undefined;
        }
      },
    }),
    AfterDraw(({ translated }) => {
      engine.mouseEvent.onMouseDown((mouseEvent) => {
        if (timelineState.actionState?.type) {
          return;
        }

        const isMouseInRect =
          translated.x + x <= mouseEvent.x &&
          mouseEvent.x <= translated.x + x + width &&
          translated.y <= mouseEvent.y &&
          mouseEvent.y <= translated.y + height;

        if (!isMouseInRect) {
          return;
        }
        const globalX = translated.x + x;
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
          clipX: x,
          clipWidth: width,
          height,
        },
      ),
  ];
}
