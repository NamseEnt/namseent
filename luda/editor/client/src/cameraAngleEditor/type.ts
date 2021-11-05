import { Selection, Vector, XywhRect } from "namui";
import { CameraAngle } from "../type";

export type CameraAngleEditorState = {
  layout: {
    rect: XywhRect;
    sub: {
      wysiwygEditor: XywhRect;
      preview: XywhRect;
    };
  };
  cameraAngle: CameraAngle;
  propertyTextEditor: {
    textInput: TextInputState;
  };
  wysiwygEditor: {
    dragging?: {
      targetId:
        | "move"
        | "crop-top-left"
        | "crop-top"
        | "crop-top-right"
        | "crop-left"
        | "crop-right"
        | "crop-bottom-left"
        | "crop-bottom"
        | "crop-bottom-right"
        | "resize-top-left"
        | "resize-top"
        | "resize-top-right"
        | "resize-left"
        | "resize-right"
        | "resize-bottom-left"
        | "resize-bottom"
        | "resize-bottom-right";
      lastMousePosition: Vector;
    };
    resizer: {
      source?: ImageSource;
    };
  };
};
export type TextInputState = {
  targetId?: string;
  selection?: Selection;
};
export type ImageSource = {
  widthHeightRatio: number;
};
