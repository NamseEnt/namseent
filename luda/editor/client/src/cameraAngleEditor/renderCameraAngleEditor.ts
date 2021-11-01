import {
  Clip,
  ColorUtil,
  FontWeight,
  Language,
  Mathu,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
  XywhRect,
} from "namui";
import { renderPropertyTextEditor } from "./propertyTextEditor/renderPropertyTextEditor";
import { WysiwygEditor } from "./wysiwygEditor/WysiwygEditor";
import { CameraAngleEditorState } from "./type";
import { Preview } from "./preview/Preview";

export function renderCameraAngleEditor(
  state: CameraAngleEditorState,
): RenderingTree {
  const borderWidth = 1;

  keepDest01RectInView(state);

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
        WysiwygEditor(state),
        Preview(state),
      ],
    ),
  );
}

function keepDest01RectInView(state: CameraAngleEditorState): void {
  const { dest01Rect } = state.cameraAngle;
  dest01Rect.x = Mathu.clamp(dest01Rect.x, 0, 1);
  dest01Rect.y = Mathu.clamp(dest01Rect.y, 0, 1);

  dest01Rect.width = Mathu.clamp(dest01Rect.width, 0, 1 - dest01Rect.x);
  dest01Rect.height = Mathu.clamp(dest01Rect.height, 0, 1 - dest01Rect.y);

  const containerRect = state.layout.sub.wysiwygEditor;
  const minWidth = 1 / containerRect.width;
  const minHeight = 1 / containerRect.height;

  if (dest01Rect.width < minWidth) {
    dest01Rect.width = minWidth;
    if (dest01Rect.width + dest01Rect.x > 1) {
      dest01Rect.x = 1 - dest01Rect.width;
    }
  }

  if (dest01Rect.height < minHeight) {
    dest01Rect.height = minHeight;
    if (dest01Rect.height + dest01Rect.y > 1) {
      dest01Rect.y = 1 - dest01Rect.height;
    }
  }
}
