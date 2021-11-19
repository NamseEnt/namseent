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
import { loadSequenceTitles } from "../../operations/loadSequenceTitles";
import { removeSequence } from "../../operations/removeSequence";
import { SequenceListViewState } from "../../type";

export const renderRemoveButton: Render<
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
      onClick: async () => {
        const title = state.preloadedSequence?.title;
        if (!title) {
          return;
        }
        await removeSequence(title);
        await loadSequenceTitles(state);
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
      text: "Remove Sequence",
    }),
  ];
};
