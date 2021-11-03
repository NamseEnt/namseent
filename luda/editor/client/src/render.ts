import { RenderingTree } from "namui";
import { CameraAngleEditorState } from "./cameraAngleEditor/type";
import { ImageEditorState } from "./imageEditor/type";
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
  return renderTimeline(state.timelineState);
}
