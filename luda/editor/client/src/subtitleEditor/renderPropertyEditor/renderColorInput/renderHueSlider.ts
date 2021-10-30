import { ColorUtil, RenderingTree, Translate, XywhRect, Rect } from "namui";
import { renderSlider } from "./renderSlider";

export function renderHueSlider(props: {
  layout: XywhRect;
  hue: number;
  onChange: (hue: number) => void;
}): RenderingTree {
  const gradientPainter = new CanvasKit.Paint();
  gradientPainter.setShader(
    CanvasKit.Shader.MakeLinearGradient(
      [0, 0],
      [props.layout.width, 0],
      [
        ColorUtil.ColorHSL01(0 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(1 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(2 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(3 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(4 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(5 / 6, 1, 0.5),
        ColorUtil.ColorHSL01(6 / 6, 1, 0.5),
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
            color: ColorUtil.ColorHSL01(props.hue, 1, 0.5),
          },
          stroke: {
            color: ColorUtil.ColorHSL01(props.hue, 1, 0.8),
            width: 4,
          },
        },
      },
      value: props.hue,
    }),
  ]);
}
