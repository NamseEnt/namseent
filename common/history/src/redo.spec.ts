import { createHistory } from "./createHistory";
import { redo } from "./redo";
import { commit } from "./commit";
import { undo } from "./undo";

describe("redo", () => {
  test("Should throw error if redo more than redo-ables", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: true,
      };
    });
    undo(history, 1);
    expect(() => {
      redo(history, 2);
    }).toThrow();
  });
  test("Should not throw error if redo less than redo-ables", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: true,
      };
    });
    undo(history, 1);
    expect(() => {
      redo(history, 1);
    }).not.toThrow();
  });
});
