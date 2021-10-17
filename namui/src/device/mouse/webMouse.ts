import { engineInternal } from "../../engine/engine";
import { MouseEvent as NamuiMouseEvent } from "../../type";

export function toNamuiMouseEvent(mouseEvent: MouseEvent): NamuiMouseEvent {
  return {
    x: mouseEvent.offsetX,
    y: mouseEvent.offsetY,
    button: mouseEvent.button,
    isLeftButtonDown: engineInternal.mouseButton.isLeftMouseButtonDown,
    isRightButtonDown: engineInternal.mouseButton.isRightMouseButtonDown,
  };
}
