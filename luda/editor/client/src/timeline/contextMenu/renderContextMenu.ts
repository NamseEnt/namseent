import { ColorUtil, Rect, Render, RenderingTree, Translate } from "namui";
import { TimelineState } from "../type";
import { nanoid } from "nanoid";
import { ContextMenuItem } from "./type";
import { renderContextMenuItem } from "./renderContextMenuItem";
import { closeContextMenu } from "./closeContextMenu";
import { createClip } from "../operations/createClip";

export const renderContextMenu: Render<TimelineState> = (state) => {
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
        const newClip = createClip({
          trackType: track.type,
          id: nanoid(),
          startMs: contextMenu.clickMs,
          endMs: contextMenu.clickMs + 3000,
        });
        track.clips.push(newClip);
      },
    },
  ];
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
