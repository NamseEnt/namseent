import produce, { castImmutable } from "immer";
import { History } from "./History";

/**
 *
 * @param state state must be immutable because createHistory will freeze it.
 */
export function createHistory<TState>(state: TState): History<TState> {
  const freezedState = castImmutable(produce(state, () => {}));
  return {
    currentIndex: 0,
    states: [freezedState],
  };
}
