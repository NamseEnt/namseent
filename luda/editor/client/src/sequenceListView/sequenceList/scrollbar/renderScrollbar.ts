import { Mathu, Render } from "namui";
import { SequenceListViewState } from "../../type";
import { renderThumb } from "./renderThumb";

export const renderScrollbar: Render<
  SequenceListViewState,
  { width: number; height: number; contentHeight: number }
> = (state, props) => {
  const { width, height, contentHeight } = props;
  const { sequenceListScrollY } = state;

  const thumbHeight = Mathu.clamp(
    (height / contentHeight) * height,
    width * 2,
    height,
  );
  const thumbY =
    (height - thumbHeight) *
    Mathu.clamp(sequenceListScrollY / (contentHeight - height), 0, 1);

  return [
    renderThumb(
      {},
      {
        y: thumbY,
        width,
        height: thumbHeight,
      },
    ),
  ];
};
