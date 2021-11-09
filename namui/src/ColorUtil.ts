import * as CanvasKit from "canvaskit-wasm";
import { Convert, Mathu } from ".";

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
    return Color01(
      Mathu.clamp(r, 0, 255) / 255,
      Mathu.clamp(g, 0, 255) / 255,
      Mathu.clamp(b, 0, 255) / 255,
      a,
    );
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
  export function ColorHSL01(h: number, s: number, l: number, a?: number) {
    const hue = (h * 360) % 360;
    const hueStage = hue / 60;
    const primaryChroma = (1 - Math.abs(2 * l - 1)) * s;
    const secondaryChroma = primaryChroma * (1 - Math.abs((hueStage % 2) - 1));
    const [baseR, baseG, baseB] =
      hueStage < 1
        ? [primaryChroma, secondaryChroma, 0]
        : hueStage < 2
        ? [secondaryChroma, primaryChroma, 0]
        : hueStage < 3
        ? [0, primaryChroma, secondaryChroma]
        : hueStage < 4
        ? [0, secondaryChroma, primaryChroma]
        : hueStage < 5
        ? [secondaryChroma, 0, primaryChroma]
        : hueStage < 6
        ? [primaryChroma, 0, secondaryChroma]
        : [0, 0, 0];
    const lightnessFactor = l - primaryChroma / 2;
    return Color01(
      baseR + lightnessFactor,
      baseG + lightnessFactor,
      baseB + lightnessFactor,
      a,
    );
  }

  export function brighterColor01(color: CanvasKit.Color, amount: number) {
    const { hue, saturation, lightness, alpha } = Convert.ColorToHsl(color);

    return ColorUtil.ColorHSL01(
      hue,
      Mathu.clamp(saturation - amount, 0, 1),
      Mathu.clamp(lightness + amount, 0, 1),
      alpha,
    );
  }

  export const Red = Color01(1, 0, 0);
  export const Green = Color01(0, 1, 0);
  export const Blue = Color01(0, 0, 1);
  export const Black = Color01(0, 0, 0);
  export const White = Color01(1, 1, 1);
  export const Transparent = Color01(0, 0, 0, 0);
}
