import { IMathu } from "./IMathu";

export class Mathu implements IMathu {
  clamp(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
  }
  in(value: number, min: number, max: number): boolean {
    return min <= value && value <= max;
  }
}
