import { Color, InputRect } from "canvaskit-wasm";
import { AfterDraw } from "..";
import {
  MouseEventCallback,
  DrawCommand,
  RenderingTree,
  BorderPosition,
} from "../type";
import { nanoid } from "nanoid";

export function Rect({
  x,
  y,
  width,
  height,
  id,
  style: { stroke, fill, round },
  onClick,
  onClickOut,
  onMouseIn,
  onMouseMoveIn,
  onMouseMoveOut,
  onMouseDown,
  onMouseUp,
  onAfterDraw,
}: {
  x: number;
  y: number;
  width: number;
  height: number;
  id?: string;
  style: {
    stroke?: {
      color: Color;
      width: number;
      borderPosition: BorderPosition;
    };
    fill?: {
      color: Color;
    };
    round?: {
      radius: number;
    };
  };
  onClick?: MouseEventCallback;
  onClickOut?: MouseEventCallback;
  onMouseIn?: () => void;
  onMouseMoveIn?: MouseEventCallback;
  onMouseMoveOut?: MouseEventCallback;
  onMouseDown?: MouseEventCallback;
  onMouseUp?: MouseEventCallback;
  onAfterDraw?: (id: string) => void;
}): RenderingTree {
  const renderingTree = [];
  function getRectPath(rect: InputRect) {
    const rectPath = new CanvasKit.Path();
    if (round) {
      rectPath.addRRect(CanvasKit.RRectXY(rect, round.radius, round.radius));
    } else {
      rectPath.addRect(rect);
    }
    return rectPath;
  }

  let rect: InputRect;
  if (!stroke || stroke.borderPosition === BorderPosition.outside) {
    rect = CanvasKit.XYWHRect(x, y, width, height);
  } else if (stroke.borderPosition === BorderPosition.inside) {
    rect = CanvasKit.XYWHRect(
      x + stroke.width,
      y + stroke.width,
      width - 2 * stroke.width,
      height - 2 * stroke.width,
    );
  } else {
    rect = CanvasKit.XYWHRect(
      x + stroke.width / 2,
      y + stroke.width / 2,
      width - stroke.width,
      height - stroke.width,
    );
  }

  const rectPath = getRectPath(rect);

  const drawCommands: DrawCommand[] = [];

  if (stroke) {
    const strokePaint = new CanvasKit.Paint();
    strokePaint.setColor(stroke.color);
    strokePaint.setStrokeWidth(stroke.width);
    strokePaint.setStyle(CanvasKit.PaintStyle.Stroke);
    strokePaint.setAntiAlias(true);

    drawCommands.push({
      type: "path",
      path: rectPath,
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
      path: rectPath,
      paint: fillPaint,
    });
  }

  if (onAfterDraw) {
    if (!id) {
      id = nanoid();
    }
    renderingTree.push(
      AfterDraw((param) => {
        onAfterDraw(id!);
      }),
    );
  }

  renderingTree.push({
    drawCalls: [
      {
        commands: drawCommands,
      },
    ],
    id,
    onClick,
    onClickOut,
    onMouseIn,
    onMouseMoveIn,
    onMouseMoveOut,
    onMouseDown,
    onMouseUp,
  });

  return renderingTree;
}
