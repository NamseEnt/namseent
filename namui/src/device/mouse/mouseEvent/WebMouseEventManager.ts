import { MouseEventCallback } from "../../../type";
import { IManagerInternal } from "../../../managers/IManager";
import { toNamuiMouseEvent } from "../webMouse";
import { IMouseEventManager } from "./IMouseEventManager";
import { IMouseButtonManagerInternal } from "../mouseButton/IMouseButtonManager";

const eventNames = ["mousedown", "mouseup", "mouseout", "mousemove"] as const;

export class WebMouseEventManager
  implements IMouseEventManager, IManagerInternal
{
  private readonly eventNameCallbacksMap: {
    [key in typeof eventNames[number]]: Set<MouseEventCallback>;
  } = {
    mousedown: new Set(),
    mouseup: new Set(),
    mouseout: new Set(),
    mousemove: new Set(),
  };
  private readonly eventNameEventListenerTuples: {
    eventName: typeof eventNames[number];
    eventListener: (event: MouseEvent) => void;
  }[];

  constructor(
    private readonly mouseButtonManager: IMouseButtonManagerInternal,
  ) {
    this.eventNameEventListenerTuples = eventNames.map((eventName) => {
      const callbacks = this.eventNameCallbacksMap[eventName];
      const eventListener = (event: MouseEvent) => {
        this.handleEventForManagerFirst(eventName, event);
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
  handleEventForManagerFirst(
    eventName: typeof eventNames[number],
    event: MouseEvent,
  ) {
    switch (eventName) {
      case "mousedown":
        switch (event.button) {
          case 0:
            this.mouseButtonManager.isLeftMouseButtonDown = true;
            break;
          case 2:
            this.mouseButtonManager.isRightMouseButtonDown = true;
            break;
        }
        break;
      case "mouseup":
        switch (event.button) {
          case 0:
            this.mouseButtonManager.isLeftMouseButtonDown = false;
            break;
          case 2:
            this.mouseButtonManager.isRightMouseButtonDown = false;
            break;
        }
    }
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
  onMouseMove(callback: MouseEventCallback): void {
    this.eventNameCallbacksMap.mousemove.add(callback);
  }
  resetBeforeRender(): void {
    Object.values(this.eventNameCallbacksMap).forEach((callbacks) =>
      callbacks.clear(),
    );
  }
}
