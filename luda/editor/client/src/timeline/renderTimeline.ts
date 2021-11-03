import { RenderingTree, Translate, engine, AfterDraw } from "namui";
import { Vector } from "namui/lib/type";
import { renderContextMenu } from "./contextMenu/renderContextMenu";
import { TimelineBody } from "./renderTimelineBody";
import { renderTimelineHeader } from "./renderTimelineHeader";
import { Clip, TimelineState } from "./type";

export function renderTimeline(state: TimelineState): RenderingTree {
  /*
     HEADER         BODY
    ┌──────────────┬────────────────┐
    │ TRACK HEADER │ TRACK BODY     │
    ├──────────────┼────────────────│
    │ TRACK HEADER │ TRACK BODY     │
    ├──────────────┼────────────────│
    │ TRACK HEADER │ TRACK BODY     │
    ├──────────────┼────────────────│
    │              │                │
    │              │                │
    └──────────────┴────────────────┘
  */

  const { layout, tracks } = state;
  const { x, y, width, height, headerWidth } = layout;
  const bodyWidth = width - headerWidth;

  return [
    Translate({ x, y }, [
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
    ]),
    AfterDraw(({ translated }) => {
      handleActionState(state, translated);
    }),
    renderContextMenu(state),
  ];
}

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

        clip[`${actionState.side === "left" ? "start" : "end"}Ms`] = (
          actionState.side === "left" ? Math.min : Math.max
        )(mousePositionMs, oppositeSideMs);
      }
      break;
    case "dragClip":
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
      state.actionState = undefined;
    });
  });
}

function getClip(state: TimelineState, clipId: string): Clip | undefined {
  return state.tracks
    .map((track) => track.clips)
    .flat()
    .find((clip) => clip.id === clipId);
}
