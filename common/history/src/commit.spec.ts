import { createHistory } from "./createHistory";
import { commit } from "./commit";

describe("commit", () => {
  test("should be failed if updateState is failed", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    const result = commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: false,
        error: "failed",
      };
    });
    expect(result).toEqual({
      isSuccessful: false,
      error: "failed",
    });
  });

  test("should be successful if updateState is successful", () => {
    const state = { a: 1 };
    const history = createHistory(state);
    const result = commit(history, (draft) => {
      draft.a = 2;
      return {
        isSuccessful: true,
      };
    });
    expect(result).toEqual({
      isSuccessful: true,
      result: history,
    });
  });
});
