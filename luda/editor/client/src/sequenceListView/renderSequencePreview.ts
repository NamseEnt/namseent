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
          x: 0,
          y: 0,
          width,
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
