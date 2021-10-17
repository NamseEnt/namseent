import { ColorUtil, Rect, RenderingTree } from "namui";
import { TimelineState } from "./type";

export function renderContextMenu(state: TimelineState): RenderingTree {
  const { contextMenu } = state;
  if (!contextMenu) {
    return;
  }

  return [
    Rect({
      x: contextMenu.x,
      y: contextMenu.y,
      width: 100,
      height: 100,
      style: {
        fill: {
          color: ColorUtil.Color01(0.5, 0.5, 0.5),
        },
      },
      onClickOut() {
        state.contextMenu = undefined;
      },
    }),
  ];
}
