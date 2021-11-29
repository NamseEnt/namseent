import { History } from "./History";

export function getRedoableCount(history: History<any>): number {
  return history.states.length - history.currentIndex - 1;
}
