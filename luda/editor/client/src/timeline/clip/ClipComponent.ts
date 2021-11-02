import {
  ColorUtil,
  Rect,
  engine,
  Cursor,
  AfterDraw,
  XywhRect,
  Mathu,
  Vector,
  Render,
  BorderPosition,
} from "namui";
import { isSubtitleClip } from "../../clipTypeGuard";
import { Clip } from "../../type";
import { TimelineState } from "../type";
import { SashComponent } from "./Sash";
import { SubtitleClipComponent } from "./SubtitleClipComponent";

export const ClipComponent: Render<
  {
    timelineState: TimelineState;
    clip: Clip;
  },
  {
    height: number;
    maxRight: number;
    sashComponent: SashComponent;
  }
> = (state, props) => {
  const { height } = props;
  const { clip, timelineState } = state;
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
    state.timelineState.selectedClip?.id === clip.id ||
    timelineState.clipIdMouseIn === clip.id ||
    timelineState.actionState?.clipId === clip.id;

  const clipRect: XywhRect = {
    x: x + 1,
    y: 1,
    width: width - 2,
    height: height - 2,
  };

  // TODO: If possible, it should be integrated with ClipComponent.
  if (isSubtitleClip(clip)) {
    return SubtitleClipComponent(
      {
        clip,
        timelineState,
      },
      props,
    );
  }

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
              borderPosition: BorderPosition.inside,
            }
          : {
              color: ColorUtil.Black,
              width: 1,
              borderPosition: BorderPosition.inside,
            },
        round: {
          radius: 5,
        },
      },
      onMouseIn() {
        engine.mousePointer.setCursor(Cursor.grab);
      },
      onMouseDown() {
        state.timelineState.selectedClip = state.clip;
      },
      onClickOut(mouseEvent) {
        if (
          state.timelineState.selectedClip?.id === state.clip.id &&
          !engine.render.isGlobalVectorOutOfRenderingData(
            Vector.from(mouseEvent),
            state.timelineState.timelineBorderId,
          )
        ) {
          state.timelineState.selectedClip = undefined;
        }
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
          timelineState.clipIdMouseIn = clip.id;
        } else if (timelineState.clipIdMouseIn === clip.id) {
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
          clipId: clip.id,
          mouseAnchorMs,
        };
      });
    }),
    shouldHighlight &&
      props.sashComponent(
        {
          clip: clip,
          timelineState,
        },
        {
          clipRect,
        },
      ),
  ];
};
