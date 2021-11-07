import {
  BorderPosition,
  Clip,
  ColorUtil,
  Mathu,
  Rect,
  Render,
  Translate,
} from "namui";
import { getBigGradationXs, Gradations } from "./Gradations";
import { TimeTexts } from "./TimeTexts";
import { TimeRulerState, TimeRulerProps } from "./type";
export const TimeRuler: Render<TimeRulerState, TimeRulerProps> = (
  state,
  props,
) => {
  const gradationGapPx = getGradationGapPx({
    min: 20,
    max: 100,
    msPerPixel: props.msPerPixel,
  });
  const gradationGapMs = gradationGapPx * props.msPerPixel;
  const gradationStartMs = -(props.startMs % (gradationGapMs * 5));
  const gradationStartPx = gradationStartMs / props.msPerPixel;

  return Translate({ ...props.layout }, [
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(0, 0, props.layout.width, props.layout.height),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      [
        Rect({
          ...props.layout,
          x: 0,
          y: 0,
          style: {
            stroke: {
              borderPosition: BorderPosition.inside,
              color: ColorUtil.Black,
              width: 1,
            },
            fill: {
              color: ColorUtil.White,
            },
          },
        }),
        TimeTexts(
          {},
          {
            ...props,
            bigGradationXs: getBigGradationXs({
              gradationGapPx,
              gradationStartPx,
              timeRulerWidth: props.layout.width,
            }),
          },
        ),
        Gradations(
          {},
          {
            ...props,
            gradationGapPx,
            gradationStartPx,
          },
        ),
      ],
    ),
  ]);
};

function getGradationGapPx({
  min,
  max,
  msPerPixel,
}: {
  min: number;
  max: number;
  msPerPixel: number;
}): number {
  const gradationGapMs = [
    100, 250, 500, 1000, 5000, 10000, 30000, 60000, 300000, 600000, 1800000,
  ]
    .map((ms) => ms / 5)
    .find((ms) => {
      const px = ms / msPerPixel;
      return Mathu.in(px, min, max);
    });

  if (gradationGapMs) {
    return gradationGapMs / msPerPixel;
  }

  return max;
}
