import {
  engine,
  Image,
  ImageFit,
  Paint,
  Render,
  Clip,
  ColorUtil,
  RenderExact,
  Vector,
  Cursor,
  Translate,
  Rect,
  WhSize,
  Convert,
  BorderPosition,
} from "namui";
import { CameraAngleEditorState, ImageSource } from "../type";
import { Croper } from "./Croper";
import { getDestRect, getSourceRect } from "./getRect";
import { Resizer } from "./Resizer";

export const WysiwygEditor: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  const containerSize: WhSize = state.layout.sub.wysiwygEditor;
  const imageSource = getImageSource(state);
  if (!imageSource) {
    return;
  }
  keepWidthHeightRatio(state, imageSource);

  engine.mouseEvent.onMouseUp(() => {
    state.wysiwygEditor.dragging = undefined;
  });

  return [
    Translate(
      {
        x: state.layout.sub.wysiwygEditor.x,
        y: state.layout.sub.wysiwygEditor.y,
      },
      [
        Rect({
          x: 0,
          y: 0,
          width: state.layout.sub.wysiwygEditor.width,
          height: state.layout.sub.wysiwygEditor.height,
          style: {
            stroke: {
              color: ColorUtil.Black,
              width: 2,
              borderPosition: BorderPosition.inside,
            },
          },
        }),
        OuterImage(state, { containerSize }),
        InnerImage(state, { containerSize }),
        Resizer(state, { containerSize, imageSource }),
        Croper(state),
      ],
    ),
  ];
};

function getImageSource(
  state: CameraAngleEditorState,
): ImageSource | undefined {
  if (state.wysiwygEditor.resizer.source) {
    return state.wysiwygEditor.resizer.source;
  }

  const image = engine.imageLoad.tryLoad(state.cameraAngle.imageSourceUrl);
  if (image) {
    const widthHeightRatio = image.width() / image.height();
    state.wysiwygEditor.resizer.source = {
      widthHeightRatio,
    };

    return state.wysiwygEditor.resizer.source;
  }

  return;
}

function keepWidthHeightRatio(
  state: CameraAngleEditorState,
  imageSource: ImageSource,
) {
  const { widthHeightRatio } = imageSource;
  const screenWhRatio = 16 / 9;

  if (widthHeightRatio > 1) {
    state.cameraAngle.source01Rect.height =
      (state.cameraAngle.source01Rect.width * screenWhRatio) / widthHeightRatio;
  } else {
    state.cameraAngle.source01Rect.width =
      (state.cameraAngle.source01Rect.height / screenWhRatio) *
      widthHeightRatio;
  }
}

const SourceImage: RenderExact<
  CameraAngleEditorState,
  {
    paint?: Paint;
  }
> = (state, props) => {
  const sourceRect = getSourceRect(state);
  return Image({
    position: {
      ...sourceRect,
    },
    size: {
      ...sourceRect,
    },
    style: {
      fit: ImageFit.fill,
      paint: props.paint,
    },
    url: state.cameraAngle.imageSourceUrl,
  });
};

const OuterImage: Render<CameraAngleEditorState, {}> = (state, props) => {
  const outsideImagePaint = new CanvasKit.Paint();
  outsideImagePaint.setStyle(CanvasKit.PaintStyle.Fill);
  outsideImagePaint.setColorFilter(
    CanvasKit.ColorFilter.MakeBlend(
      ColorUtil.Grayscale01(0.5),
      CanvasKit.BlendMode.Multiply,
    ),
  );
  const destRect = getDestRect(state);
  return Clip(
    {
      path: new CanvasKit.Path().addRect(Convert.xywhToCanvasKit(destRect)),
      clipOp: CanvasKit.ClipOp.Difference,
    },
    [
      SourceImage(state, {
        paint: outsideImagePaint,
      }),
    ],
  );
};

const InnerImage: Render<
  CameraAngleEditorState,
  {
    containerSize: WhSize;
  }
> = (state, props) => {
  engine.mouseEvent.onMouseMove((event) => {
    const { dragging } = state.wysiwygEditor;
    if (!dragging || dragging.targetId !== "move") {
      return;
    }
    const mouseVector = Vector.from(event);
    const diff = mouseVector.sub(dragging.lastMousePosition);
    state.cameraAngle.source01Rect.x += diff.x / props.containerSize.width;
    state.cameraAngle.source01Rect.y += diff.y / props.containerSize.height;
    dragging.lastMousePosition = mouseVector;
  });

  const destRect = getDestRect(state);
  return Clip(
    {
      path: new CanvasKit.Path().addRect(Convert.xywhToCanvasKit(destRect)),
      clipOp: CanvasKit.ClipOp.Intersect,
    },
    [
      {
        ...SourceImage(state, {}),
        onMouseDown(event) {
          state.wysiwygEditor.dragging = {
            targetId: "move",
            lastMousePosition: Vector.from(event),
          };
        },
        onMouseIn() {
          engine.mousePointer.setCursor(Cursor.move);
        },
      },
    ],
  );
};
