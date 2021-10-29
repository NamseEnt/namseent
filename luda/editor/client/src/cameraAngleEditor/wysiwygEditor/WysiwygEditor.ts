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
} from "namui";
import { CameraAngleEditorState } from "../type";
import { Croper } from "./Croper";
import { Resizer } from "./Resizer";

export const WysiwygEditor: Render<CameraAngleEditorState> = (
  state: CameraAngleEditorState,
) => {
  const source = getImageSource(state);
  if (!source) {
    return;
  }

  engine.mouseEvent.onMouseUp(() => {
    state.wysiwygEditor.dragging = undefined;
  });

  return [
    OuterImage(state),
    InnerImage(state),
    Resizer(state, source),
    Croper(state),
  ];
};

function getImageSource(
  state: CameraAngleEditorState,
): CameraAngleEditorState["wysiwygEditor"]["resizer"]["source"] {
  if (state.wysiwygEditor.resizer.source) {
    return state.wysiwygEditor.resizer.source;
  }

  const image = engine.imageLoader.tryLoad(state.cameraAngle.imageSourceUrl);
  if (image) {
    const widthHeightRatio = image.width() / image.height();
    state.wysiwygEditor.resizer.source = {
      widthHeightRatio,
    };
    state.cameraAngle.sourceRect.width =
      state.cameraAngle.sourceRect.height * widthHeightRatio;
    return state.wysiwygEditor.resizer.source;
  }

  return;
}

const SourceImage: RenderExact<
  CameraAngleEditorState,
  {
    paint?: Paint;
  }
> = (state, props) => {
  return Image({
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
      paint: props.paint,
    },
    url: state.cameraAngle.imageSourceUrl,
  });
};

const OuterImage: Render<CameraAngleEditorState> = (state) => {
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
          state.cameraAngle.destRect.x,
          state.cameraAngle.destRect.y,
          state.cameraAngle.destRect.width,
          state.cameraAngle.destRect.height,
        ),
      ),
      clipOp: CanvasKit.ClipOp.Difference,
    },
    [SourceImage(state, { paint: outsideImagePaint })],
  );
};

const InnerImage: Render<CameraAngleEditorState> = (state) => {
  engine.mouseEvent.onMouseMove((event) => {
    const { dragging } = state.wysiwygEditor;
    if (!dragging || dragging.targetId !== "move") {
      return;
    }
    const mouseVector = Vector.from(event);
    const diff = mouseVector.sub(dragging.lastMousePosition);
    state.cameraAngle.sourceRect.x += diff.x;
    state.cameraAngle.sourceRect.y += diff.y;
    dragging.lastMousePosition = mouseVector;
  });

  return Clip(
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
