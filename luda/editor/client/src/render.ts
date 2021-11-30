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
import { TimelineSequenceNullableState, TimelineState } from "./timeline/type";
import { saver } from "./saver/saver";
import { renderSequenceListView } from "./sequenceListView/renderSequenceListView";
import { SequenceListViewState } from "./sequenceListView/type";
import { renderTopBar } from "./topBar/renderTopBar";
import { TopBarState } from "./topBar/type";

type State = {
  timelineState: TimelineSequenceNullableState;
  cameraAngleEditorWithoutCameraAngleState: CameraAngleEditorWithoutCameraAngleState;
  subtitleEditorWithoutSubtitleState: SubtitleEditorWithoutSubtitleState;
  livePlayer: {
    state: LivePlayerState;
    layout: LivePlayerProps["layout"];
  };
  sequenceListViewState: SequenceListViewState;
  topBarState: TopBarState;
};

export function render(state: State): RenderingTree {
  const { title, tracks } = state.timelineState;
  const doesTimelineHasSequence = title && tracks;

  if (!doesTimelineHasSequence) {
    return renderSequenceListView(
      {
        sequenceListView: state.sequenceListViewState,
        timeline: state.timelineState,
      },
      {},
    );
  }

  const timelineState: TimelineState = state.timelineState as TimelineState;

  const autoSaveState = saver.autoSave(
    `/sequence/${title}.json`,
    state.timelineState.tracks,
  );

  return [
    ClipEditor({
      cameraAngleEditorWithoutCameraAngleState:
        state.cameraAngleEditorWithoutCameraAngleState,
      subtitleEditorWithoutSubtitleState:
        state.subtitleEditorWithoutSubtitleState,
      timelineState,
    }),
    Timeline(timelineState, {
      changePlaybackTimeMs(playbackTimeMs) {
        changeLivePlayerPlaybackTime(state.livePlayer.state, playbackTimeMs);
      },
      playbackTimeMs: getPlaybackTimeMs(state.livePlayer.state),
    }),
    LivePlayer(state.livePlayer.state, {
      layout: state.livePlayer.layout,
      tracks,
    }),
    renderTopBar(
      {
        timelineState,
        topBarState: state.topBarState,
        sequenceListViewState: state.sequenceListViewState,
      },
      {
        autoSave: autoSaveState,
      },
    ),
  ];
}

const ClipEditor: Render<{
  timelineState: TimelineState;
  subtitleEditorWithoutSubtitleState: SubtitleEditorWithoutSubtitleState;
  cameraAngleEditorWithoutCameraAngleState: CameraAngleEditorWithoutCameraAngleState;
}> = ({
  timelineState,
  subtitleEditorWithoutSubtitleState,
  cameraAngleEditorWithoutCameraAngleState,
}) => {
  const { selectedClip } = timelineState;
  if (!selectedClip) {
    return;
  }

  if (isCameraClip(selectedClip)) {
    const cameraAngleEditorState: CameraAngleEditorState = Object.assign(
      cameraAngleEditorWithoutCameraAngleState,
      {
        cameraAngle: selectedClip.cameraAngle,
      },
    );
    return renderCameraAngleEditor(cameraAngleEditorState);
  }

  if (isSubtitleClip(selectedClip)) {
    const subtitleEditorState: SubtitleEditorState = Object.assign(
      subtitleEditorWithoutSubtitleState,
      {
        subtitle: selectedClip.subtitle,
      },
    );
    return renderSubtitleEditor(
      {
        subtitleEditor: subtitleEditorState,
        timeline: timelineState,
      },
      {},
    );
  }

  return;
};
