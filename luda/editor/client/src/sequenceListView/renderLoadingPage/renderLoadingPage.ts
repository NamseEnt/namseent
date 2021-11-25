import { ColorUtil, Rect, Render } from "namui";
import { SequenceListViewState } from "../type";

export const renderLoadingPage: Render<
  SequenceListViewState,
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { loadingSequence } = state;
  const loadingSequenceState = loadingSequence?.state;

  const isLoadingOrFailed =
    loadingSequenceState &&
    (loadingSequenceState.type === "loading" ||
      loadingSequenceState.type === "failed");

  if (!isLoadingOrFailed) {
    return undefined;
  }

  switch (loadingSequenceState.type) {
    case "loading":
      {
      }
      break;
    case "failed":
      {
        loadingSequenceState.errorCode;
      }
      break;
  }
  // const { isLoading, errorCode } = loadingSequence;

  const { width, height } = props;
  const margin = 16;

  return [
    Rect({
      x: 0,
      y: 0,
      width,
      height,
      style: {
        fill: {
          color: ColorUtil.Color01(0, 0, 0, 0.8),
        },
      },
    }),
  ];
};
