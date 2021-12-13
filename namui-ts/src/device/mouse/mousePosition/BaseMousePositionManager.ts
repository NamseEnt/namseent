import { RenderingTree } from "../../..";
import { Vector } from "../../../type";
import { IManagerInternal } from "../../../managers/IManager";
import { getInOutRenderingDataLists } from "../getInOutRenderingDataLists";
import { IMousePositionManager } from "./IMousePositionManager";

export abstract class BaseMousePositionManager
  implements IMousePositionManager, IManagerInternal
{
  public mousePosition: Vector = new Vector(0, 0);
  afterRender(renderingTree: RenderingTree) {
    const { inners } = getInOutRenderingDataLists(
      renderingTree,
      this.mousePosition,
    );

    inners.forEach((renderingDataAndVector) => {
      renderingDataAndVector.renderingData.onMouseIn?.();
    });
  }
}
