import { RenderingTree } from "namui";
import { renderCameraAngleEditor } from "./cameraAngleEditor/renderCameraAngleEditor";
import { CameraAngleEditorState } from "./cameraAngleEditor/type";
import { renderImageEditor } from "./imageEditor/renderImageEditor";
import { ImageEditorState } from "./imageEditor/type";
import { renderSubtitleEditor } from "./subtitleEditor/renderSubtitleEditor";
import { SubtitleEditorState } from "./subtitleEditor/type";
import { renderTimeline } from "./timeline/renderTimeline";
import { TimelineState } from "./timeline/type";

type State = {
  imageEditorState: ImageEditorState;
  timelineState: TimelineState;
  cameraAngleEditorState: CameraAngleEditorState;
  subtitleEditorState: SubtitleEditorState;
};

export function render(state: State): RenderingTree {
  // return renderImageEditor(state.imageEditorState);
  // return renderTimeline(state.timelineState);
  // return renderCameraAngleEditor(state.cameraAngleEditorState);
  return renderSubtitleEditor(state.subtitleEditorState);
}
