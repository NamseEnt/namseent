import { Vector, RenderingData, DrawCommand, RenderingTree } from "../../type";
import { visitRenderingTreeWithVector } from "./visitRenderingTree";

export type RenderingDataWithVector = {
  renderingData: RenderingData;
  translated: Vector;
};

export function getInOutRenderingDataLists(
  renderingTree: RenderingTree,
  vector: Vector,
): {
  inners: RenderingDataWithVector[];
  outers: RenderingDataWithVector[];
} {
  const vectorInRenderingDataList: RenderingDataWithVector[] = [];
  const vectorOutRenderingDataList: RenderingDataWithVector[] = [];

  visitRenderingTreeWithVector(
    renderingTree,
    vector,
    (node, localVector, isClipped) => {
      if (!node || node instanceof Array || "type" in node) {
        return;
      }

      const renderingData: RenderingData = node;

      const listToPush =
        !isClipped && isVectorInRenderingData(renderingData, localVector)
          ? vectorInRenderingDataList
          : vectorOutRenderingDataList;

      listToPush.push({
        renderingData,
        translated: localVector,
      });
    },
    false,
  );

  return {
    inners: vectorInRenderingDataList,
    outers: vectorOutRenderingDataList,
  };
}

function isVectorInRenderingData(
  renderingData: RenderingData,
  vector: Vector,
): boolean {
  return renderingData.drawCalls.some((drawCall) => {
    // TODO : Handle drawCall.clip
    return drawCall.commands.some((drawCommand) => {
      return isVectorInDrawCommand(drawCommand, vector);
    });
  });
}

function isVectorInDrawCommand(
  drawCommand: DrawCommand,
  vector: Vector,
): boolean {
  switch (drawCommand.type) {
    case "path": {
      const { paint } = drawCommand;
      const path = drawCommand.path.copy();
      try {
        if (path.contains(vector.x, vector.y)) {
          return true;
        }

        const stroked = path.stroke({
          cap: paint.getStrokeCap(),
          join: paint.getStrokeJoin(),
          width: paint.getStrokeWidth(),
          miter_limit: paint.getStrokeMiter(),
        });
        // NOTE : I'm not sure this is right way to handle stroke information of paint.
        if (!stroked) {
          return false;
        }
        const result = stroked.contains(vector.x, vector.y);
        return result;
      } finally {
        path?.delete();
      }
    }
    case "image": {
      return (
        drawCommand.x <= vector.x &&
        vector.x <= drawCommand.x + drawCommand.size.width &&
        drawCommand.y <= vector.y &&
        vector.y <= drawCommand.y + drawCommand.size.height
      );
    }
    // TODO : Support for text
  }
  return false;
}
