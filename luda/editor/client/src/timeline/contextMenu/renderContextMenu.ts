import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { ContextMenu, TimelineState } from "../type";
import { nanoid } from "nanoid";

type ContextMenuItem = {
  id: string;
  label: string;
  onClick: () => void;
};

export function renderContextMenu(state: TimelineState): RenderingTree {
  const { contextMenu } = state;
  if (!contextMenu) {
    return;
  }
  const contextMenuItems: ContextMenuItem[] = [
    {
      id: "0",
      label: "클립 추가하기",
      onClick: () => {
        const track = state.tracks.find(
          (track) => track.id === contextMenu.trackId,
        );
        if (!track) {
          throw new Error("track not found");
        }
        track.clips.push({
          id: nanoid(),
          startMs: contextMenu.clickMs,
          endMs: contextMenu.clickMs + 3000,
        });
      },
    },
  ];
  const menuItemHeight = 24;
  const menuHeight = contextMenuItems.length * menuItemHeight;
  const menuWidth = 100;

  const renderingContextMenuItems: RenderingTree[] = contextMenuItems.map(
    (contextMenuItem, index) =>
      Translate({ x: 0, y: menuItemHeight * index }, [
        renderContextMenuItem({
          contextMenuItem,
          state,
          contextMenu,
          menuItemHeight,
          menuItemWidth: menuWidth,
        }),
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
}
function renderContextMenuItem({
  contextMenuItem,
  state,
  contextMenu,
  menuItemHeight,
  menuItemWidth,
}: {
  contextMenuItem: ContextMenuItem;
  state: TimelineState;
  contextMenu: ContextMenu;
  menuItemHeight: number;
  menuItemWidth: number;
}): RenderingTree {
  const isMouseInItem = contextMenu.mouseInItemId === contextMenuItem.id;
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
        contextMenu.mouseInItemId = contextMenuItem.id;
      },
      onMouseMoveOut() {
        if (contextMenu.mouseInItemId === contextMenuItem.id) {
          contextMenu.mouseInItemId = undefined;
        }
      },
      onClick() {
        contextMenuItem.onClick();
        closeContextMenu(state);
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
}

function closeContextMenu(state: TimelineState): void {
  state.contextMenu = undefined;
}
