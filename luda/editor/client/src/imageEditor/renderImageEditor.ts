import { RenderingTree } from "namui";
import { renderImageList } from "./renderImageList";
import { ImageEditorState } from "./type";

export function renderImageEditor(
  imageEditorState: ImageEditorState,
): RenderingTree {
  return renderImageList(imageEditorState);
}
