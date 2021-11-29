import { getUndoableCount } from "./getUndoableCount";
import { History } from "./History";

export function undo<TState>(history: History<TState>, count: number): void {
  if (getUndoableCount(history) < count) {
    throw new Error(`Cannot undo more than ${count}`);
  }
  history.currentIndex -= count;
}
