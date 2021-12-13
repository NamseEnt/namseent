import { nanoid } from "nanoid";
import { createClip } from "../operations/createClip";
import { ContextMenuState, TimelineState } from "../type";
import { ContextMenuItem } from "./type";

export function getContextMenuItems({
  timeline,
  contextMenu,
}: {
  timeline: TimelineState;
  contextMenu: ContextMenuState;
}): ContextMenuItem[] {
  switch (contextMenu.type) {
    case "trackBody":
      return [
        {
          id: "0",
          label: "클립 추가하기",
          onClick: () => {
            const track = timeline.tracks.find(
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
    case "clip":
      return [
        {
          id: "0",
          label: "클립 제거하기",
          onClick: () => {
            const track = timeline.tracks.find(
              (track) => track.id === contextMenu.trackId,
            );
            if (!track) {
              throw new Error("track not found");
            }
            const index = track.clips.findIndex(
              (clip) => clip.id === contextMenu.clipId,
            );
            if (index === -1) {
              throw new Error("clip not found");
            }
            track.clips.splice(index, 1);
          },
        },
      ];
    default:
      throw new Error(
        `Unknown context menu type ${
          (contextMenu as any).type
        }, please implement it.`,
      );
  }
}
