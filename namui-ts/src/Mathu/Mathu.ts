import { XywhRect, Vector } from "..";
import { IMathu } from "./IMathu";

export class Mathu implements IMathu {
  translate(rect: XywhRect, vector: Vector): XywhRect {
    return {
      x: rect.x + vector.x,
      y: rect.y + vector.y,
      width: rect.width,
      height: rect.height,
    };
  }
  contains(rect: XywhRect, point: Vector): boolean {
    return (
      this.in(point.x, rect.x, rect.x + rect.width) &&
      this.in(point.y, rect.y, rect.y + rect.height)
    );
  }
  clamp(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
  }
  in(value: number, min: number, max: number): boolean {
    return min <= value && value <= max;
  }
}
