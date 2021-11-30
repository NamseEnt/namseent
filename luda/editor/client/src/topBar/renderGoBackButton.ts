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
import { loadSequenceTitles } from "../sequenceListView/operations/loadSequenceTitles";
import { SequenceListViewState } from "../sequenceListView/type";
import { initTimeline } from "../timeline/operations/initTimeline";
import { TimelineState } from "../timeline/type";

export const renderGoBackButton: Render<
  {
    timelineState: TimelineState;
    sequenceListViewState: SequenceListViewState;
  },
  {
    width: number;
    height: number;
  }
> = ({ timelineState, sequenceListViewState }, props) => {
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
        initTimeline(timelineState);
        loadSequenceTitles(sequenceListViewState);
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
        size: 14,
      },
      style: {
        color: ColorUtil.White,
      },
      text: "Go Back",
    }),
  ];
};
