import {
  ColorUtil,
  RenderingTree,
  Translate,
  XywhRect,
  BorderPosition,
} from "namui";
import { renderSlider } from "./renderSlider";

export function renderAlphaSlider(props: {
  layout: XywhRect;
  hue: number;
  saturation: number;
  lightness: number;
  alpha: number;
  onChange: (alpha: number) => void;
}): RenderingTree {
  const backgroundGradientPainter = new CanvasKit.Paint();
  backgroundGradientPainter.setShader(
    CanvasKit.Shader.MakeLinearGradient(
      [0, 0],
      [4, 4],
      [
        ColorUtil.ColorHSL01(0, 0, 1),
        ColorUtil.ColorHSL01(0, 0, 1),
        ColorUtil.ColorHSL01(0, 0, 0.5),
        ColorUtil.ColorHSL01(0, 0, 0.5),
      ],
      [0, 0.5, 0.5, 1],
      CanvasKit.TileMode.Repeat,
      undefined,
      0,
      undefined,
    ),
  );
  const gradientPainter = new CanvasKit.Paint();
  gradientPainter.setShader(
    CanvasKit.Shader.MakeLinearGradient(
      [0, 0],
      [props.layout.width, 0],
      [
        ColorUtil.ColorHSL01(props.hue, props.saturation, props.lightness, 0),
        ColorUtil.ColorHSL01(props.hue, props.saturation, props.lightness, 1),
      ],
      null,
      CanvasKit.TileMode.Clamp,
      undefined,
      0,
      undefined,
    ),
  );

  return Translate(props.layout, [
    {
      drawCalls: [
        {
          commands: [
            {
              type: "path",
              path: new CanvasKit.Path().addRect(
                CanvasKit.XYWHRect(
                  0,
                  0,
                  props.layout.width,
                  props.layout.height,
                ),
              ),
              paint: backgroundGradientPainter,
            },
          ],
        },
      ],
    },
    {
      drawCalls: [
        {
          commands: [
            {
              type: "path",
              path: new CanvasKit.Path().addRect(
                CanvasKit.XYWHRect(
                  0,
                  0,
                  props.layout.width,
                  props.layout.height,
                ),
              ),
              paint: gradientPainter,
            },
          ],
        },
      ],
    },
    renderSlider({
      layout: {
        ...props.layout,
        x: 0,
        y: 0,
      },
      max: 1,
      min: 0,
      onChange: (value) => props.onChange(value),
      style: {
        background: {
          fill: {
            color: ColorUtil.Transparent,
          },
        },
        thumb: {
          fill: {
            color: ColorUtil.ColorHSL01(
              props.hue,
              props.saturation,
              props.lightness,
              props.alpha,
            ),
          },
          stroke: {
            color: ColorUtil.ColorHSL01(
              props.hue,
              props.saturation,
              props.lightness,
            ),
            width: 4,
            borderPosition: BorderPosition.inside,
          },
        },
      },
      value: props.alpha,
    }),
  ]);
}
