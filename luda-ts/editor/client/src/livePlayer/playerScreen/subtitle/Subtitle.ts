import {
  Clip,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Vector,
  WhSize,
} from "namui";
import { SubtitleFontType, SubtitleStyle } from "../../../type";

export type Subtitle = {
  text: string;
  fontType: SubtitleFontType;
  style: SubtitleStyle;
};

export const Subtitle: Render<
  {},
  {
    whSize: WhSize;
    subtitle: Subtitle;
    /**
     * Start from 0.
     * 0 is bottom.
     */
    lineIndex: number;
  }
> = (state, props) => {
  const {
    subtitle,
    whSize: { width, height },
  } = props;
  const screenSizeRelativeRatio = width / 1080;
  const fontSize = props.subtitle.fontType.size * screenSizeRelativeRatio;

  const lastSubtitleCenterVector = new Vector(width / 2, (height * 4) / 5);
  const lineHeightRate = 1.5;
  const subtitleCenterVector = lastSubtitleCenterVector.translate(
    0,
    -fontSize * props.lineIndex * lineHeightRate,
  );

  return [
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(0, 0, width, height),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      [
        Text({
          ...subtitleCenterVector,
          align: TextAlign.center,
          baseline: TextBaseline.bottom,
          fontType: {
            ...props.subtitle.fontType,
            size: fontSize,
          },
          style: {
            ...props.subtitle.style,
            border: {
              ...props.subtitle.style.border,
              width:
                props.subtitle.style.border.width * screenSizeRelativeRatio,
            },
            dropShadow: {
              ...props.subtitle.style.dropShadow,
              x: props.subtitle.style.dropShadow.x * screenSizeRelativeRatio,
              y: props.subtitle.style.dropShadow.y * screenSizeRelativeRatio,
            },
          },
          text: props.subtitle.text,
        }),
      ],
    ),
  ];
};
