import { ColorUtil, Rect, Render, RenderingTree } from "namui";
import { TimelineState, Clip } from "../type";

export const Sash: Render<
  {
    timelineState: TimelineState;
    clip: Clip;
  },
  {
    clipX: number;
    clipWidth: number;
    sashWidth: number;
    maxRight: number;
    height: number;
    side: "left" | "right";
  }
> = (state, { clipX, clipWidth, sashWidth, maxRight, height, side }) => {
  const leftSashLeft = clipX - sashWidth / 2;
  const leftSashRight = leftSashLeft + leftSashLeft;

  const rightSashLeft = clipX + clipWidth - sashWidth / 2;
  const rightSashRight = rightSashLeft + sashWidth;

  const left = side === "left" ? leftSashLeft : rightSashLeft;
  const right = side === "left" ? leftSashRight : rightSashRight;
  const isVisible = right > 0 && left < maxRight;

  if (!isVisible) {
    return;
  }

  const shouldHighlight =
    (state.clip.mouseIn === side && !state.timelineState.actionState) ||
    (state.timelineState.actionState?.type === "resizeClip" &&
      state.timelineState.actionState.side === side &&
      state.timelineState.actionState.clipId === state.clip.id);

  return [
    Rect({
      x: left,
      y: 0,
      width: sashWidth,
      height,
      style: {
        fill: {
          color: shouldHighlight ? ColorUtil.Blue : ColorUtil.Transparent,
        },
      },
      onMouseMoveIn: () => {
        // TODO : what if mouse is on two contiguous clips's sashes?
        state.clip.mouseIn = side;
      },
      onMouseMoveOut: () => {
        if (state.clip.mouseIn === side) {
          state.clip.mouseIn = undefined;
        }
      },
      onMouseDown: () => {
        state.timelineState.actionState = {
          type: "resizeClip",
          clipId: state.clip.id,
          side,
        };
      },
    }),
  ];
};
