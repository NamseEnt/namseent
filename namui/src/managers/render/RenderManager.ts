import { RenderingTree, Vector } from "../..";
import { getInOutRenderingDataLists } from "../../device/mouse/getInOutRenderingDataLists";
import { IManagerInternal } from "../IManager";
import { IRenderManager } from "./IRenderManager";

export class RenderManager implements IRenderManager, IManagerInternal {
  private lastRenderingTree?: RenderingTree;

  resetBeforeRender?: () => void;
  destroy?: () => void;
  afterRender(renderingTree: RenderingTree): void {
    this.lastRenderingTree = renderingTree;
  }

  isGlobalVectorOutOfRenderingData(
    globalVector: Vector,
    renderingDataId: string,
  ): boolean {
    const { lastRenderingTree } = this;
    if (!lastRenderingTree) {
      return false;
    }

    const { inners } = getInOutRenderingDataLists(
      lastRenderingTree,
      globalVector,
    );
    return inners.every((inner) => inner.renderingData.id !== renderingDataId);
  }
}
