import {
  ColorUtil,
  Image,
  ImageFit,
  RenderingTree,
  Translate,
  Vector,
  PathDrawCommand,
  RenderingData,
  Cursor,
  engine,
  Clip,
  Paint,
  Rect,
  AfterDraw,
  Convert,
  XywhRect,
} from "namui";
import { LtrbRect } from "namui/lib/type";
import { CameraAngleEditorState } from "../type";

export function renderImageRect(state: CameraAngleEditorState): RenderingTree {
  const image = engine.imageLoader.tryLoad(state.cameraAngle.imageSourceUrl);
  const { source } = state.wysiwygEditor.image;

  if (!source) {
    if (image) {
      const widthHeightRatio = image.width() / image.height();
      state.wysiwygEditor.image.source = {
        widthHeightRatio,
      };
      state.cameraAngle.sourceRect.width =
        state.cameraAngle.sourceRect.height * widthHeightRatio;
    }
    return;
  }

  const outsideImagePaint = new CanvasKit.Paint();
  outsideImagePaint.setStyle(CanvasKit.PaintStyle.Fill);
  outsideImagePaint.setColorFilter(
    CanvasKit.ColorFilter.MakeBlend(
      ColorUtil.Grayscale01(0.5),
      CanvasKit.BlendMode.Multiply,
    ),
  );
  const imageRendering = (paint?: Paint) =>
    Image({
      position: {
        x: state.cameraAngle.sourceRect.x,
        y: state.cameraAngle.sourceRect.y,
      },
      size: {
        width: state.cameraAngle.sourceRect.width,
        height: state.cameraAngle.sourceRect.height,
      },
      style: {
        fit: ImageFit.fill,
        paint,
      },
      url: state.cameraAngle.imageSourceUrl,
    });

  return [
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(
            state.cameraAngle.destRect.x,
            state.cameraAngle.destRect.y,
            state.cameraAngle.destRect.width,
            state.cameraAngle.destRect.height,
          ),
        ),
        clipOp: CanvasKit.ClipOp.Difference,
      },
      [imageRendering(outsideImagePaint)],
    ),
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(
            state.cameraAngle.destRect.x,
            state.cameraAngle.destRect.y,
            state.cameraAngle.destRect.width,
            state.cameraAngle.destRect.height,
          ),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      [
        {
          ...imageRendering(),
          onMouseDown(event) {
            state.wysiwygEditor.image.dragging = {
              handleId: "center",
              lastMousePosition: Vector.from(event),
            };
          },
          onMouseIn() {
            engine.mousePointer.setCursor(Cursor.move);
          },
        },
      ],
    ),
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
            widthHeightRatio: source.widthHeightRatio,
          },
          state,
        ),
      ],
    ),
  ];
}

function renderImageSizeHandles(
  props: {
    widthHeightRatio: number;
  },
  state: CameraAngleEditorState,
): RenderingTree {
  const { sourceRect } = state.cameraAngle;
  const handleRadius = 5;
  const handles: {
    id: string;
    position: Vector;
    cursor: Cursor;
  }[] = [
    {
      id: "top-left",
      position: new Vector(0, 0),
      cursor: Cursor.leftTopRightBottomResize,
    },
    {
      id: "top",
      position: new Vector(sourceRect.width / 2, 0),
      cursor: Cursor.topBottomResize,
    },
    {
      id: "top-right",
      position: new Vector(sourceRect.width, 0),
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "left",
      position: new Vector(0, sourceRect.height / 2),
      cursor: Cursor.leftRightResize,
    },
    {
      id: "right",
      position: new Vector(sourceRect.width, sourceRect.height / 2),
      cursor: Cursor.leftRightResize,
    },
    {
      id: "bottom-left",
      position: new Vector(0, sourceRect.height),
      cursor: Cursor.rightTopLeftBottomResize,
    },
    {
      id: "bottom",
      position: new Vector(sourceRect.width / 2, sourceRect.height),
      cursor: Cursor.topBottomResize,
    },
    {
      id: "bottom-right",
      position: new Vector(sourceRect.width, sourceRect.height),
      cursor: Cursor.leftTopRightBottomResize,
    },
  ];

  engine.mouseEvent.onMouseUp(() => {
    state.wysiwygEditor.image.dragging = undefined;
  });

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
          state.wysiwygEditor.image.dragging = {
            handleId: handle.id,
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
        const { dragging } = state.wysiwygEditor.image;
        if (!dragging) {
          return;
        }
        const { handleId } = dragging;
        const mouseVector = Vector.from(event);
        const diff = mouseVector.sub(dragging.lastMousePosition);

        if (handleId === "center") {
          sourceRect.x += diff.x;
          sourceRect.y += diff.y;
        } else {
          const mouseLocalVector = mouseVector.sub(container);
          const isDiagonal = handleId.includes("-");

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
              rect: Convert.XywhToLtrb(sourceRect),
              main,
              sub,
              mouseLocalVector,
              widthHeightRatio: props.widthHeightRatio,
            });
            const nextRect = Convert.LtrbToXywh(nextLtrbRect);
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
