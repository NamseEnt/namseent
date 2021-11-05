import {
  Render,
  WhSize,
  XywhRect,
  ColorUtil,
  Rect,
  TextAlign,
  TextBaseline,
  FontWeight,
  Language,
  PathDrawCommand,
  Text,
} from "namui";
import { ImageBrowserState } from "./type";

export const BackButton: Render<
  ImageBrowserState,
  { itemSize: WhSize; thumbnailRect: XywhRect }
> = (state, props) => {
  const { itemSize, thumbnailRect } = props;

  const arrowPath = new CanvasKit.Path();
  arrowPath.moveTo(0, 0.5);
  arrowPath.lineTo(0.5, 0);
  arrowPath.lineTo(0.5, 0.25);
  arrowPath.lineTo(1, 0.25);
  arrowPath.lineTo(1, 0.75);
  arrowPath.lineTo(0.5, 0.75);
  arrowPath.lineTo(0.5, 1);
  arrowPath.lineTo(0, 0.5);
  arrowPath.transform(
    CanvasKit.Matrix.scaled(thumbnailRect.width, thumbnailRect.height),
  );
  arrowPath.transform(
    CanvasKit.Matrix.translated(thumbnailRect.x, thumbnailRect.y),
  );

  const arrowPaint = new CanvasKit.Paint();
  arrowPaint.setColor(ColorUtil.Black);
  arrowPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  arrowPaint.setStrokeWidth(2);

  return [
    Rect({
      x: 0,
      y: 0,
      ...itemSize,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
        },
        round: {
          radius: 5,
        },
        fill: {
          color: ColorUtil.White,
        },
      },
      onClick: () => {
        const chunks = state.key.split("-");
        chunks.pop();
        state.key = chunks.join("-");
      },
    }),
    Text({
      x: itemSize.width / 2,
      y: itemSize.height - 20,
      text: "Back",
      align: TextAlign.center,
      baseline: TextBaseline.top,
      fontType: {
        fontWeight: FontWeight.regular,
        language: Language.ko,
        serif: false,
        size: 16,
      },
      style: {
        color: ColorUtil.Black,
      },
    }),
    {
      drawCalls: [
        {
          commands: [
            PathDrawCommand({
              path: arrowPath,
              paint: arrowPaint,
            }),
          ],
        },
      ],
    },
  ];
};
