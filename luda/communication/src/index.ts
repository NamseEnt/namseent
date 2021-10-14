import { startEngine, Language } from "namui";
import { render } from "./render";

startEngine(
  {
    counters: [],
    communication: {
      shotIndex: 0,
      language: Language.ko,
    },
  },
  render,
  {
    hotReload: {
      buildServerUrl: "ws://localhost:8080",
    },
  },
);
