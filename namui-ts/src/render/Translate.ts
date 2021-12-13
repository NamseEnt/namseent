import { RenderingTree } from "../type";
import { TranslateCommand } from "../types/SpecialRenderingCommand";

export function Translate(
  matrix: {
    x: number;
    y: number;
  },
  renderingTree: RenderingTree,
): TranslateCommand {
  return {
    type: "translate",
    ...matrix,
    renderingTree,
  };
}
