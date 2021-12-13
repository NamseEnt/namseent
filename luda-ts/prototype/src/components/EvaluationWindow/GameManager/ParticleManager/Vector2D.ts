type Vector2DProps = {
  x?: number;
  y?: number;
  rotation?: number;
};

export default class Vector2D {
  public x: number = 0;
  public y: number = 0;
  public rotation: number = 0;

  constructor(props: Vector2DProps = {}) {
    this.x = props.x || 0;
    this.y = props.y || 0;
    this.rotation = props.rotation || 0;
  }

  public increase(vector2D: Vector2D, dt: number) {
    this.x += vector2D.x * dt;
    this.y += vector2D.y * dt;
    this.rotation += vector2D.rotation * dt;
  }
}
