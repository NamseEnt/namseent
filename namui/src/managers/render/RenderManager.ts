import { RenderingTree, Vector } from "../..";
import { getInOutRenderingDataLists } from "../../device/mouse/getInOutRenderingDataLists";
import { handleMouseEvent } from "../../device/mouse/handleMouseEvent";
import { IMouseEventManager } from "../../device/mouse/mouseEvent/IMouseEventManager";
import { EngineContext } from "../../type";
import { IManagerInternal } from "../IManager";
import { IRenderManager } from "./IRenderManager";

export class RenderManager implements IRenderManager, IManagerInternal {
  private lastRenderingTree?: RenderingTree;
  constructor(
    private readonly engineContext: EngineContext,
    private readonly mouseEventManager: IMouseEventManager,
  ) {}

  resetBeforeRender() {
    this.lastRenderingTree = undefined;
  }
  destroy?: () => void;
  afterRender(renderingTree: RenderingTree): void {
    this.lastRenderingTree = renderingTree;

    this.mouseEventManager.onMouseDown((event) => {
      console.log("onmousedown");
      handleMouseEvent(this.engineContext, event, "onMouseDown");
    });
    this.mouseEventManager.onMouseUp((event) => {
      handleMouseEvent(this.engineContext, event, "onMouseUp");
    });
    this.mouseEventManager.onMouseMove((event) => {
      handleMouseEvent(
        this.engineContext,
        event,
        "onMouseMoveIn",
        "onMouseMoveOut",
      );
    });
  }

  isGlobalVectorOutOfRenderingData(
    globalVector: Vector,
    renderingDataId: string,
  ): boolean {
    const { lastRenderingTree } = this;
    if (!lastRenderingTree) {
      console.warn("this function should be called after render");
      return false;
    }

    const { inners } = getInOutRenderingDataLists(
      lastRenderingTree,
      globalVector,
    );
    return inners.every((inner) => inner.renderingData.id !== renderingDataId);
  }
}
