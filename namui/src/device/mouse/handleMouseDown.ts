import { MouseEvent, EngineContext, Vector } from "../../type";
import { getInOutRenderingDataLists } from "./getInOutRenderingDataLists";

export function handleMouseDown(context: EngineContext, event: MouseEvent) {
  const { lastRenderedTree: renderingTree } = context;
  if (!renderingTree) {
    return;
  }

  const clickVector = new Vector(event.x, event.y);

  const { in: clickInRenderingDataList } = getInOutRenderingDataLists(
    renderingTree,
    clickVector,
  );

  clickInRenderingDataList.forEach(
    (x) => x.onMouseDown && x.onMouseDown(event),
  );
}
