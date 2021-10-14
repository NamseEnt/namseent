import Vector2D from "../Vector2D";
import GlowImageParticle, { GlowImageParticleProps } from "./GlowImageParticle";

export type SimonPoseParticleProps = GlowImageParticleProps & {
  acceleration?: Vector2D;
  airResistance?: number;
};

export default class SimonPoseParticle extends GlowImageParticle {
  protected accelerationWithoutAirResistance: Vector2D;
  protected airResistance: number;

  constructor(props: SimonPoseParticleProps) {
    super({
      ...props,
      acceleration: new Vector2D(),
    });
    const { acceleration, airResistance } = props;

    this.accelerationWithoutAirResistance = acceleration || new Vector2D();
    this.airResistance = airResistance || 0;
  }

  private getAlpha() {
    const progress = this.age / this.lifeTime;
    return Math.max(0, Math.min(1, 1 - Math.pow(progress - 0.5, 2) * 2));
  }

  tick(dt: number) {
    this.age += dt;
    if (this.age > this.lifeTime) {
      return false;
    }
    this.acceleration.x =
      this.accelerationWithoutAirResistance.x +
      this.airResistance * this.velocity.x * Math.abs(this.velocity.x);
    this.acceleration.y =
      this.accelerationWithoutAirResistance.y +
      this.airResistance * this.velocity.y * Math.abs(this.velocity.y);
    this.acceleration.rotation =
      this.accelerationWithoutAirResistance.rotation +
      this.airResistance * this.velocity.rotation * this.velocity.rotation;
    super.tick(dt);
    return true;
  }

  render(context: CanvasRenderingContext2D) {
    const alpha = this.getAlpha();
    context.save();
    context.globalAlpha = alpha;
    super.render(context);
    context.restore();
  }
}
