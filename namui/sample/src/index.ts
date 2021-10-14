import { startEngine } from "namui";
import { render } from "./render";

startEngine(
  {
    text: "click and input text!",
    focus: false,
  },
  render,
  {
    hotReload: {
      buildServerUrl: "ws://localhost:8080",
    },
  },
);
