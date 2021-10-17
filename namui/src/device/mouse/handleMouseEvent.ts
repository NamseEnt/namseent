import { MouseEvent, EngineContext, Vector, RenderingData } from "../../type";
import { getInOutRenderingDataLists } from "./getInOutRenderingDataLists";

type EventHandlers<TTarget, TEventType> = {
  [P in keyof TTarget as TTarget[P] extends ((event: any) => void) | undefined
    ? Parameters<NonNullable<TTarget[P]>>[0] extends TEventType
      ? P
      : never
    : never]: TTarget[P];
};
type EventHandlerNames<TEventType> = keyof Required<
  EventHandlers<RenderingData, TEventType>
>;

export function handleMouseEvent(
  context: EngineContext,
  event: MouseEvent,
  handlerName: EventHandlerNames<MouseEvent>,
) {
  const { lastRenderedTree: renderingTree } = context;
  if (!renderingTree) {
    return;
  }

  const clickVector = new Vector(event.x, event.y);

  const { in: clickInRenderingDataList } = getInOutRenderingDataLists(
    renderingTree,
    clickVector,
  );

  clickInRenderingDataList.forEach((x) => {
    const handler = x[handlerName];
    if (!handler) {
      return;
    }
    handler(event);
  });
}
