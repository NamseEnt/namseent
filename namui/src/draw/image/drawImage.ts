import { ColorUtil } from "../..";
import { engine } from "../../engine/engine";
import { EngineContext, ImageDrawCommand } from "../../type";
import { getRectsInFit } from "./getRectsInFit";

export function drawImage(
  engineContext: EngineContext,
  command: ImageDrawCommand,
): void {
  const { canvas } = engineContext;
  const { x, y, url, size, fit } = command;
  const { imageLoader } = engine;
  const image = imageLoader.tryLoad(url);
  if (!image) {
    return;
  }
  const imageInfo = image.getImageInfo();

  if (
    [size.width, size.height, imageInfo.width, imageInfo.height].includes(0)
  ) {
    return;
  }
  const { srcRect, destRect } = getRectsInFit({
    fit,
    imageSize: imageInfo,
    commandRect: {
      ...size,
      x,
      y,
    },
  });

  const paint = command.paint ?? new CanvasKit.Paint();
  const didCreatePaint = !command.paint;

  if (didCreatePaint) {
    paint.setStyle(CanvasKit.PaintStyle.Fill);
    paint.setColor(ColorUtil.Grayscale01(0.5));
  }

  canvas.drawImageRect(image, srcRect, destRect, paint);

  if (didCreatePaint) {
    paint.delete();
  }
}
