import { Immutable } from "immer";
import { History } from "./History";

export function getCurrentState<TState>(
  history: History<TState>,
): Immutable<TState> {
  const currentState = history.states[history.currentIndex];
  if (currentState === undefined) {
    console.debug(`history.currentIndex: ${history.currentIndex}`);
    throw new Error("Cannot get current state from history");
  }
  return currentState;
}
