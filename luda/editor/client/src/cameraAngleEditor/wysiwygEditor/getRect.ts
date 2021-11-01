import { XywhRect } from "namui";
import { CameraAngleEditorState } from "../type";

export function getDestRect(state: CameraAngleEditorState): XywhRect {
  return {
    x: state.layout.sub.wysiwygEditor.width * state.cameraAngle.dest01Rect.x,
    y: state.layout.sub.wysiwygEditor.height * state.cameraAngle.dest01Rect.y,
    width:
      state.layout.sub.wysiwygEditor.width * state.cameraAngle.dest01Rect.width,
    height:
      state.layout.sub.wysiwygEditor.height *
      state.cameraAngle.dest01Rect.height,
  };
}

export function getSourceRect(state: CameraAngleEditorState): XywhRect {
  return {
    x: state.layout.sub.wysiwygEditor.width * state.cameraAngle.source01Rect.x,
    y: state.layout.sub.wysiwygEditor.height * state.cameraAngle.source01Rect.y,
    width:
      state.layout.sub.wysiwygEditor.width *
      state.cameraAngle.source01Rect.width,
    height:
      state.layout.sub.wysiwygEditor.height *
      state.cameraAngle.source01Rect.height,
  };
}
