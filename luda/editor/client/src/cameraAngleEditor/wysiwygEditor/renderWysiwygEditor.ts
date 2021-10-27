import { RenderingTree } from "namui";
import { CameraAngleEditorState } from "../type";
import { renderCropRect } from "./renderCropRect";

export function renderWysiwygEditor(
  state: CameraAngleEditorState,
): RenderingTree {
  return [renderImageRect(state), renderCropRect(state)];
}

function renderImageRect(state: CameraAngleEditorState): RenderingTree {
  return;
}
