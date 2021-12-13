import Vector2D from "../Vector2D";
import Particle from "../Particle";
import Color from "../../ColorManager/Color";

export type TextParticleProps = {
  text: string;
  position?: Vector2D;
  velocity?: Vector2D;
  acceleration?: Vector2D;
  lifeTime?: number;
  baseLine?: CanvasTextBaseline;
  align?: CanvasTextAlign;
  font?: string;
  fontSize?: number;
  fillColor?: Color;
  strokeColor?: Color;
  lineWidth?: number;
};

export default class TextParticle extends Particle {
  protected text: string;
  protected position: Vector2D;
  protected velocity: Vector2D;
  protected acceleration: Vector2D;
  protected lifeTime: number;
  protected baseLine: CanvasTextBaseline;
  protected align: CanvasTextAlign;
  protected font: string;
  protected fontSize: number;
  protected fillColor: Color;
  protected strokeColor: Color;
  protected lineWidth: number;
  protected age: number = 0;

  constructor(props: TextParticleProps) {
    super();
    this.text = props.text;
    this.position = props.position || new Vector2D({});
    this.velocity = props.velocity || new Vector2D({});
    this.acceleration = props.acceleration || new Vector2D({});
    this.lifeTime = props.lifeTime || 1;
    this.baseLine = props.baseLine || "middle";
    this.align = props.align || "center";
    this.font = props.font || "Malgun Gothic,serif";
    this.fontSize = props.fontSize || 14;
    this.fillColor = props.fillColor || new Color({});
    this.strokeColor = props.strokeColor || new Color({});
    this.lineWidth = props.lineWidth || 1;
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
    context.save();
    context.textAlign = this.align;
    context.textBaseline = this.baseLine;
    context.font = `${this.fontSize}px ${this.font}`;
    context.strokeStyle = this.strokeColor.toRgbaString();
    context.fillStyle = this.fillColor.toRgbaString();
    context.lineWidth = this.lineWidth;
    context.translate(this.position.x, this.position.y);
    context.rotate(this.position.rotation);
    context.strokeText(this.text, 0, 0);
    context.fillText(this.text, 0, 0);
    context.restore();
  }
}
