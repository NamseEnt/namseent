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
} from "namui";
import { CameraAngleEditorState, ImageSource } from "../type";
import { Croper } from "./Croper";
import { Resizer } from "./Resizer";

export const WysiwygEditor: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  const containerSize: WhSize = state.layout.sub.wysiwygEditor;
  const imageSource = getImageSource(state, containerSize);
  if (!imageSource) {
    return;
  }

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
  containerSize: WhSize,
): ImageSource | undefined {
  if (state.wysiwygEditor.resizer.source) {
    return state.wysiwygEditor.resizer.source;
  }

  const image = engine.imageLoader.tryLoad(state.cameraAngle.imageSourceUrl);
  if (image) {
    const widthHeightRatio = image.width() / image.height();
    state.wysiwygEditor.resizer.source = {
      widthHeightRatio,
    };

    const containerWhRatio = containerSize.width / containerSize.height;
    if (widthHeightRatio > 1) {
      state.cameraAngle.source01Rect.width = 1;
      state.cameraAngle.source01Rect.height =
        containerWhRatio / widthHeightRatio;
    } else {
      state.cameraAngle.source01Rect.height = 1;
      state.cameraAngle.source01Rect.width =
        (1 / containerWhRatio) * widthHeightRatio;
    }

    return state.wysiwygEditor.resizer.source;
  }

  return;
}

const SourceImage: RenderExact<
  CameraAngleEditorState,
  {
    paint?: Paint;
    containerSize: WhSize;
  }
> = (state, props) => {
  return Image({
    position: {
      x: props.containerSize.width * state.cameraAngle.source01Rect.x,
      y: props.containerSize.height * state.cameraAngle.source01Rect.y,
    },
    size: {
      width: props.containerSize.width * state.cameraAngle.source01Rect.width,
      height:
        props.containerSize.height * state.cameraAngle.source01Rect.height,
    },
    style: {
      fit: ImageFit.fill,
      paint: props.paint,
    },
    url: state.cameraAngle.imageSourceUrl,
  });
};

const OuterImage: Render<
  CameraAngleEditorState,
  {
    containerSize: WhSize;
  }
> = (state, props) => {
  const outsideImagePaint = new CanvasKit.Paint();
  outsideImagePaint.setStyle(CanvasKit.PaintStyle.Fill);
  outsideImagePaint.setColorFilter(
    CanvasKit.ColorFilter.MakeBlend(
      ColorUtil.Grayscale01(0.5),
      CanvasKit.BlendMode.Multiply,
    ),
  );
  return Clip(
    {
      path: new CanvasKit.Path().addRect(
        CanvasKit.XYWHRect(
          props.containerSize.width * state.cameraAngle.dest01Rect.x,
          props.containerSize.height * state.cameraAngle.dest01Rect.y,
          props.containerSize.width * state.cameraAngle.dest01Rect.width,
          props.containerSize.height * state.cameraAngle.dest01Rect.height,
        ),
      ),
      clipOp: CanvasKit.ClipOp.Difference,
    },
    [
      SourceImage(state, {
        paint: outsideImagePaint,
        containerSize: props.containerSize,
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

  return Clip(
    {
      path: new CanvasKit.Path().addRect(
        CanvasKit.XYWHRect(
          props.containerSize.width * state.cameraAngle.dest01Rect.x,
          props.containerSize.height * state.cameraAngle.dest01Rect.y,
          props.containerSize.width * state.cameraAngle.dest01Rect.width,
          props.containerSize.height * state.cameraAngle.dest01Rect.height,
        ),
      ),
      clipOp: CanvasKit.ClipOp.Intersect,
    },
    [
      {
        ...SourceImage(state, { containerSize: props.containerSize }),
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
