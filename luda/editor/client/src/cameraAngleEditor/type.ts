import { Selection, Vector, XywhRect } from "namui";

export type CameraAngleEditorState = {
  layout: {
    rect: XywhRect;
  };
  cameraAngle: CameraAngle;
  propertyTextEditor: {
    textInput: TextInputState;
  };
  wysiwygEditor: {
    crop: {
      dragging?: {
        handleId: string;
        lastMousePosition: Vector;
      };
    };
  };
};
export type CameraAngle = {
  imageSourceUrl: string;
  sourceRect: XywhRect;
  destRect: XywhRect;
};
export type TextInputState = {
  targetId?: string;
  selection?: Selection;
};
