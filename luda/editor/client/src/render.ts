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
import { renderSequenceListView } from "./sequenceListView/renderSequenceListView";
import { SequenceListViewState } from "./sequenceListView/type";

type State = {
  timelineState: TimelineState;
  cameraAngleEditorWithoutCameraAngleState: CameraAngleEditorWithoutCameraAngleState;
  subtitleEditorWithoutSubtitleState: SubtitleEditorWithoutSubtitleState;
  livePlayer: {
    state: LivePlayerState;
    layout: LivePlayerProps["layout"];
  };
  sequenceListViewState: SequenceListViewState;
};

export function render(state: State): RenderingTree {
  const { editingSequenceTitle } = state.sequenceListViewState;
  if (editingSequenceTitle) {
    saver.autoSave(
      `/sequence/${editingSequenceTitle}.json`,
      state.timelineState.tracks,
    );
  }

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
      tracks: state.timelineState.tracks,
    }),
    SequenceListView(state),
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

const SequenceListView: Render<State> = (state) => {
  const { selectedClip } = state.timelineState;
  if (selectedClip) {
    return;
  }

  return renderSequenceListView(
    {
      sequenceListView: state.sequenceListViewState,
      timeline: state.timelineState,
    },
    {},
  );
};
