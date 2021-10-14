import { RenderingTree } from "namui";
import { renderImageEditor } from "./imageEditor/renderImageEditor";
import { ImageEditorState } from "./imageEditor/type";
import { renderTimeline } from "./timeline/renderTimeline";
import { TimelineState } from "./timeline/type";

type State = {
  imageEditorState: ImageEditorState;
  timelineState: TimelineState;
};

export function render(state: State): RenderingTree {
  // return renderImageEditor(state.imageEditorState);
  return renderTimeline(state.timelineState);
}
