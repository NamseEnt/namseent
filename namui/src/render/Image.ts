import { ImageDrawCommand, RenderingTree, ImageFit } from "../type";

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
  };
  url: string;
}): RenderingTree {
  const imageDrawCommand: ImageDrawCommand = {
    type: "image",
    x,
    y,
    url,
    size,
    fit: style.fit,
  };

  return {
    drawCalls: [
      {
        commands: [imageDrawCommand],
      },
    ],
  };
}
