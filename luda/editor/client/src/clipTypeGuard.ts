import { CameraClip, Clip } from "./type";

export function isCameraClip(clip: Clip): clip is CameraClip {
  return (clip as any).type === "camera";
}
