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
import { renderRows } from "../../../common/renderRows";
import { TimelineState } from "../../../timeline/type";
import { preloadSequence } from "../../operations/preloadSequence";
import { SequenceListViewState } from "../../type";
import { renderLoadButton } from "./renderLoadButton";
import { renderPreviewSlider } from "./renderPreviewSlider";

export function generateSequenceListRows(
  state: {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  props: {
    sequenceTitles: string[];
    width: number;
  },
): Parameters<typeof renderRows>[0] {
  const { sequenceListView } = state;
  const { preloadedSequence } = sequenceListView;
  const { sequenceTitles, width } = props;
  const margin = 4;
  const spacing = 4;
  const titleHeight = 32;
  const previewSliderWidth = width - 2 * margin;
  const previewSliderHeight = 32;
  const loadButtonHeight = 36;

  return sequenceTitles.map((title) => {
    const selected = preloadedSequence?.title === title;
    const loadable =
      preloadedSequence &&
      !preloadedSequence.isLoading &&
      preloadedSequence.isSequence;
    const height =
      2 * margin +
      titleHeight +
      (selected ? previewSliderHeight + spacing : 0) +
      (selected && loadable ? loadButtonHeight + spacing : 0);

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
            if (preloadedSequence?.title !== title) {
              preloadSequence(sequenceListView, title);
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
              renderPreviewSlider(sequenceListView, {
                width: previewSliderWidth,
                height: previewSliderHeight,
              }),
            )
          : undefined,

        selected && loadable
          ? Translate(
              {
                x: margin,
                y: margin + titleHeight + previewSliderHeight + 2 * spacing,
              },
              renderLoadButton(state, {
                width: previewSliderWidth,
                height: loadButtonHeight,
                title,
              }),
            )
          : undefined,
      ],
    };
  });
}
