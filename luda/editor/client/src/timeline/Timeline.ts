import {
  Translate,
  engine,
  AfterDraw,
  Cursor,
  Render,
  Rect,
  ColorUtil,
} from "namui";
import { Vector } from "namui/lib/type";
import { Clip } from "../type";
import { renderContextMenu } from "./contextMenu/renderContextMenu";
import { TimelineBody } from "./renderTimelineBody";
import { renderTimelineHeader } from "./renderTimelineHeader";
import { TimeRuler } from "./timeRuler/TimeRuler";
import { TimelineState } from "./type";

export const Timeline: Render<TimelineState> = (state) => {
  /*
     HEADER         BODY
    ┌──────────────┬────────────────┐
    │  00:00:TIME  │   TIME RULER   │
    ├──────────────┼────────────────│
    │ TRACK HEADER │   TRACK BODY   │
    ├──────────────┼────────────────│
    │ TRACK HEADER │   TRACK BODY   │
    ├──────────────┼────────────────│
    │ TRACK HEADER │   TRACK BODY   │
    ├──────────────┼────────────────│
    │              │                │
    │              │                │
    └──────────────┴────────────────┘
  */

  const { layout, tracks } = state;
  const { x, y, width, height, headerWidth } = layout;
  const bodyWidth = width - headerWidth;

  return [
    Rect({
      ...layout,
      id: state.timelineBorderId,
      style: {
        fill: {
          color: ColorUtil.Transparent,
        },
      },
    }),
    Translate({ x, y }, [
      Translate(
        {
          x: headerWidth,
          y: 0,
        },
        TimeRuler(
          {},
          {
            layout: {
              x: 0,
              y: 0,
              width: bodyWidth,
              height: state.layout.timeRulerHeight,
            },
            msPerPixel: state.layout.msPerPixel,
            startMs: state.layout.startMs,
          },
        ),
      ),
      Translate(
        {
          x: 0,
          y: state.layout.timeRulerHeight,
        },
        [
          renderTimelineHeader({
            width: headerWidth,
            height,
            tracks,
          }),
          Translate(
            {
              x: headerWidth,
              y: 0,
            },
            TimelineBody(
              {
                timelineState: state,
                tracks,
              },
              {
                width: bodyWidth,
                height,
              },
            ),
          ),
        ],
      ),
    ]),
    AfterDraw(({ translated }) => {
      handleActionState(state, translated);
    }),
    renderContextMenu(state),
  ];
};

function getMousePositionMs(
  state: TimelineState,
  translatedTimelineVector: Vector,
): number {
  const { mousePosition } = engine.mousePosition;
  const trackBodyLeftX =
    translatedTimelineVector.x + state.layout.x + state.layout.headerWidth;
  const mouseXOnTrackBody = mousePosition.x - trackBodyLeftX;
  return state.layout.startMs + mouseXOnTrackBody * state.layout.msPerPixel;
}

function handleActionState(
  state: TimelineState,
  translatedTimelineVector: Vector,
): void {
  const { actionState } = state;
  if (!actionState) {
    return;
  }
  if (actionState.terminatePhase === "terminating") {
    actionState.terminatePhase = "terminated";
  } else if (actionState.terminatePhase === "terminated") {
    return (state.actionState = undefined);
  }

  switch (actionState.type) {
    case "resizeClip":
      {
        registerDraggingActionResetCallback(state);

        const clip = getClip(state, actionState.clipId);
        if (!clip) {
          throw new Error("clip not found");
        }

        const mousePositionMs = getMousePositionMs(
          state,
          translatedTimelineVector,
        );

        const oppositeSideMs =
          clip[`${actionState.side === "left" ? "end" : "start"}Ms`];
        const availableBorderMs =
          oppositeSideMs + (actionState.side === "left" ? -1 : 1) * 200;

        const nextMs = mousePositionMs - actionState.sashMouseAnchorMs;

        clip[`${actionState.side === "left" ? "start" : "end"}Ms`] = (
          actionState.side === "left" ? Math.min : Math.max
        )(nextMs, availableBorderMs);
      }
      break;
    case "dragClip":
      {
        registerDraggingActionResetCallback(state);

        engine.mousePointer.setCursor(Cursor.grab);

        const clip = getClip(state, actionState.clipId);
        if (!clip) {
          throw new Error("clip not found");
        }

        const mousePositionMs = getMousePositionMs(
          state,
          translatedTimelineVector,
        );

        const durationMs = clip.endMs - clip.startMs;
        clip.startMs = mousePositionMs - actionState.mouseAnchorMs;
        clip.endMs = clip.startMs + durationMs;
      }
      break;
    default:
      console.warn(`unknown actionState: ${(actionState as any).type}`);
  }
}
function registerDraggingActionResetCallback(state: TimelineState): void {
  const actionResetEventRegisters = [
    (callback: () => void) => engine.mouseEvent.onMouseUp(callback),
    (callback: () => void) => engine.mouseEvent.onMouseOut(callback),
    (callback: () => void) => {
      engine.screen.onVisibilityChange((visible) => {
        if (!visible) {
          callback();
        }
      });
    },
  ] as const;

  actionResetEventRegisters.forEach((eventRegister) => {
    eventRegister(() => {
      if (!state.actionState) {
        return;
      }
      if (state.actionState.terminatePhase) {
        return;
      }
      state.actionState.terminatePhase = "terminating";
    });
  });
}

function getClip(state: TimelineState, clipId: string): Clip | undefined {
  return state.tracks
    .map((track) => track.clips)
    .flat()
    .find((clip) => clip.id === clipId);
}
