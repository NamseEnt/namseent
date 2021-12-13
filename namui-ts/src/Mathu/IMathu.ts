import { Vector, XywhRect } from "..";

export interface IMathu {
  clamp(value: number, min: number, max: number): number;
  in(value: number, min: number, max: number): boolean;
  translate(rect: XywhRect, vector: Vector): XywhRect;
  contains(rect: XywhRect, point: Vector): boolean;
}
