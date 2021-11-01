export interface IMathu {
  clamp(value: number, min: number, max: number): number;
  in(value: number, min: number, max: number): boolean;
}
