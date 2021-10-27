import {
  ColorUtil,
  Cursor,
  engine,
  Rect,
  RenderingTree,
  Translate,
  Vector,
} from "namui";
import { CameraAngleEditorState } from "../type";

export function renderCropRect(state: CameraAngleEditorState): RenderingTree {
  const { destRect } = state.cameraAngle;
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
          },
        },
      }),
      renderHandles(state),
    ]),
  ];
}

function renderHandles(state: CameraAngleEditorState): RenderingTree {
  const { destRect } = state.cameraAngle;
  const center = {
    x: destRect.width / 2,
    y: destRect.height / 2,
  };

  const handleSize = 24;
  const handleThickness = 6;

  const handles: {
    id: string;
    polyPoints: Vector[];
    cursor: Cursor;
  }[] = [
    {
      id: "top",
      polyPoints: [
        new Vector(center.x - handleSize / 2, 0),
        new Vector(center.x + handleSize / 2, 0),
        new Vector(center.x + handleSize / 2, handleThickness),
        new Vector(center.x - handleSize / 2, handleThickness),
      ],
      cursor: Cursor.topBottomResize,
    },
    {
      id: "bottom",
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
      id: "left",
      polyPoints: [
        new Vector(0, center.y - handleSize / 2),
        new Vector(handleThickness, center.y - handleSize / 2),
        new Vector(handleThickness, center.y + handleSize / 2),
        new Vector(0, center.y + handleSize / 2),
      ],
      cursor: Cursor.leftRightResize,
    },
    {
      id: "right",
      polyPoints: [
        new Vector(destRect.width - handleThickness, center.y - handleSize / 2),
        new Vector(destRect.width, center.y - handleSize / 2),
        new Vector(destRect.width, center.y + handleSize / 2),
        new Vector(destRect.width - handleThickness, center.y + handleSize / 2),
      ],
      cursor: Cursor.leftRightResize,
    },
    {
      id: "left top",
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
      id: "right top",
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
      id: "left bottom",
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
      id: "right bottom",
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

  engine.mouseEvent.onMouseUp(() => {
    state.wysiwygEditor.crop.dragging = undefined;
  });
  engine.mouseEvent.onMouseMove((event) => {
    const { dragging } = state.wysiwygEditor.crop;
    if (!dragging) {
      return;
    }
    const mouseVector = Vector.from(event);
    const diff = mouseVector.sub(dragging.lastMousePosition);

    console.log(diff.x, diff.y);

    if (dragging.handleId.includes("top")) {
      state.cameraAngle.destRect.y += diff.y;
      state.cameraAngle.destRect.height -= diff.y;
    }
    if (dragging.handleId.includes("bottom")) {
      state.cameraAngle.destRect.height += diff.y;
    }
    if (dragging.handleId.includes("left")) {
      state.cameraAngle.destRect.x += diff.x;
      state.cameraAngle.destRect.width -= diff.x;
    }
    if (dragging.handleId.includes("right")) {
      state.cameraAngle.destRect.width += diff.x;
    }

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
          state.wysiwygEditor.crop.dragging = {
            handleId: handle.id,
            lastMousePosition: Vector.from(event),
          };
        },
      };
    }),
  ];
}
