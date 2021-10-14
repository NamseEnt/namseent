import { createContext } from "react";
import { initialState, State } from "./State";

export type Context = [
  state: State,
  update: (updater: (state: State) => void) => void,
];

export const context = createContext<Context>([
  initialState,
  () => {
    throw new Error("Update function not implemented. Check provider");
  },
]);
