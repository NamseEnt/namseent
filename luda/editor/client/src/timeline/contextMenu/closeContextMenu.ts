import { TimelineState } from "../type";

export function closeContextMenu(state: TimelineState): void {
  state.contextMenu = undefined;
}
