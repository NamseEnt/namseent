import { CameraClip, Clip, SubtitleClip } from "./type";

export function isCameraClip(clip: Clip): clip is CameraClip {
  return (clip as any).type === "camera";
}

export function isSubtitleClip(clip: Clip): clip is SubtitleClip {
  return (clip as any).type === "subtitle";
}
