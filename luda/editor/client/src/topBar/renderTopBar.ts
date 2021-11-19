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
import { SequenceListViewState } from "../sequenceListView/type";
import { renderGoBackButton } from "./renderGoBackButton";
import { TopBarState } from "./type";

export const renderTopBar: Render<
  { topBar: TopBarState; sequenceListView: SequenceListViewState },
  {}
> = (state, props) => {
  const { rect } = state.topBar.layout;
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
      renderGoBackButton(state.sequenceListView, {
        width: 64,
        height,
      }),
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
      text: state.sequenceListView.editingSequenceTitle!,
    }),
  ];
};
