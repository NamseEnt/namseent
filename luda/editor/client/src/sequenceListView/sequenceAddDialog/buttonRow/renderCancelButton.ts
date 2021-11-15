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
} from "namui";
import { SequenceListViewState } from "../../type";

export const renderCancelButton: Render<
  SequenceListViewState,
  {
    width: number;
  }
> = (state, props) => {
  const { width } = props;
  const height = 36;

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        fill: {
          color: ColorUtil.Color0255(242, 38, 19),
        },
        stroke: {
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Color0255(255, 148, 120),
          width: 1,
        },
        round: {
          radius: 4,
        },
      },
      onClick: () => {
        state.addingSequence = false;
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
      text: "Cancel",
    }),
  ];
};
