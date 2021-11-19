import { Cursor } from "../../../type";
import { IManagerInternal } from "../../../managers/IManager";
import { IMousePointerManager } from "./IMousePointerManager";

export class WebMousePointerManager
  implements IMousePointerManager, IManagerInternal
{
  destroy(): void {
    this.setCursor(Cursor.default);
  }
  resetBeforeRender(): void {
    this.setCursor(Cursor.default);
  }
  setCursor(cursor: Cursor): void {
    document.body.style.cursor = this.cursorToCssCursorValue(cursor);
  }
  private cursorToCssCursorValue(cursor: Cursor): string {
    switch (cursor) {
      case Cursor.default:
        return "default";
      case Cursor.topBottomResize:
        return "ns-resize";
      case Cursor.leftRightResize:
        return "ew-resize";
      case Cursor.leftTopRightBottomResize:
        return "nwse-resize";
      case Cursor.rightTopLeftBottomResize:
        return "nesw-resize";
      case Cursor.text:
        return "text";
      case Cursor.grab:
        return "grab";
      case Cursor.move:
        return "move";
      case Cursor.pointer:
        return "pointer";
      default:
        throw new Error(`Unknown cursor ${cursor}`);
    }
  }
}
