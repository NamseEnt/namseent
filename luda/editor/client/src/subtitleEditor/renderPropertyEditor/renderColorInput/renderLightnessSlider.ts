import { ColorUtil, RenderingTree, Translate, XywhRect } from "namui";
import { renderSlider } from "./renderSlider";

export function renderLightnessSlider(props: {
  layout: XywhRect;
  lightness: number;
  onChange: (lightness: number) => void;
}): RenderingTree {
  const gradientPainter = new CanvasKit.Paint();
  gradientPainter.setShader(
    CanvasKit.Shader.MakeLinearGradient(
      [0, 0],
      [props.layout.width, 0],
      [ColorUtil.ColorHSL01(0, 0, 0), ColorUtil.ColorHSL01(0, 0, 1)],
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
            color: ColorUtil.ColorHSL01(0, 0, props.lightness),
          },
          stroke: {
            color: ColorUtil.ColorHSL01(0, 0, 1 - props.lightness),
            width: 4,
          },
        },
      },
      value: props.lightness,
    }),
  ]);
}
