import { TimelineSequenceNullableState, Track } from "../type";

export function initTimeline(state: TimelineSequenceNullableState): void;
export function initTimeline(
  state: TimelineSequenceNullableState,
  tracks: Track[],
  title: string,
): void;
export function initTimeline(
  state: TimelineSequenceNullableState,
  tracks?: Track[],
  title?: string,
) {
  state.actionState = undefined;
  state.clipIdMouseIn = undefined;
  state.contextMenu = undefined;
  state.selectedClip = undefined;
  state.title = title;
  state.tracks = tracks || [];
}
