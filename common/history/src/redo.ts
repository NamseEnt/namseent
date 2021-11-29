import { getRedoableCount } from "./getRedoableCount";
import { History } from "./History";
export function redo<TState>(history: History<TState>, count: number): void {
  if (getRedoableCount(history) < count) {
    throw new Error(`Cannot redo more than ${count}`);
  }
  history.currentIndex += count;
}
