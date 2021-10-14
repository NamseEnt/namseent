import hayeonPoseW from "../../../../../../public/image/evaluation-simon/hayeon-pose-w.png";
import hayeonPoseA from "../../../../../../public/image/evaluation-simon/hayeon-pose-a.png";
import hayeonPoseS from "../../../../../../public/image/evaluation-simon/hayeon-pose-s.png";
import hayeonPoseD from "../../../../../../public/image/evaluation-simon/hayeon-pose-d.png";
import { SimonKey } from "../../type";
import Vector2D from "../Vector2D";
import ParticleManager from "..";
import Color from "../../ColorManager/Color";
import SimonPoseParticle from "../particles/SimonPoseParticle";
import { getImageElement } from "../../../../../util/getImageElement";

type EmitSimonPoseProps = {
  particleManager: ParticleManager;
  position: Vector2D;
  color: Color;
  lifeTime?: number;
  width: number;
  key: SimonKey;
};

const hayeonPoseImageSourceMap: Record<SimonKey, HTMLImageElement> = {
  w: getImageElement(hayeonPoseW),
  a: getImageElement(hayeonPoseA),
  s: getImageElement(hayeonPoseS),
  d: getImageElement(hayeonPoseD),
};

const poseDirectionMap: Record<SimonKey, { x: number; y: number }> = {
  w: { x: 0, y: -1 },
  a: { x: -1, y: 0 },
  s: { x: 0, y: 1 },
  d: { x: 1, y: 0 },
};

export default function emitSimonPose(props: EmitSimonPoseProps) {
  const { particleManager, position, color, lifeTime, width, key } = props;
  const direction = poseDirectionMap[key];

  particleManager.addParticle(
    new SimonPoseParticle({
      width,
      imageSource: hayeonPoseImageSourceMap[key],
      color,
      position,
      velocity: new Vector2D({
        x: direction.x * width * 5,
        y: direction.y * width * 5,
      }),
      airResistance: -0.008,
      lifeTime,
    }),
  );
}
