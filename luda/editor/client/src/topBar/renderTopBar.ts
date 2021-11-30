import {
  BorderPosition,
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { AutoSaveState } from "../saver/ISaver";
import { SequenceListViewState } from "../sequenceListView/type";
import { TimelineState } from "../timeline/type";
import { renderGoBackButton } from "./renderGoBackButton";
import { TopBarState } from "./type";

export const renderTopBar: Render<
  {
    topBarState: TopBarState;
    timelineState: TimelineState;
    sequenceListViewState: SequenceListViewState;
  },
  {
    autoSave: AutoSaveState;
  }
> = ({ topBarState, timelineState, sequenceListViewState }, props) => {
  const { rect } = topBarState.layout;
  const margin = 4;
  const width = rect.width - 2 * margin;
  const height = rect.height - 2 * margin;
  const spacing = 4;

  return [
    Rect({
      ...rect,
      style: {
        stroke: {
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Black,
          width: 1,
        },
      },
    }),
    Translate(
      { x: margin, y: margin },
      renderGoBackButton(
        {
          timelineState,
          sequenceListViewState,
        },
        {
          width: 64,
          height,
        },
      ),
    ),
    Text({
      x: margin + 64 + spacing,
      y: margin + height / 2,
      align: TextAlign.left,
      baseline: TextBaseline.middle,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 14,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: timelineState.title!,
    }),
    Text({
      x: margin + width,
      y: margin + height / 2,
      align: TextAlign.right,
      baseline: TextBaseline.middle,
      fontType: {
        language: Language.ko,
        serif: false,
        fontWeight: FontWeight.regular,
        size: 14,
      },
      style: {
        color:
          props.autoSave === AutoSaveState.saved
            ? ColorUtil.Grayscale01(0.4)
            : ColorUtil.Red,
      },
      text: savingText(props.autoSave),
    }),
  ];
};

function savingText(autoSaveState: AutoSaveState): string {
  switch (autoSaveState) {
    case AutoSaveState.saving:
      return "Saving...";
    case AutoSaveState.saved:
      return "Up to date";
    case AutoSaveState.retryingOnError:
      return "Retrying on error...";
    case AutoSaveState.failToRecoverError:
      return "Fail to recover error. Please check the error!";
  }
}
