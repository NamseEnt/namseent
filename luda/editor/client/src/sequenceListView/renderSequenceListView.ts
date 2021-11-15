import { BorderPosition, ColorUtil, Rect, Render } from "namui";
import { TimelineState } from "../timeline/type";
import { SequenceListViewState } from "./type";

export const renderSequenceListView: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  {}
> = (state, props) => {
  return [
    Rect({
      ...state.sequenceListView.layout.rect,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
  ];
};
