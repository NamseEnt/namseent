import { MouseEvent, EngineContext, Vector } from "../../type";
import { getInOutRenderingDataLists } from "./getInOutRenderingDataLists";

export function handleMouseClick(context: EngineContext, event: MouseEvent) {
  const { lastRenderedTree: renderingTree } = context;
  if (!renderingTree) {
    return;
  }

  const clickVector = new Vector(event.x, event.y);

  const { in: clickInRenderingDataList, out: clickOutRenderingDataList } =
    getInOutRenderingDataLists(renderingTree, clickVector);

  clickInRenderingDataList.forEach((x) => x.onClick && x.onClick(event));
  clickOutRenderingDataList.forEach((x) => x.onClickOut && x.onClickOut(event));
}
