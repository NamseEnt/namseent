import {
  ColorUtil,
  RenderingTree,
  Translate,
  XywhRect,
  BorderPosition,
} from "namui";
import { renderSlider } from "./renderSlider";

export function renderSaturationSlider(props: {
  layout: XywhRect;
  hue: number;
  saturation: number;
  onChange: (saturation: number) => void;
}): RenderingTree {
  const gradientPainter = new CanvasKit.Paint();
  gradientPainter.setShader(
    CanvasKit.Shader.MakeLinearGradient(
      [0, 0],
      [props.layout.width, 0],
      [
        ColorUtil.ColorHSL01(props.hue, 0, 0.5),
        ColorUtil.ColorHSL01(props.hue, 1, 0.5),
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
            color: ColorUtil.ColorHSL01(props.hue, props.saturation, 0.5),
          },
          stroke: {
            color: ColorUtil.ColorHSL01(props.hue, props.saturation, 0.8),
            width: 4,
            borderPosition: BorderPosition.inside,
          },
        },
      },
      value: props.saturation,
    }),
  ]);
}
