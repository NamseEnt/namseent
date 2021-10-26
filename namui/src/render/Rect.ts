import { Color } from "canvaskit-wasm";
import { MouseEventCallback, DrawCommand, RenderingTree } from "../type";

export function Rect({
  x,
  y,
  width,
  height,
  style: { stroke, fill },
  onClick,
  onClickOut,
  onMouseMoveIn,
  onMouseMoveOut,
  onMouseDown,
  onMouseUp,
}: {
  x: number;
  y: number;
  width: number;
  height: number;
  style: {
    stroke?: {
      color: Color;
      width: number;
    };
    fill?: {
      color: Color;
    };
  };
  onClick?: MouseEventCallback;
  onClickOut?: MouseEventCallback;
  onMouseMoveIn?: MouseEventCallback;
  onMouseMoveOut?: MouseEventCallback;
  onMouseDown?: MouseEventCallback;
  onMouseUp?: MouseEventCallback;
}): RenderingTree {
  const borderRectPath = new CanvasKit.Path().addRect(
    CanvasKit.XYWHRect(x, y, width, height),
  );

  const drawCommands: DrawCommand[] = [];

  if (stroke) {
    const strokePaint = new CanvasKit.Paint();
    strokePaint.setColor(stroke.color);
    strokePaint.setStrokeWidth(stroke.width);
    strokePaint.setStyle(CanvasKit.PaintStyle.Stroke);
    strokePaint.setAntiAlias(true);

    drawCommands.push({
      type: "path",
      path: borderRectPath,
      paint: strokePaint,
    });
  }

  if (fill) {
    const fillPaint = new CanvasKit.Paint();
    fillPaint.setColor(fill.color);
    fillPaint.setStyle(CanvasKit.PaintStyle.Fill);
    fillPaint.setAntiAlias(true);

    drawCommands.push({
      type: "path",
      path: borderRectPath,
      paint: fillPaint,
    });
  }

  return {
    drawCalls: [
      {
        commands: drawCommands,
      },
    ],
    onClick,
    onClickOut,
    onMouseMoveIn,
    onMouseMoveOut,
    onMouseDown,
    onMouseUp,
  };
}
