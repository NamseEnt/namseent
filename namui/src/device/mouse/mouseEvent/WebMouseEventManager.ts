import { MouseEventCallback } from "../../../type";
import { IManagerInternal } from "../../IManager";
import { toNamuiMouseEvent } from "../webMouse";
import { IMouseEventManager } from "./IMouseEventManager";

const eventNames = ["mousedown", "mouseup", "mouseout"] as const;

export class WebMouseEventManager
  implements IMouseEventManager, IManagerInternal
{
  private readonly eventNameCallbacksMap: {
    [key in typeof eventNames[number]]: Set<MouseEventCallback>;
  } = {
    mousedown: new Set(),
    mouseup: new Set(),
    mouseout: new Set(),
  };
  private readonly eventNameEventListenerTuples: {
    eventName: typeof eventNames[number];
    eventListener: (event: MouseEvent) => void;
  }[];

  constructor() {
    this.eventNameEventListenerTuples = eventNames.map((eventName) => {
      const callbacks = this.eventNameCallbacksMap[eventName];
      const eventListener = (event: MouseEvent) => {
        const namuiEventExceptTranslated = toNamuiMouseEvent(event);
        const namuiEvent = {
          ...namuiEventExceptTranslated,
          translated: {
            x: namuiEventExceptTranslated.x,
            y: namuiEventExceptTranslated.y,
          },
        };
        callbacks.forEach((callback) => callback(namuiEvent));
      };
      window.addEventListener(eventName, eventListener);
      return { eventName, eventListener };
    });
  }
  destroy(): void {
    this.eventNameEventListenerTuples.forEach((tuple) => {
      window.removeEventListener(tuple.eventName, tuple.eventListener);
    });
  }
  onMouseDown(callback: MouseEventCallback): void {
    this.eventNameCallbacksMap.mousedown.add(callback);
  }
  onMouseUp(callback: MouseEventCallback): void {
    this.eventNameCallbacksMap.mouseup.add(callback);
  }
  onMouseOut(callback: MouseEventCallback): void {
    this.eventNameCallbacksMap.mouseout.add(callback);
  }
  resetBeforeRender(): void {
    Object.values(this.eventNameCallbacksMap).forEach((callbacks) =>
      callbacks.clear(),
    );
  }
}
