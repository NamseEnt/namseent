import { ClipOp, Path } from "canvaskit-wasm";
import { RenderingTree } from "../type";
import { ClipCommand } from "../types/SpecialRenderingCommand";

export function Clip(
  {
    path,
    clipOp,
  }: {
    path: Path;
    clipOp: ClipOp;
  },
  renderingTree: RenderingTree,
): ClipCommand {
  return {
    type: "clip",
    path,
    clipOp,
    renderingTree,
  };
}
