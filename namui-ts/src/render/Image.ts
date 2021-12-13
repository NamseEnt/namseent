import { Paint } from "canvaskit-wasm";
import { RenderingData } from "..";
import { ImageDrawCommand, ImageFit } from "../type";

export function Image({
  position: { x, y },
  size,
  url,
  style,
}: {
  position: {
    x: number;
    y: number;
  };
  size: {
    width: number;
    height: number;
  };
  style: {
    fit: ImageFit;
    paint?: Paint;
  };
  url: string;
}): RenderingData {
  const imageDrawCommand: ImageDrawCommand = {
    type: "image",
    x,
    y,
    url,
    size,
    fit: style.fit,
    paint: style.paint,
  };

  return {
    drawCalls: [
      {
        commands: [imageDrawCommand],
      },
    ],
  };
}
