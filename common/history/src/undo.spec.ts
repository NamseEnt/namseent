import { createHistory } from "./createHistory";
import { undo } from "./undo";
import { commit } from "./commit";

describe("undo", () => {
  test("Should throw error if undo more than undo-ables", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: true,
      };
    });
    expect(() => {
      undo(history, 2);
    }).toThrow();
  });
  test("Should not throw error if undo less than undo-ables", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: true,
      };
    });
    expect(() => {
      undo(history, 1);
    }).not.toThrow();
  });
});
