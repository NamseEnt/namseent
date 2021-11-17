import { Render } from "namui";
import { PlayerScreen } from "../livePlayer/playerScreen/PlayerScreen";
import { SequenceListViewState } from "./type";

export const renderSequencePreview: Render<
  SequenceListViewState,
  { width: number; height: number }
> = (state, props) => {
  const { width, height } = props;

  return [
    PlayerScreen(
      {},
      {
        layout: {
          x: (width - height * (16 / 9)) / 2,
          y: 0,
          width: height * (16 / 9),
          height,
        },
        livePlayerState: {
          anchorMs: state.preloadedSequence?.seekerMs || 0,
          isPlaying: false,
          playStartTimeMs: 0,
        },
        tracks: state.preloadedSequence?.tracks || [],
      },
    ),
  ];
};
