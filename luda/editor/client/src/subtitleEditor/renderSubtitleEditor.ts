import { Clip, ColorUtil, Rect, RenderingTree, Translate } from "namui";
import { renderPreview } from "./renderPreview";
import { renderPropertyEditor } from "./renderPropertyEditor/renderPropertyEditor";
import { SubtitleEditorState } from "./type";

export function renderSubtitleEditor(
  state: SubtitleEditorState,
): RenderingTree {
  const borderWidth = 1;
  const margin = 8;
  return [
    Translate(
      state.layout.rect,
      Clip(
        {
          path: new CanvasKit.Path().addRect(
            CanvasKit.XYWHRect(
              -borderWidth,
              -borderWidth,
              state.layout.rect.width + 2 * borderWidth,
              state.layout.rect.height + 2 * borderWidth,
            ),
          ),
          clipOp: CanvasKit.ClipOp.Intersect,
        },
        [
          Rect({
            x: 0,
            y: 0,
            width: state.layout.rect.width,
            height: state.layout.rect.height,
            style: {
              stroke: {
                color: ColorUtil.Black,
                width: 1,
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
                width: state.layout.rect.width - 2 * margin,
                height: 160,
              },
            },
            subtitle: state.subtitle,
            videoSize: state.layout.videoSize,
          }),
          renderPropertyEditor(
            {
              layout: {
                x: margin,
                y: 2 * margin + 160,
                width: state.layout.rect.width - 2 * margin,
              },
            },
            state,
          ),
        ],
      ),
    ),
  ];
}
