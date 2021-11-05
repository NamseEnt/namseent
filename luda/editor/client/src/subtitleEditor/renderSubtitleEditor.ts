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
  {},
  {
    subtitleEditor: SubtitleEditorState;
    timeline: TimelineState;
  }
> = (state, props) => {
  if (
    !props.timeline.selectedClip ||
    !isSubtitleClip(props.timeline.selectedClip)
  ) {
    return;
  }

  const borderWidth = 1;
  const margin = 8;
  return [
    Translate(
      props.subtitleEditor.layout.rect,
      Clip(
        {
          path: new CanvasKit.Path().addRect(
            CanvasKit.XYWHRect(
              -borderWidth,
              -borderWidth,
              props.subtitleEditor.layout.rect.width + 2 * borderWidth,
              props.subtitleEditor.layout.rect.height + 2 * borderWidth,
            ),
          ),
          clipOp: CanvasKit.ClipOp.Intersect,
        },
        [
          Rect({
            x: 0,
            y: 0,
            width: props.subtitleEditor.layout.rect.width,
            height: props.subtitleEditor.layout.rect.height,
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
                width: props.subtitleEditor.layout.rect.width - 2 * margin,
                height: 160,
              },
            },
            subtitle: props.subtitleEditor.subtitle,
            videoSize: props.subtitleEditor.layout.videoSize,
          }),
          renderPropertyEditor(
            {
              layout: {
                x: margin,
                y: 2 * margin + 160,
                width: props.subtitleEditor.layout.rect.width - 2 * margin,
              },
            },
            props.subtitleEditor,
          ),
        ],
      ),
    ),
  ];
};
