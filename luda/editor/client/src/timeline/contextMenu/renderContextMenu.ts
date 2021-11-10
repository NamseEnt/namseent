import { ColorUtil, Rect, Render, RenderingTree, Translate } from "namui";
import { TimelineState } from "../type";
import { renderContextMenuItem } from "./renderContextMenuItem";
import { closeContextMenu } from "./closeContextMenu";
import { getContextMenuItems } from "./getContextMenuItems";

export const renderContextMenu: Render<TimelineState> = (state) => {
  const { contextMenu } = state;
  if (!contextMenu) {
    return;
  }

  const contextMenuItems = getContextMenuItems({
    timeline: state,
    contextMenu,
  });

  const menuItemHeight = 24;
  const menuHeight = contextMenuItems.length * menuItemHeight;
  const menuWidth = 100;

  const renderingContextMenuItems: RenderingTree[] = contextMenuItems.map(
    (contextMenuItem, index) =>
      Translate({ x: 0, y: menuItemHeight * index }, [
        renderContextMenuItem(
          {
            timeline: state,
            contextMenu,
          },
          {
            contextMenuItem,
            menuItemHeight,
            menuItemWidth: menuWidth,
          },
        ),
      ]),
  );

  return [
    Translate({ x: contextMenu.x, y: contextMenu.y }, [
      Rect({
        x: 0,
        y: 0,
        width: menuWidth,
        height: menuHeight,
        style: {
          fill: {
            color: ColorUtil.Color0255(41, 42, 45),
          },
        },
        onClickOut() {
          closeContextMenu(state);
        },
      }),
      renderingContextMenuItems,
    ]),
  ];
};
