import { History } from "./History";

export function getUndoableCount(history: History<any>): number {
  return history.currentIndex;
}
