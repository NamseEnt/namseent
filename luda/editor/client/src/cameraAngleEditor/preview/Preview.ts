import {
  Clip,
  ColorUtil,
  Convert,
  Image,
  ImageFit,
  Rect,
  Render,
  Translate,
} from "namui";
import { CameraAngleEditorState } from "../type";

export const Preview: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
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
                  Convert.xywhToCanvasKit(state.cameraAngle.destRect),
                ),
                clipOp: CanvasKit.ClipOp.Intersect,
              },
              [
                Image({
                  position: state.cameraAngle.sourceRect,
                  size: state.cameraAngle.sourceRect,
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
