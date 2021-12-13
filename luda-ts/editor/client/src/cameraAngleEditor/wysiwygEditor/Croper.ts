import {
  ColorUtil,
  Cursor,
  engine,
  Mathu,
  Rect,
  Render,
  RenderingTree,
  Translate,
  Vector,
  XywhRect,
  BorderPosition,
} from "namui";
import { CameraAngleEditorState } from "../type";
import { getDestRect } from "./getRect";
import { update01Rect } from "./update01Rect";

export const Croper: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  const destRect = getDestRect(state);
  return [
    Translate(destRect, [
      Rect({
        ...destRect,
        x: 0,
        y: 0,
        style: {
          stroke: {
            color: ColorUtil.Grayscale01(0.5),
            width: 1,
            borderPosition: BorderPosition.inside,
          },
        },
      }),
      Handles(state),
    ]),
  ];
};

const Handles: Render<CameraAngleEditorState> = (state) => {
  const handles = getHandles(state);
  engine.mouseEvent.onMouseMove((event) => {
    const { dragging } = state.wysiwygEditor;
    if (!dragging || !dragging.targetId.startsWith("crop-")) {
      return;
    }
    const mouseVector = Vector.from(event);
    const diff = mouseVector.sub(dragging.lastMousePosition);
    const nextRect = getDestRect(state);
    if (
      dragging.targetId.includes("top") &&
      Mathu.in(nextRect.y + diff.y, 0, state.layout.sub.wysiwygEditor.height)
    ) {
      nextRect.y += diff.y;
      nextRect.height -= diff.y;
    }
    if (dragging.targetId.includes("bottom")) {
      nextRect.height += diff.y;
    }
    if (
      dragging.targetId.includes("left") &&
      Mathu.in(nextRect.x + diff.x, 0, state.layout.sub.wysiwygEditor.width)
    ) {
      nextRect.x += diff.x;
      nextRect.width -= diff.x;
    }
    if (dragging.targetId.includes("right")) {
      nextRect.width += diff.x;
    }
    update01Rect(state, state.cameraAngle.dest01Rect, nextRect);

    dragging.lastMousePosition = mouseVector;
  });

  return [
    handles.map((handle) => {
      const flattenedPointArray = handle.polyPoints.reduce((acc, point) => {
        acc.push(point.x, point.y);
        return acc;
      }, [] as number[]);

      const path = new CanvasKit.Path().addPoly(flattenedPointArray, true);

      const strokePaint = new CanvasKit.Paint();
      strokePaint.setStyle(CanvasKit.PaintStyle.Stroke);
      strokePaint.setStrokeWidth(1);
      strokePaint.setColor(ColorUtil.White);

      const fillPaint = new CanvasKit.Paint();
      fillPaint.setStyle(CanvasKit.PaintStyle.Fill);
      fillPaint.setColor(ColorUtil.Black);

      return {
        drawCalls: [
          {
            commands: [
              {
                type: "path",
                path,
                paint: fillPaint,
              },
              {
                type: "path",
                path,
                paint: strokePaint,
              },
            ],
          },
        ],
        onMouseIn() {
          engine.mousePointer.setCursor(handle.cursor);
        },
        onMouseDown(event) {
          state.wysiwygEditor.dragging = {
            targetId: handle.id,
            lastMousePosition: Vector.from(event),
          };
        },
      };
    }),
  ];
};

function getHandles(state: CameraAngleEditorState): {
  id:
    | "crop-top-left"
    | "crop-top"
    | "crop-top-right"
    | "crop-left"
    | "crop-right"
    | "crop-bottom-left"
    | "crop-bottom"
    | "crop-bottom-right";
  polyPoints: Vector[];
  cursor: Cursor;
}[] {
  const destRect = getDestRect(state);
  const center = {
    x: destRect.width / 2,
    y: destRect.height / 2,
  };

  const handleSize = 24;
  const handleThickness = 6;
  return [
    {
      id: "crop-top",
      polyPoints: [
        new Vector(center.x - handleSize / 2, 0),
        new Vector(center.x + handleSize / 2, 0),
        new Vector(center.x + handleSize / 2, handleThickness),
        new Vector(center.x - handleSize / 2, handleThickness),
      ],
      cursor: Cursor.topBottomResize,
    },
    {
      id: "crop-bottom",
      polyPoints: [
        new Vector(
          center.x - handleSize / 2,
          destRect.height - handleThickness,
        ),
        new Vector(
          center.x + handleSize / 2,
          destRect.height - handleThickness,
        ),
        new Vector(center.x + handleSize / 2, destRect.height),
        new Vector(center.x - handleSize / 2, destRect.height),
      ],
      cursor: Cursor.topBottomResize,
    },
    {
      id: "crop-left",
      polyPoints: [
        new Vector(0, center.y - handleSize / 2),
        new Vector(handleThickness, center.y - handleSize / 2),
        new Vector(handleThickness, center.y + handleSize / 2),
        new Vector(0, center.y + handleSize / 2),
      ],
      cursor: Cursor.leftRightResize,
    },
    {
      id: "crop-right",
      polyPoints: [
        new Vector(destRect.width - handleThickness, center.y - handleSize / 2),
        new Vector(destRect.width, center.y - handleSize / 2),
        new Vector(destRect.width, center.y + handleSize / 2),
        new Vector(destRect.width - handleThickness, center.y + handleSize / 2),
      ],
      cursor: Cursor.leftRightResize,
    },
    {
      id: "crop-top-left",
      polyPoints: [
        new Vector(0, 0),
        new Vector(handleSize, 0),
        new Vector(handleSize, handleThickness),
        new Vector(handleThickness, handleThickness),
        new Vector(handleThickness, handleSize),
        new Vector(0, handleSize),
      ],
      cursor: Cursor.leftTopRightBottomResize,
    },
    {
      id: "crop-top-right",
      polyPoints: [
        new Vector(destRect.width, 0),
        new Vector(destRect.width, handleSize),
        new Vector(destRect.width - handleThickness, handleSize),
        new Vector(destRect.width - handleThickness, handleThickness),
        new Vector(destRect.width - handleSize, handleThickness),
        new Vector(destRect.width - handleSize, 0),
      ],
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "crop-bottom-left",
      polyPoints: [
        new Vector(0, destRect.height),
        new Vector(handleSize, destRect.height),
        new Vector(handleSize, destRect.height - handleThickness),
        new Vector(handleThickness, destRect.height - handleThickness),
        new Vector(handleThickness, destRect.height - handleSize),
        new Vector(0, destRect.height - handleSize),
      ],
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "crop-bottom-right",
      polyPoints: [
        new Vector(destRect.width, destRect.height),
        new Vector(destRect.width, destRect.height - handleSize),
        new Vector(
          destRect.width - handleThickness,
          destRect.height - handleSize,
        ),
        new Vector(
          destRect.width - handleThickness,
          destRect.height - handleThickness,
        ),
        new Vector(
          destRect.width - handleSize,
          destRect.height - handleThickness,
        ),
        new Vector(destRect.width - handleSize, destRect.height),
      ],
      cursor: Cursor.leftTopRightBottomResize,
    },
  ];
}
