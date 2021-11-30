import { TimelineSequenceNullableState, TimelineState } from "./timeline/type";

export function doesTimelineHasSequence(
  timelineState: TimelineSequenceNullableState | TimelineState,
): timelineState is TimelineState {
  return (
    timelineState.title !== undefined && timelineState.tracks !== undefined
  );
}
