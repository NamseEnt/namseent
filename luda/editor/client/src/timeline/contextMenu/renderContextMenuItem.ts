import {
  Render,
  Rect,
  ColorUtil,
  TextAlign,
  TextBaseline,
  Language,
  FontWeight,
  Text,
} from "namui";
import { TimelineState, ContextMenuState } from "../type";
import { closeContextMenu } from "./closeContextMenu";
import { ContextMenuItem } from "./type";

export const renderContextMenuItem: Render<
  {
    timeline: TimelineState;
    contextMenu: ContextMenuState;
  },
  {
    contextMenuItem: ContextMenuItem;
    menuItemHeight: number;
    menuItemWidth: number;
  }
> = (state, props) => {
  const { contextMenuItem, menuItemHeight, menuItemWidth } = props;
  const isMouseInItem = state.contextMenu.mouseInItemId === contextMenuItem.id;
  return [
    Rect({
      x: 0,
      y: 0,
      width: menuItemWidth,
      height: menuItemHeight,
      style: {
        fill: {
          color: isMouseInItem
            ? ColorUtil.Color0255(41, 42, 128)
            : ColorUtil.Transparent,
        },
      },
      onMouseMoveIn() {
        state.contextMenu.mouseInItemId = contextMenuItem.id;
      },
      onMouseMoveOut() {
        if (state.contextMenu.mouseInItemId === contextMenuItem.id) {
          state.contextMenu.mouseInItemId = undefined;
        }
      },
      onClick() {
        contextMenuItem.onClick();
        closeContextMenu(state.timeline);
      },
    }),
    Text({
      x: 5,
      y: 0,
      text: contextMenuItem.label,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      style: {
        color: ColorUtil.White,
      },
      fontType: {
        language: Language.ko,
        serif: false,
        size: 12,
        fontWeight: FontWeight.regular,
      },
    }),
  ];
};
