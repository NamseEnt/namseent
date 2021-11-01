import {
  ColorUtil,
  RenderingTree,
  Translate,
  Vector,
  PathDrawCommand,
  RenderingData,
  Cursor,
  engine,
  Rect,
  AfterDraw,
  Convert,
  Render,
} from "namui";
import { LtrbRect } from "namui/lib/type";
import { CameraAngleEditorState } from "../type";

export const Resizer: Render<
  CameraAngleEditorState,
  {
    widthHeightRatio: number;
  }
> = (state, props) => {
  return [
    Rect({
      ...state.cameraAngle.sourceRect,
      style: {
        stroke: {
          color: ColorUtil.Grayscale01(0.2),
          width: 1,
        },
      },
    }),
    Translate(
      {
        x: state.cameraAngle.sourceRect.x,
        y: state.cameraAngle.sourceRect.y,
      },
      [
        renderImageSizeHandles(
          {
            widthHeightRatio: props.widthHeightRatio,
          },
          state,
        ),
      ],
    ),
  ];
};

function renderImageSizeHandles(
  props: {
    widthHeightRatio: number;
  },
  state: CameraAngleEditorState,
): RenderingTree {
  const { sourceRect } = state.cameraAngle;
  const handleRadius = 5;
  const handles: {
    id:
      | "resize-top-left"
      | "resize-top"
      | "resize-top-right"
      | "resize-left"
      | "resize-right"
      | "resize-bottom-left"
      | "resize-bottom"
      | "resize-bottom-right";
    position: Vector;
    cursor: Cursor;
  }[] = [
    {
      id: "resize-top-left",
      position: new Vector(0, 0),
      cursor: Cursor.leftTopRightBottomResize,
    },
    {
      id: "resize-top",
      position: new Vector(sourceRect.width / 2, 0),
      cursor: Cursor.topBottomResize,
    },
    {
      id: "resize-top-right",
      position: new Vector(sourceRect.width, 0),
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "resize-left",
      position: new Vector(0, sourceRect.height / 2),
      cursor: Cursor.leftRightResize,
    },
    {
      id: "resize-right",
      position: new Vector(sourceRect.width, sourceRect.height / 2),
      cursor: Cursor.leftRightResize,
    },
    {
      id: "resize-bottom-left",
      position: new Vector(0, sourceRect.height),
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "resize-bottom",
      position: new Vector(sourceRect.width / 2, sourceRect.height),
      cursor: Cursor.topBottomResize,
    },
    {
      id: "resize-bottom-right",
      position: new Vector(sourceRect.width, sourceRect.height),
      cursor: Cursor.leftTopRightBottomResize,
    },
  ];

  return [
    ...handles.map((handle) => {
      const left = handle.position.x - handleRadius;
      const top = handle.position.y - handleRadius;
      const right = handle.position.x + handleRadius;
      const bottom = handle.position.y + handleRadius;

      const path = new CanvasKit.Path();
      path.addOval([left, top, right, bottom]);

      const fillPaint = new CanvasKit.Paint();
      fillPaint.setStyle(CanvasKit.PaintStyle.Fill);
      fillPaint.setColor(ColorUtil.White);

      const strokePaint = new CanvasKit.Paint();
      strokePaint.setStyle(CanvasKit.PaintStyle.Stroke);
      strokePaint.setColor(ColorUtil.Grayscale01(0.5));
      strokePaint.setStrokeWidth(2);
      strokePaint.setAntiAlias(true);

      return RenderingData({
        drawCalls: [
          {
            commands: [
              PathDrawCommand({
                path,
                paint: fillPaint,
              }),
              PathDrawCommand({
                path,
                paint: strokePaint,
              }),
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
      });
    }),
    AfterDraw(({ translated }) => {
      const container = new Vector(
        translated.x - sourceRect.x,
        translated.y - sourceRect.y,
      );
      engine.mouseEvent.onMouseMove((event) => {
        const { dragging } = state.wysiwygEditor;
        if (!dragging || !dragging.targetId.startsWith("resize-")) {
          return;
        }
        const { targetId } = dragging;
        const mouseVector = Vector.from(event);
        const diff = mouseVector.sub(dragging.lastMousePosition);

        const mouseLocalVector = mouseVector.sub(container);
        const handleId = targetId.substring("resize-".length);
        const isDiagonal = targetId.includes("-");

        if (isDiagonal) {
          const centerPoint = new Vector(
            sourceRect.x + sourceRect.width / 2,
            sourceRect.y + sourceRect.height / 2,
          );
          const endPoint = new Vector(
            handleId.includes("left")
              ? sourceRect.x
              : sourceRect.x + sourceRect.width,
            handleId.includes("top")
              ? sourceRect.y
              : sourceRect.y + sourceRect.height,
          );

          const diagonalVector = endPoint.sub(centerPoint);
          const centerToMouseLocalVector = mouseLocalVector.sub(centerPoint);

          const isCcw = diagonalVector.cross(centerToMouseLocalVector) > 0;
          function getMain(): keyof LtrbRect {
            switch (handleId) {
              case "top-left":
                return isCcw ? "top" : "left";
              case "top-right":
                return isCcw ? "right" : "top";
              case "bottom-right":
                return isCcw ? "bottom" : "right";
              case "bottom-left":
                return isCcw ? "left" : "bottom";
              default:
                throw new Error("unreachable");
            }
          }
          const main = getMain();
          const sub = handleId
            .replace(main, "")
            .replace("-", "") as keyof LtrbRect;

          const nextLtrbRect = resizeRect({
            rect: Convert.xywhToLtrb(sourceRect),
            main,
            sub,
            mouseLocalVector,
            widthHeightRatio: props.widthHeightRatio,
          });
          const nextRect = Convert.ltrbToXywh(nextLtrbRect);
          sourceRect.x = nextRect.x;
          sourceRect.y = nextRect.y;
          sourceRect.width = nextRect.width;
          sourceRect.height = nextRect.height;
        } else {
          if (handleId === "top" || handleId === "bottom") {
            let deltaWidth: number;

            if (handleId === "top") {
              sourceRect.y += diff.y;
              sourceRect.height -= diff.y;
              deltaWidth = -diff.y * props.widthHeightRatio;
            } else {
              sourceRect.height += diff.y;
              deltaWidth = diff.y * props.widthHeightRatio;
            }

            sourceRect.x -= deltaWidth / 2;
            sourceRect.width += deltaWidth;
          } else {
            let deltaHeight: number;

            if (handleId === "left") {
              sourceRect.x += diff.x;
              sourceRect.width -= diff.x;
              deltaHeight = -diff.x / props.widthHeightRatio;
            } else {
              sourceRect.width += diff.x;
              deltaHeight = diff.x / props.widthHeightRatio;
            }

            sourceRect.y -= deltaHeight / 2;
            sourceRect.height += deltaHeight;
          }
        }
        dragging.lastMousePosition = mouseVector;
      });
    }),
  ];
}

function resizeRect({
  rect,
  main,
  sub,
  mouseLocalVector,
  widthHeightRatio,
}: {
  rect: LtrbRect;
  main: keyof LtrbRect;
  sub: keyof LtrbRect;
  mouseLocalVector: Vector;
  widthHeightRatio: number;
}): LtrbRect {
  const nextRect: LtrbRect = {
    ...rect,
  };

  const isMainHorizontal = main === "left" || main === "right";

  nextRect[main] = isMainHorizontal ? mouseLocalVector.x : mouseLocalVector.y;
  const deltaMain = nextRect[main] - rect[main];

  const shouldDeltaReverse =
    (main === "right" && sub === "top") ||
    (main === "top" && sub === "right") ||
    (main === "left" && sub === "bottom") ||
    (main === "bottom" && sub === "left");

  nextRect[sub] +=
    deltaMain *
    (isMainHorizontal ? 1 / widthHeightRatio : widthHeightRatio) *
    (shouldDeltaReverse ? -1 : 1);

  return nextRect;
}
