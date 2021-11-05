import { XywhRect } from "namui";

export type BaseClip = {
  id: string;
  startMs: number;
  endMs: number;
};

export type Clip = BaseClip | CameraClip;

export type CameraClip = BaseClip & {
  type: "camera";
  cameraAngle: CameraAngle;
};

export type CameraAngle = {
  imageSourceUrl: string;
  source01Rect: XywhRect;
  dest01Rect: XywhRect;
};
