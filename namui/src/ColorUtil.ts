import * as CanvasKit from "canvaskit-wasm";

export namespace ColorUtil {
  export function Color01(
    r: number,
    g: number,
    b: number,
    a?: number,
  ): CanvasKit.Color {
    if (a === undefined) {
      a = 1;
    }
    return Float32Array.of(r, g, b, a);
  }
  export function Color0255(
    r: number,
    g: number,
    b: number,
    a?: number,
  ): CanvasKit.Color {
    if (a === undefined) {
      a = 1;
    }
    return Color01(clamp(r) / 255, clamp(g) / 255, clamp(b) / 255, a);
  }
  export function Grayscale01(grayscale: number, a?: number): CanvasKit.Color {
    return Color01(grayscale, grayscale, grayscale, a);
  }
  export function Grayscale0255(
    grayscale: number,
    a?: number,
  ): CanvasKit.Color {
    return Color0255(grayscale, grayscale, grayscale, a);
  }
  function clamp(color0255: number) {
    return Math.round(Math.max(0, Math.min(color0255 || 0, 255)));
  }

  export const Red = Color01(1, 0, 0);
  export const Green = Color01(0, 1, 0);
  export const Blue = Color01(0, 0, 1);
  export const Black = Color01(0, 0, 0);
  export const White = Color01(1, 1, 1);
  export const Transparent = Color01(0, 0, 0, 0);
}
