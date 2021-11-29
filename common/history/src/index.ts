import { enableAllPlugins } from "immer";
enableAllPlugins();

export { commit } from "./commit";
export { createHistory } from "./createHistory";
export { getCurrentState } from "./getCurrentState";
export { getRedoableCount } from "./getRedoableCount";
export { getUndoableCount } from "./getUndoableCount";
export { History } from "./History";
export { redo } from "./redo";
export { undo } from "./undo";
export { Immutable, Draft } from "immer";
