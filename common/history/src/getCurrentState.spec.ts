import { createHistory } from "./createHistory";
import { getCurrentState } from "./getCurrentState";
import { redo } from "./redo";
import { undo } from "./undo";
import { commit } from "./commit";

describe("getCurrentState", () => {
  test("Current state of Just created history should be what you pass on creating", () => {
    const state = {};
    const history = createHistory(state);
    const currentState = getCurrentState(history);
    expect(currentState).toBe(state);
  });

  test("Commit should change current state as committed one", () => {
    const state = {
      a: 1,
    };
    const history = createHistory(state);
    commit(history, (state) => {
      state.a = 2;
      return {
        isSuccessful: true,
      };
    });
    const currentState = getCurrentState(history);
    expect(currentState).toEqual({
      a: 2,
    });
  });

  test("Undo should change current state as previous one", () => {
    const state = {
      a: 1,
    };
    const history = createHistory(state);
    commit(history, (state) => {
      state.a = 2;
      return {
        isSuccessful: true,
      };
    });
    undo(history, 1);
    const currentState = getCurrentState(history);
    expect(currentState).toEqual({
      a: 1,
    });
  });

  test("Redo should change current state as next one", () => {
    const state = {
      a: 1,
    };
    const history = createHistory(state);
    commit(history, (state) => {
      state.a = 2;
      return {
        isSuccessful: true,
      };
    });
    undo(history, 1);
    redo(history, 1);
    const currentState = getCurrentState(history);
    expect(currentState).toEqual({
      a: 2,
    });
  });
});
