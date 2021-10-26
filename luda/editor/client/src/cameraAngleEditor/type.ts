import { Selection, XywhRect } from "namui";

export type CameraAngleEditorState = {
  layout: {
    rect: XywhRect;
  };
  cameraAngle: CameraAngle;
  propertyTextEditor: {
    textInput: TextInputState;
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
