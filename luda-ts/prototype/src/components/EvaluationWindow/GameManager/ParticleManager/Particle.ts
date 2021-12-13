export default abstract class Particle {
  public abstract tick(dt: number): boolean;
  public abstract render(context: CanvasRenderingContext2D): void;
}
