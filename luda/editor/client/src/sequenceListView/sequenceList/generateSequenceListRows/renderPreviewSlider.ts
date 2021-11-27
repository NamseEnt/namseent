import {
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
} from "namui";
import { LoadSequence } from "../../operations/loadSequence";
import { SequenceListViewState } from "../../type";

export const renderPreviewSlider: Render<
  SequenceListViewState,
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { width, height } = props;
  const { preloadedSequence } = state;

  if (!preloadedSequence) {
    return;
  }

  const loadSequenceState = preloadedSequence.state;
  if (!loadSequenceState) {
    return;
  }

  const loaded = loadSequenceState.type === "loaded";

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
        round: {
          radius: height / 2,
        },
      },
      onMouseMoveIn: (event) => {
        if (!loaded || !preloadedSequence.lengthMs) {
          return;
        }
        preloadedSequence.seekerMs =
          (preloadedSequence.lengthMs * event.translated.x) / width;
      },
    }),
    !loaded
      ? Text({
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
          text: infoText(loadSequenceState),
        })
      : preloadedSequence.lengthMs
      ? Rect({
          x:
            ((width - height) * preloadedSequence.seekerMs) /
            preloadedSequence.lengthMs,
          y: 0,
          width: height,
          height,
          style: {
            fill: {
              color: ColorUtil.Grayscale01(1),
            },
            round: {
              radius: height / 2,
            },
          },
        })
      : undefined,
  ];
};

function infoText(loadSequenceState: LoadSequence.LoadSequenceState) {
  switch (loadSequenceState.type) {
    case "loading": {
      return "Loading...";
    }

    case "failed": {
      return `Error: ${loadSequenceState.errorCode}`;
    }

    default: {
      throw new Error("Impossible state");
    }
  }
}
