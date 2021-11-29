import { History } from "./History";
import { Result } from "types";
import { getCurrentState } from "./getCurrentState";
import produce, { Draft, Immutable } from "immer";

export function commit<TState, TError>(
  history: History<TState>,
  updateState: (state: Draft<Immutable<TState>>) => Result<void, TError>,
): Result<History<TState>, TError> {
  const currentState = getCurrentState(history);

  let result: Result<void, TError> | undefined;
  const nextState = produce(currentState, (draft) => {
    result = updateState(draft);
  });

  if (!result) {
    throw new Error("produce doesn't called updateState");
  }
  if (!result.isSuccessful) {
    return result;
  }

  history.states.push(nextState);
  history.currentIndex += 1;
  return { isSuccessful: true, result: history };
}
