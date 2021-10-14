import Vector2D from "../Vector2D";
import TextParticle, { TextParticleProps } from "./TextParticle";

export default class FadingTextParticle extends TextParticle {
  protected accelerationWithoutDrag: Vector2D;

  constructor(props: TextParticleProps) {
    super(props);
    this.accelerationWithoutDrag = this.acceleration;
    this.acceleration = new Vector2D({});
  }

  protected getFade() {
    const progress = this.age / this.lifeTime;
    return Math.max(0, Math.min(1, 1 - Math.pow(progress - 0.5, 2) * 2));
  }

  public tick(dt: number) {
    const constant = -0.00004;
    this.acceleration.x =
      this.accelerationWithoutDrag.x +
      constant * this.velocity.x * Math.abs(this.velocity.x);
    this.acceleration.y =
      this.accelerationWithoutDrag.y +
      constant * this.velocity.y * Math.abs(this.velocity.y);
    this.acceleration.rotation =
      this.accelerationWithoutDrag.rotation +
      constant * this.velocity.rotation * Math.abs(this.velocity.rotation);
    return super.tick(dt);
  }

  public render(context: CanvasRenderingContext2D) {
    const fade = this.getFade();
    context.save();
    context.globalAlpha = fade;
    context.translate(this.position.x, this.position.y);
    context.scale(fade, fade);
    context.translate(-this.position.x, -this.position.y);
    super.render(context);
    context.restore();
  }
}
