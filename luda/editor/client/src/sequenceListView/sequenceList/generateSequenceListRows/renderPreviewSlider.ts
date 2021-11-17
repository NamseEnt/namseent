import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";
import { SequenceListViewState } from "../../type";

export const renderPreviewSlider: Render<
  SequenceListViewState,
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { width, height } = props;
  const { preloadedSequence } = state;

  if (!preloadedSequence) {
    return;
  }

  const loadable = !preloadedSequence.isLoading && preloadedSequence.isSequence;

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        fill: {
          color: ColorUtil.Grayscale01(0.5),
        },
        round: {
          radius: height / 2,
        },
      },
      onMouseMoveIn: (event) => {
        if (preloadedSequence.isLoading || !preloadedSequence.isSequence) {
          return;
        }
        preloadedSequence.seekerMs =
          (preloadedSequence.lengthMs * event.translated.x) / width;
      },
    }),
    !loadable
      ? Text({
          x: width / 2,
          y: height / 2,
          align: TextAlign.left,
          baseline: TextBaseline.middle,
          fontType: {
            language: Language.ko,
            serif: false,
            fontWeight: FontWeight.regular,
            size: 20,
          },
          style: {
            color: ColorUtil.White,
          },
          text: preloadedSequence.isLoading
            ? "Loading..."
            : "It's not sequence file",
        })
      : Rect({
          x:
            ((width - height) * preloadedSequence.seekerMs) /
            preloadedSequence.lengthMs,
          y: 0,
          width: height,
          height,
          style: {
            fill: {
              color: ColorUtil.Grayscale01(1),
            },
            round: {
              radius: height / 2,
            },
          },
        }),
  ];
};
