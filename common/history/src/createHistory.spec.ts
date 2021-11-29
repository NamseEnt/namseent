import { createHistory } from "./createHistory";
import { getCurrentState } from "./getCurrentState";
import { redo } from "./redo";
import { undo } from "./undo";

describe("createHistory", () => {
  test("should freeze original state", () => {
    const state = { a: 1 };
    expect(() => {
      state.a = 2;
    }).not.toThrow();
    const history = createHistory(state);
    expect(() => {
      state.a = 3;
    }).toThrow();
    const nextState = getCurrentState(history);
    expect(nextState).toEqual({
      a: 2,
    });
  });
});
