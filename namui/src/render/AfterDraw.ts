import {
  AfterDrawCallback,
  AfterDrawCommand,
} from "../types/SpecialRenderingCommand";

export function AfterDraw(callback: AfterDrawCallback): AfterDrawCommand {
  return {
    type: "afterDraw",
    callback,
  };
}
