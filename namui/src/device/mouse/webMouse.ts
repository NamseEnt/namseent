import { engineInternal } from "../../engine/engine";
import { MouseEventExceptTranslated } from "../../type";

export function toNamuiMouseEvent(
  mouseEvent: MouseEvent,
): MouseEventExceptTranslated {
  return {
    x: mouseEvent.offsetX,
    y: mouseEvent.offsetY,
    button: mouseEvent.button,
    isLeftButtonDown: engineInternal.mouseButton.isLeftMouseButtonDown,
    isRightButtonDown: engineInternal.mouseButton.isRightMouseButtonDown,
  };
}
