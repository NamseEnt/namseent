import Vector2D from "../Vector2D";
import Particle from "../Particle";
import Color from "../../ColorManager/Color";

export type GlowImageParticleProps = {
  width: number;
  imageSource: HTMLImageElement;
  color?: Color;
  position?: Vector2D;
  velocity?: Vector2D;
  acceleration?: Vector2D;
  lifeTime?: number;
};

export default class GlowImageParticle extends Particle {
  protected position: Vector2D;
  protected velocity: Vector2D;
  protected acceleration: Vector2D;
  protected lifeTime: number;
  protected age: number = 0;
  protected imageSource: HTMLImageElement;
  protected color: Color;
  protected width: number;

  constructor(props: GlowImageParticleProps) {
    super();
    const {
      width,
      imageSource,
      position,
      velocity,
      acceleration,
      lifeTime,
      color,
    } = props;
    this.width = width;
    this.position = position || new Vector2D();
    this.velocity = velocity || new Vector2D();
    this.acceleration = acceleration || new Vector2D();
    this.lifeTime = lifeTime || 1;
    this.imageSource = imageSource;
    this.color = color || new Color();
  }

  tick(dt: number) {
    this.age += dt;
    if (this.age > this.lifeTime) {
      return false;
    }
    this.velocity.increase(this.acceleration, dt);
    this.position.increase(this.velocity, dt);
    return true;
  }

  render(context: CanvasRenderingContext2D) {
    const width = this.width;
    const height = this.imageSource.width
      ? (this.imageSource.height / this.imageSource.width) * width
      : 0;
    const { h: hue } = this.color.hsl;

    context.globalCompositeOperation = "lighter";
    context.translate(this.position.x, this.position.y);
    context.rotate(this.position.rotation);
    context.shadowBlur = width / 24;
    context.shadowColor = "white";
    context.filter = `invert() sepia() saturate(10000%) hue-rotate(${
      hue - 30
    }deg)`;
    context.drawImage(this.imageSource, -width / 2, -height / 2, width, height);
    context.restore();
  }
}
