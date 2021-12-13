import { Render, RenderingTree } from "namui";
import { renderCameraAngleEditor } from "./cameraAngleEditor/renderCameraAngleEditor";
import {
  CameraAngleEditorState,
  CameraAngleEditorWithoutCameraAngleState,
} from "./cameraAngleEditor/type";
import { isCameraClip, isSubtitleClip } from "./clipTypeGuard";
import { renderSubtitleEditor } from "./subtitleEditor/renderSubtitleEditor";
import { LivePlayerProps, LivePlayer } from "./livePlayer/LivePlayer";
import { changeLivePlayerPlaybackTime } from "./livePlayer/operations/changeLivePlayerPlaybackTime";
import { getPlaybackTimeMs } from "./livePlayer/operations/getPlaybackTimeMs";
import { LivePlayerState } from "./livePlayer/type";
import {
  SubtitleEditorState,
  SubtitleEditorWithoutSubtitleState,
} from "./subtitleEditor/type";
import { Timeline } from "./timeline/Timeline";
import { TimelineState } from "./timeline/type";
import { saver } from "./saver/saver";
import { getCurrentState } from "history";

type State = {
  timelineState: TimelineState;
  cameraAngleEditorWithoutCameraAngleState: CameraAngleEditorWithoutCameraAngleState;
  subtitleEditorWithoutSubtitleState: SubtitleEditorWithoutSubtitleState;
  livePlayer: {
    state: LivePlayerState;
    layout: LivePlayerProps["layout"];
  };
};

export function render(state: State): RenderingTree {
  const sequence = getCurrentState(state.timelineState.history);
  saver.autoSave("/sequence/sequence1.json", sequence);

  return [
    ClipEditor(state),
    Timeline(state.timelineState, {
      changePlaybackTimeMs(playbackTimeMs) {
        changeLivePlayerPlaybackTime(state.livePlayer.state, playbackTimeMs);
      },
      playbackTimeMs: getPlaybackTimeMs(state.livePlayer.state),
    }),
    LivePlayer(state.livePlayer.state, {
      layout: state.livePlayer.layout,
      tracks: sequence.tracks,
    }),
  ];
}

const ClipEditor: Render<State> = (state) => {
  const { selectedClip } = state.timelineState;
  if (!selectedClip) {
    return;
  }

  if (isCameraClip(selectedClip)) {
    const cameraAngleEditorState: CameraAngleEditorState = Object.assign(
      state.cameraAngleEditorWithoutCameraAngleState,
      {
        cameraAngle: selectedClip.cameraAngle,
      },
    );
    return renderCameraAngleEditor(cameraAngleEditorState);
  }

  if (isSubtitleClip(selectedClip)) {
    const subtitleEditorState: SubtitleEditorState = Object.assign(
      state.subtitleEditorWithoutSubtitleState,
      {
        subtitle: selectedClip.subtitle,
      },
    );
    return renderSubtitleEditor(
      {
        subtitleEditor: subtitleEditorState,
        timeline: state.timelineState,
      },
      {},
    );
  }

  return;
};
