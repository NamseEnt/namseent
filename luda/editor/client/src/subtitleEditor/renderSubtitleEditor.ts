import {
  BorderPosition,
  Clip,
  ColorUtil,
  Rect,
  Render,
  Translate,
} from "namui";
import { isSubtitleClip } from "../clipTypeGuard";
import { TimelineState } from "../timeline/type";
import { renderPreview } from "./renderPreview";
import { renderPropertyEditor } from "./renderPropertyEditor/renderPropertyEditor";
import { SubtitleEditorState } from "./type";

export const renderSubtitleEditor: Render<
  {
    subtitleEditor: SubtitleEditorState;
    timeline: TimelineState;
  },
  {}
> = (state, props) => {
  if (
    !state.timeline.selectedClip ||
    !isSubtitleClip(state.timeline.selectedClip)
  ) {
    return;
  }

  const borderWidth = 1;
  const margin = 8;
  return [
    Translate(
      state.subtitleEditor.layout.rect,
      Clip(
        {
          path: new CanvasKit.Path().addRect(
            CanvasKit.XYWHRect(
              -borderWidth,
              -borderWidth,
              state.subtitleEditor.layout.rect.width + 2 * borderWidth,
              state.subtitleEditor.layout.rect.height + 2 * borderWidth,
            ),
          ),
          clipOp: CanvasKit.ClipOp.Intersect,
        },
        [
          Rect({
            x: 0,
            y: 0,
            width: state.subtitleEditor.layout.rect.width,
            height: state.subtitleEditor.layout.rect.height,
            style: {
              stroke: {
                color: ColorUtil.Black,
                width: 1,
                borderPosition: BorderPosition.inside,
              },
              fill: {
                color: ColorUtil.White,
              },
            },
          }),
          renderPreview({
            layout: {
              rect: {
                x: margin,
                y: margin,
                width: state.subtitleEditor.layout.rect.width - 2 * margin,
                height: 160,
              },
            },
            subtitle: state.subtitleEditor.subtitle,
            videoSize: state.subtitleEditor.layout.videoSize,
          }),
          renderPropertyEditor(
            {
              layout: {
                x: margin,
                y: 2 * margin + 160,
                width: state.subtitleEditor.layout.rect.width - 2 * margin,
              },
            },
            state.subtitleEditor,
          ),
        ],
      ),
    ),
  ];
};
