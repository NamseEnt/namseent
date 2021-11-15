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
import { TimelineState, TrackType } from "../../../timeline/type";
import { SequenceListViewState } from "../../type";

export const renderOkButton: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  {
    width: number;
  }
> = (state, props) => {
  const { sequenceListView, timeline } = state;
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
        sequenceListView.editingFileName = sequenceListView.newTitle;
        timeline.tracks = [
          {
            id: "camera",
            type: TrackType.camera,
            clips: [],
          },
          {
            id: "subtitle",
            type: TrackType.subtitle,
            clips: [],
          },
        ];
        sequenceListView.addingSequence = false;
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
      text: "Ok",
    }),
  ];
};
