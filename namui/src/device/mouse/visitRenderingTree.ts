import { RenderingTree } from "../..";
import { Vector } from "../../type";

export function visitRenderingTreeWithVector(
  renderingTree: RenderingTree,
  vector: Vector,
  callback: (node: RenderingTree, localVector: Vector) => void,
): void {
  if (!(renderingTree instanceof Array)) {
    renderingTree = [renderingTree];
  }

  renderingTree.forEach((element) => {
    callback(element, vector);
    if (!element) {
      return;
    }
    if (element instanceof Array) {
      return visitRenderingTreeWithVector(element, vector, callback);
    }
    if ("type" in element) {
      switch (element.type) {
        case "translate":
          return visitRenderingTreeWithVector(
            element.renderingTree,
            vector.translate(-element.x, -element.y),
            callback,
          );
        case "clip":
          return visitRenderingTreeWithVector(
            element.renderingTree,
            vector,
            callback,
          );
      }
    }
  });
}
