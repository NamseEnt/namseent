import { Clip, Convert, Image, ImageFit, Render, WhSize } from "namui";
import { CameraAngle } from "../../../type";

export const CameraClip: Render<
  {},
  { cameraAngle: CameraAngle; whSize: WhSize }
> = (state, props) => {
  const {
    cameraAngle,
    whSize: { width, height },
  } = props;
  const { source01Rect, dest01Rect, imageSourceUrl } = cameraAngle;

  const sourceRect = {
    x: source01Rect.x * width,
    y: source01Rect.y * height,
    width: source01Rect.width * width,
    height: source01Rect.height * height,
  };
  console.log(sourceRect);
  const destRect = {
    x: dest01Rect.x * width,
    y: dest01Rect.y * height,
    width: dest01Rect.width * width,
    height: dest01Rect.height * height,
  };

  return [
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(0, 0, width, height),
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
              url: imageSourceUrl,
              style: {
                fit: ImageFit.fill,
              },
            }),
          ],
        ),
      ],
    ),
  ];
};
