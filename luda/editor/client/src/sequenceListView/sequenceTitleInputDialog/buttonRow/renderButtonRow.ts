import { Render, Translate } from "namui";
import { TimelineSequenceNullableState } from "../../../timeline/type";
import { SequenceListViewState } from "../../type";
import { renderCancelButton } from "./renderCancelButton";
import { renderOkButton } from "./renderOkButton";

export const renderButtonRow: Render<
  {
    timeline: TimelineSequenceNullableState;
    sequenceListView: SequenceListViewState;
  },
  { width: number }
> = (state, props) => {
  const spacing = 8;
  const width = (props.width - spacing) / 2;

  return [
    renderOkButton(state, { width }),
    Translate(
      {
        x: width + spacing,
        y: 0,
      },
      renderCancelButton(state.sequenceListView, { width }),
    ),
  ];
};
