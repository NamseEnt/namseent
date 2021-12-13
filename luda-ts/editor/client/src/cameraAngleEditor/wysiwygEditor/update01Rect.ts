import { WhSize, XywhRect } from "namui";
import { CameraAngleEditorState } from "../type";

export function update01Rect(
  state: CameraAngleEditorState,
  target01Rect: XywhRect,
  nextRect: XywhRect,
): void {
  const containerSize: WhSize = state.layout.sub.wysiwygEditor;

  target01Rect.x = nextRect.x / containerSize.width;
  target01Rect.y = nextRect.y / containerSize.height;
  target01Rect.width = nextRect.width / containerSize.width;
  target01Rect.height = nextRect.height / containerSize.height;
}
