import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { renderRows } from "../../common/renderRows";
import { preloadSequence } from "../operations/preloadSequence";
import { SequenceListViewState } from "../type";

export function generateSequenceListRows(
  state: SequenceListViewState,
  props: {
    sequenceTitles: string[];
    width: number;
  },
): Parameters<typeof renderRows>[0] {
  const { preloadedSequence } = state;
  const { sequenceTitles, width } = props;
  const margin = 4;
  const spacing = 4;
  const titleHeight = 32;
  const previewSliderWidth = width - 2 * margin;
  const previewSliderHeight = 32;

  return sequenceTitles.map((title) => {
    const selected = preloadedSequence?.title === title;
    const height =
      2 * margin +
      (selected ? titleHeight + previewSliderHeight + spacing : titleHeight);

    return {
      height,
      renderingData: [
        Rect({
          x: 0,
          y: 0,
          width,
          height,
          style: {
            fill: {
              color: ColorUtil.Grayscale01(0.3),
            },
            round: {
              radius: 4,
            },
          },
          onClick: () => {
            if (state.preloadedSequence?.title !== title) {
              preloadSequence(state, title);
            }
          },
        }),
        Text({
          x: margin,
          y: titleHeight / 2 + margin,
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
          text: title,
        }),
        selected
          ? Translate(
              {
                x: margin,
                y: margin + titleHeight + spacing,
              },
              [
                Rect({
                  x: 0,
                  y: 0,
                  width: previewSliderWidth,
                  height: previewSliderHeight,
                  style: {
                    fill: {
                      color: ColorUtil.Grayscale01(0.5),
                    },
                    round: {
                      radius: previewSliderHeight / 2,
                    },
                  },
                  onMouseMoveIn: (event) => {
                    if (
                      preloadedSequence.isLoading ||
                      !preloadedSequence.isSequence
                    ) {
                      return;
                    }
                    preloadedSequence.seekerMs =
                      (preloadedSequence.lengthMs * event.translated.x) /
                      previewSliderWidth;
                  },
                }),
                preloadedSequence.isLoading || !preloadedSequence.isSequence
                  ? Text({
                      x: previewSliderWidth / 2,
                      y: previewSliderHeight / 2,
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
                        ((previewSliderWidth - previewSliderHeight) *
                          preloadedSequence.seekerMs) /
                        preloadedSequence.lengthMs,
                      y: 0,
                      width: previewSliderHeight,
                      height: previewSliderHeight,
                      style: {
                        fill: {
                          color: ColorUtil.Grayscale01(1),
                        },
                        round: {
                          radius: previewSliderHeight / 2,
                        },
                      },
                    }),
              ],
            )
          : undefined,
      ],
    };
  });
}
