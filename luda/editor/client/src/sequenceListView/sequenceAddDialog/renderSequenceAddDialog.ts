import { Render } from "namui";
import { renderRows } from "../../common/renderRows";
import { TimelineState } from "../../timeline/type";
import { SequenceListViewState } from "../type";
import { renderButtonRow } from "./buttonRow/renderButtonRow";
import { renderSequenceTitleInput } from "./renderSequenceTitleInput";

export const renderSequenceAddDialog: Render<
  {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  { width: number }
> = (state, props) => {
  const { width } = props;
  return renderRows(
    [
      {
        height: 32,
        renderingData: renderSequenceTitleInput(state.sequenceListView, {
          width,
        }),
      },
      {
        height: 36,
        renderingData: renderButtonRow(state, { width }),
      },
    ],
    8,
  );
};
