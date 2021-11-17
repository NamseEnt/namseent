import { BorderPosition, ColorUtil, Mathu, Rect, Render } from "namui";
import { SequenceListViewState } from "../../type";
import { renderThumb } from "./renderThumb";

export const renderScrollbar: Render<
  SequenceListViewState,
  { width: number; height: number; contentHeight: number }
> = (state, props) => {
  const { width, height, contentHeight } = props;
  const { sequenceListScrollY } = state;

  const borderWidth = 1;
  const thumbHeight = Mathu.clamp(
    (height / contentHeight) * height,
    width * 2,
    height,
  );
  const thumbY =
    (height - thumbHeight) *
    Mathu.clamp(sequenceListScrollY / (contentHeight - height), 0, 1);

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        fill: {
          color: ColorUtil.Grayscale01(0.5),
        },
      },
    }),
    renderThumb(
      {},
      {
        y: thumbY,
        width,
        height: thumbHeight,
      },
    ),
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        stroke: {
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Grayscale01(0.5),
          width: borderWidth,
        },
      },
    }),
  ];
};
