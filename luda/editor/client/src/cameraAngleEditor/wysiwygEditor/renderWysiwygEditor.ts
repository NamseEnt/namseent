import { RenderingTree } from "namui";
import { CameraAngleEditorState } from "../type";
import { renderCropRect } from "./renderCropRect";
import { renderImageRect } from "./renderImageRect";

export function renderWysiwygEditor(
  state: CameraAngleEditorState,
): RenderingTree {
  return [renderImageRect(state), renderCropRect(state)];
}
