import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
  BorderPosition,
} from "namui";
import { renderAlphaSlider } from "./renderAlphaSlider";
import { renderRows } from "../renderRows";
import { ColorInputState, SubtitleEditorState } from "../../type";
import { renderHueSlider } from "./renderHueSlider";
import { renderSaturationSlider } from "./renderSaturationSlider";
import { renderLightnessSlider } from "./renderLightnessSlider";
import { convertColorToHsl } from "../../../util/color/convertColorToHsl";

export function renderColorInput(
  props: {
    width: number;
    label: string;
    value: Float32Array;
    colorInputId: string;
    onChange: (color: Float32Array) => void;
  },
  state: ColorInputState,
): RenderingTree {
  const borderWidth = 1;
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

  return [
    Text({
      x: 0,
      y: 0,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 12,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: props.label,
    }),
    {
      drawCalls: [
        {
          commands: [
            {
              type: "path",
              path: new CanvasKit.Path().addRect(
                CanvasKit.XYWHRect(
                  150 + borderWidth,
                  0 + borderWidth,
                  20 - 2 * borderWidth,
                  20 - 2 * borderWidth,
                ),
              ),
              paint: backgroundGradientPainter,
            },
          ],
        },
      ],
    },
    Rect({
      x: 150 + borderWidth,
      y: 0 + borderWidth,
      width: 20 - 2 * borderWidth,
      height: 20 - 2 * borderWidth,
      style: {
        fill: {
          color: props.value,
        },
        stroke: {
          color:
            state.targetId === props.colorInputId
              ? ColorUtil.Blue
              : ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
      onClick() {
        if (state.targetId !== props.colorInputId) {
          const { hue, saturation, lightness, alpha } = convertColorToHsl(
            props.value,
          );
          state.targetId = props.colorInputId;
          state.hue = hue;
          state.saturation = saturation;
          state.lightness = lightness;
          state.alpha = alpha;
        } else {
          state.targetId = undefined;
        }
      },
    }),
    state.targetId === props.colorInputId
      ? Translate(
          { x: 0, y: 24 },
          renderRows([
            {
              height: 20,
              renderingData: renderHueSlider({
                hue: state.hue,
                layout: {
                  x: 0,
                  y: 0,
                  width: props.width,
                  height: 20,
                },
                onChange: (hue) => {
                  state.hue = hue;
                  props.onChange(
                    ColorUtil.ColorHSL01(
                      state.hue,
                      state.saturation,
                      state.lightness,
                      state.alpha,
                    ),
                  );
                },
              }),
            },
            {
              height: 20,
              renderingData: renderSaturationSlider({
                saturation: state.saturation,
                hue: state.hue,
                layout: {
                  x: 0,
                  y: 0,
                  width: props.width,
                  height: 20,
                },
                onChange: (value) => {
                  state.saturation = value;
                  props.onChange(
                    ColorUtil.ColorHSL01(
                      state.hue,
                      state.saturation,
                      state.lightness,
                      state.alpha,
                    ),
                  );
                },
              }),
            },
            {
              height: 20,
              renderingData: renderLightnessSlider({
                lightness: state.lightness,
                layout: {
                  x: 0,
                  y: 0,
                  width: props.width,
                  height: 20,
                },
                onChange: (value) => {
                  state.lightness = value;
                  props.onChange(
                    ColorUtil.ColorHSL01(
                      state.hue,
                      state.saturation,
                      state.lightness,
                      state.alpha,
                    ),
                  );
                },
              }),
            },
            {
              height: 20,
              renderingData: renderAlphaSlider({
                hue: state.hue,
                saturation: state.saturation,
                lightness: state.lightness,
                alpha: state.alpha,
                layout: {
                  x: 0,
                  y: 0,
                  width: props.width,
                  height: 20,
                },
                onChange: (value) => {
                  state.alpha = value;
                  props.onChange(
                    ColorUtil.ColorHSL01(
                      state.hue,
                      state.saturation,
                      state.lightness,
                      state.alpha,
                    ),
                  );
                },
              }),
            },
          ]),
        )
      : undefined,
  ];
}
