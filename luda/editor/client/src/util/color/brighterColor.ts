import { ColorUtil } from "namui";
import { clamp } from "../clamp";
import { convertColorToHsl } from "./convertColorToHsl";

export function brighterColor01(color: Float32Array, amount: number) {
  const { hue, saturation, lightness, alpha } = convertColorToHsl(color);

  return ColorUtil.ColorHSL01(
    hue,
    saturation,
    clamp(lightness + amount),
    alpha,
  );
}
