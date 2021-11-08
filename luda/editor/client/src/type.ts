import { XywhRect } from "namui";

export type BaseClip = {
  /**
   * Special Ids
   * - Start with 'fake'
   *   - it's not a real clip.
   */
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
