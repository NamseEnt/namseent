import {
  Clip,
  ColorUtil,
  Convert,
  Image,
  ImageFit,
  Rect,
  Render,
  Translate,
  BorderPosition,
} from "namui";
import { CameraAngleEditorState } from "../type";
import { getDestRect, getSourceRect } from "../wysiwygEditor/getRect";

export const Preview: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  const sourceRect = getSourceRect(state);
  const destRect = getDestRect(state);

  return [
    Translate(
      {
        ...state.layout.sub.preview,
      },
      [
        Rect({
          ...state.layout.sub.preview,
          x: 0,
          y: 0,
          style: {
            stroke: {
              color: ColorUtil.Black,
              width: 1,
              borderPosition: BorderPosition.inside,
            },
          },
        }),
        Clip(
          {
            path: new CanvasKit.Path().addRect(
              CanvasKit.XYWHRect(
                0,
                0,
                state.layout.sub.preview.width,
                state.layout.sub.preview.height,
              ),
            ),
            clipOp: CanvasKit.ClipOp.Intersect,
          },
          [
            Clip(
              {
                path: new CanvasKit.Path().addRect(
                  Convert.xywhToCanvasKit(destRect),
                ),
                clipOp: CanvasKit.ClipOp.Intersect,
              },
              [
                Image({
                  position: sourceRect,
                  size: sourceRect,
                  url: state.cameraAngle.imageSourceUrl,
                  style: {
                    fit: ImageFit.fill,
                  },
                }),
              ],
            ),
          ],
        ),
      ],
    ),
  ];
};
