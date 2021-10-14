import { MouseEvent, EngineContext, Vector } from "../../type";
import { getInOutRenderingDataLists } from "./getInOutRenderingDataLists";

export function handleMouseInOut(context: EngineContext, event: MouseEvent) {
  const { lastRenderedTree: renderingTree } = context;
  if (!renderingTree) {
    return;
  }

  const mouseVector = new Vector(event.x, event.y);

  const { in: mouseInRenderingDataList, out: mouseOutRenderingDataList } =
    getInOutRenderingDataLists(renderingTree, mouseVector);

  mouseInRenderingDataList.forEach((x) => x.onMouseIn && x.onMouseIn(event));
  mouseOutRenderingDataList.forEach((x) => x.onMouseOut && x.onMouseOut(event));
}
