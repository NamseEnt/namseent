import ParticleManager from "..";
import Color from "../../ColorManager/Color";
import FadingTextParticle from "../particles/FadingTextParticle";
import Vector2D from "../Vector2D";

type EmitTextPoppingEffectProps = {
  particleManager: ParticleManager;
  position: Vector2D;
  unitSize: number;
  power: number;
  text: string;
  fillColor: Color;
  strokeColor: Color;
  fontSize: number;
  lineWidth: number;
  lifeTime?: number;
};

export default function emitTextPoppingEffect(
  props: EmitTextPoppingEffectProps,
) {
  const {
    particleManager,
    position,
    unitSize,
    power,
    text,
    fillColor,
    strokeColor,
    fontSize,
    lineWidth,
    lifeTime,
  } = props;

  const theta = (Math.random() * Math.PI) / 2 + Math.PI / 4 + Math.PI;
  const radius = (Math.random() / 2 + 0.5) * power * unitSize;

  particleManager.addParticle(
    new FadingTextParticle({
      text,
      position,
      acceleration: new Vector2D({
        y: 100 * unitSize,
      }),
      fillColor,
      fontSize,
      lifeTime,
      lineWidth,
      strokeColor,
      velocity: new Vector2D({
        x: Math.cos(theta) * radius,
        y: Math.sin(theta) * radius,
        rotation: (Math.random() - 0.5) * Math.PI * 2 * (power / 20),
      }),
    }),
  );
}
