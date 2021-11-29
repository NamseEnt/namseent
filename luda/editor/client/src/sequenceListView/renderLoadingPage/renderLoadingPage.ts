import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
} from "namui";
import { initTimeline } from "../../timeline/operations/initTimeline";
import { TimelineSequenceNullableState } from "../../timeline/type";
import { LoadSequence } from "../operations/loadSequence";
import { SequenceListViewState } from "../type";
import { renderCloseButton } from "./renderCloseButton";

export const renderLoadingPage: Render<
  {
    sequenceListViewState: SequenceListViewState;
    timelineState: TimelineSequenceNullableState;
  },
  {
    width: number;
    height: number;
  }
> = ({ sequenceListViewState, timelineState }, props) => {
  const { loadingSequence } = sequenceListViewState;
  const loadingSequenceState = loadingSequence?.state;

  if (!loadingSequenceState) {
    return undefined;
  }

  const loaded = loadingSequenceState.type === "loaded";
  const failed = loadingSequenceState.type === "failed";

  if (loaded) {
    initTimeline(
      timelineState,
      loadingSequenceState.tracks,
      loadingSequence.title,
    );
    sequenceListViewState.loadingSequence = undefined;
  }

  const { width, height } = props;
  const margin = 16;
  const buttonWidth = width - 2 * margin;
  const buttonHeight = 36;
  const textHeightHalf = 10;

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
    Text({
      x: width / 2,
      y: height / 2 - textHeightHalf,
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
      text: infoText(loadingSequenceState),
    }),
    failed
      ? [
          Text({
            x: width / 2,
            y: height / 2 + textHeightHalf,
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
            text: `ErrorCode: ${loadingSequenceState.errorCode}`,
          }),

          Translate(
            {
              x: margin,
              y: height - buttonHeight - margin,
            },
            renderCloseButton(sequenceListViewState, {
              width: buttonWidth,
              height: buttonHeight,
            }),
          ),
        ]
      : undefined,
  ];
};

function infoText(loadSequenceState: LoadSequence.LoadSequenceState) {
  switch (loadSequenceState.type) {
    case "loading": {
      return "Loading...";
    }

    case "loaded": {
      return "Loaded. Timeline editor will open soon.";
    }

    case "failed": {
      return "Failed to load. Try again.";
    }
  }
}
