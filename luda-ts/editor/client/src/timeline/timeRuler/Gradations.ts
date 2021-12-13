import { Render, RenderingData, ColorUtil, PathDrawCommand } from "namui";
import { TimeRulerProps } from "./type";

export const Gradations: Render<
  {},
  TimeRulerProps & {
    gradationStartPx: number;
    gradationGapPx: number;
  }
> = (state, props) => {
  const bigGradationHeight = (props.layout.height * 2) / 3;
  const smallGradationHeight = bigGradationHeight / 3;

  const bigGradationPaint = new CanvasKit.Paint();
  bigGradationPaint.setColor(ColorUtil.Grayscale01(0.5));
  bigGradationPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  bigGradationPaint.setStrokeWidth(2);

  const smallGradationPaint = new CanvasKit.Paint();
  smallGradationPaint.setColor(ColorUtil.Grayscale01(0.5));
  smallGradationPaint.setStyle(CanvasKit.PaintStyle.Stroke);
  smallGradationPaint.setStrokeWidth(1);

  const bigGradationXs = getBigGradationXs({
    gradationGapPx: props.gradationGapPx,
    gradationStartPx: props.gradationStartPx,
    timeRulerWidth: props.layout.width,
  });

  const gradationProperties: {
    x: number;
    isBig: boolean;
  }[] = [];

  bigGradationXs.forEach((bigGradationX) => {
    gradationProperties.push({
      x: bigGradationX,
      isBig: true,
    });
    for (let i = 1; i < 5; i += 1) {
      gradationProperties.push({
        x: bigGradationX + i * props.gradationGapPx,
        isBig: false,
      });
    }
  });

  return gradationProperties.map(({ isBig, x }) => {
    const gradationHeight = isBig ? bigGradationHeight : smallGradationHeight;

    const path = new CanvasKit.Path();
    path.moveTo(x, (props.layout.height - gradationHeight) / 2);
    path.lineTo(x, props.layout.height);

    const gradation: RenderingData = {
      drawCalls: [
        {
          commands: [
            PathDrawCommand({
              path,
              paint: isBig ? bigGradationPaint : smallGradationPaint,
            }),
          ],
        },
      ],
    };
    return gradation;
  });
};

export function getBigGradationXs({
  timeRulerWidth,
  gradationStartPx,
  gradationGapPx,
}: {
  timeRulerWidth: number;
  gradationStartPx: number;
  gradationGapPx: number;
}): number[] {
  const bigGradationXs: number[] = [];
  for (let x = gradationStartPx; x < timeRulerWidth; x += gradationGapPx * 5) {
    bigGradationXs.push(x);
  }
  return bigGradationXs;
}
