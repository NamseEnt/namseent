import { Immutable } from "immer";

export type History<TState> = {
  currentIndex: number;
  states: Immutable<TState>[];
};
