import {
  MouseEvent,
  EngineContext,
  Vector,
  RenderingData,
  MouseEventExceptTranslated,
} from "../../type";
import {
  getInOutRenderingDataLists,
  RenderingDataWithVector,
} from "./getInOutRenderingDataLists";

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
  event: MouseEventExceptTranslated,
  inHandlerName: EventHandlerNames<MouseEvent>,
  outHandlerName?: EventHandlerNames<MouseEvent>,
) {
  const { lastRenderedTree: renderingTree } = context;
  if (!renderingTree) {
    return;
  }

  const mouseVector = new Vector(event.x, event.y);

  const { inners, outers } = getInOutRenderingDataLists(
    renderingTree,
    mouseVector,
  );

  (
    [
      [inners, inHandlerName],
      [outers, outHandlerName],
    ] as const
  ).forEach(([renderingDataList, handlerName]) => {
    if (!handlerName) {
      return;
    }
    invokeHandler(renderingDataList, event, handlerName);
  });
}

function invokeHandler(
  renderingDataWithVectors: RenderingDataWithVector[],
  event: MouseEventExceptTranslated,
  handlerName: EventHandlerNames<MouseEvent>,
): void {
  renderingDataWithVectors.forEach(({ renderingData, translated }) => {
    const handler = renderingData[handlerName];
    if (!handler) {
      return;
    }
    handler({
      ...event,
      translated,
    });
  });
}
