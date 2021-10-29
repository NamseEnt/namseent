import {
  Clip,
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { renderPropertyTextEditor } from "./propertyTextEditor/renderPropertyTextEditor";
import { WysiwygEditor } from "./wysiwygEditor/WysiwygEditor";
import { CameraAngleEditorState } from "./type";

export function renderCameraAngleEditor(
  state: CameraAngleEditorState,
): RenderingTree {
  const borderWidth = 1;
  return Translate(
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
        renderPropertyTextEditor(state),
        Translate(
          {
            x: 400,
            y: 0,
          },
          WysiwygEditor(state),
        ),
      ],
    ),
  );
}
