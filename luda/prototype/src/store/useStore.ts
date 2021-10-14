import { useContext } from "react";
import { context } from "./context";

export default function useStore() {
  return useContext(context);
}
