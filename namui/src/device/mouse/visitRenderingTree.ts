import { RenderingTree } from "../..";
import { Vector } from "../../type";

export function visitRenderingTreeWithVector(
  renderingTree: RenderingTree,
  vector: Vector,
  callback: (
    node: RenderingTree,
    localVector: Vector,
    isClipped: boolean,
  ) => void,
  isClipped: boolean,
): void {
  if (!(renderingTree instanceof Array)) {
    renderingTree = [renderingTree];
  }

  renderingTree.forEach((element) => {
    callback(element, vector, isClipped);
    if (!element) {
      return;
    }
    if (element instanceof Array) {
      return visitRenderingTreeWithVector(element, vector, callback, isClipped);
    }
    if ("type" in element) {
      switch (element.type) {
        case "translate":
          return visitRenderingTreeWithVector(
            element.renderingTree,
            vector.translate(-element.x, -element.y),
            callback,
            isClipped,
          );
        case "clip":
          const isPathContainsVector = element.path.contains(
            vector.x,
            vector.y,
          );

          const isVectorFilteredByClip =
            element.clipOp === CanvasKit.ClipOp.Intersect
              ? !isPathContainsVector
              : isPathContainsVector;

          return visitRenderingTreeWithVector(
            element.renderingTree,
            vector,
            callback,
            isClipped || isVectorFilteredByClip,
          );
      }
    }
  });
}
