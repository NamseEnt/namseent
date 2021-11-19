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
import { renderRemoveButton } from "./renderRemoveButton";
import { renderRenameButton } from "./renderRenameButton";
import { renderTimeText } from "./renderTimeText";

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
  const sequenceLengthHeight = 18;
  const removeButtonHeight = 36;
  const renameButtonHeight = 36;
  const loadButtonHeight = 36;

  return sequenceTitles.map((title) => {
    const selected = preloadedSequence?.title === title;
    const loadable =
      preloadedSequence &&
      !preloadedSequence.isLoading &&
      preloadedSequence.isSequence;

    const rows: Parameters<typeof renderRows>[0] = [
      {
        height: titleHeight,
        renderingData: Text({
          x: 0,
          y: titleHeight / 2,
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
      },
      {
        height: selected ? previewSliderHeight : 0,
        renderingData: selected
          ? renderPreviewSlider(sequenceListView, {
              width: previewSliderWidth,
              height: previewSliderHeight,
            })
          : undefined,
      },
      {
        height: selected ? sequenceLengthHeight : 0,
        renderingData: selected
          ? renderTimeText(
              {},
              {
                width: previewSliderWidth,
                height: sequenceLengthHeight,
                timeMs: preloadedSequence?.lengthMs || 0,
              },
            )
          : undefined,
      },
      {
        height: selected ? renameButtonHeight : 0,
        renderingData: selected
          ? renderRenameButton(sequenceListView, {
              width: previewSliderWidth,
              height: renameButtonHeight,
            })
          : undefined,
      },
      {
        height: selected && loadable ? loadButtonHeight : 0,
        renderingData:
          selected && loadable
            ? renderLoadButton(state, {
                width: previewSliderWidth,
                height: loadButtonHeight,
                title,
              })
            : undefined,
      },
      {
        height: selected ? removeButtonHeight : 0,
        renderingData: selected
          ? renderRemoveButton(sequenceListView, {
              width: previewSliderWidth,
              height: removeButtonHeight,
            })
          : undefined,
      },
    ];

    const contentHeight = rows.reduce(
      (contentHeight, row) =>
        contentHeight + (row.height ? row.height + spacing : 0),
      -spacing,
    );

    return {
      height: contentHeight + 2 * margin,
      renderingData: [
        Rect({
          x: 0,
          y: 0,
          width,
          height: contentHeight + 2 * margin,
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
        Translate(
          {
            x: margin,
            y: margin,
          },
          renderRows(rows, spacing),
        ),
      ],
    };
  });
}
