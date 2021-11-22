import {
  BorderPosition,
  ColorUtil,
  Cursor,
  engine,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";
import { SequenceListViewActionState, SequenceListViewState } from "../../type";

export const renderRenameButton: Render<
  SequenceListViewState,
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { width, height } = props;

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        fill: {
          color: ColorUtil.Color0255(107, 185, 240),
        },
        stroke: {
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Color0255(228, 241, 254),
          width: 1,
        },
        round: {
          radius: 4,
        },
      },
      onClick: () => {
        state.actionState = SequenceListViewActionState.renameSequence;
        state.newTitle = state.preloadedSequence?.title || state.newTitle;
      },
      onMouseIn: () => {
        engine.mousePointer.setCursor(Cursor.pointer);
      },
    }),
    Text({
      x: width / 2,
      y: height / 2,
      align: TextAlign.center,
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
      text: "Rename Sequence",
    }),
  ];
};
