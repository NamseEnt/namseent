import { Render, RenderingTree } from "namui";
import { renderCameraAngleEditor } from "./cameraAngleEditor/renderCameraAngleEditor";
import { CameraAngleEditorState } from "./cameraAngleEditor/type";
import { isCameraClip, isSubtitleClip } from "./clipTypeGuard";
import { ImageEditorState } from "./imageEditor/type";
import { renderSubtitleEditor } from "./subtitleEditor/renderSubtitleEditor";
import { LivePlayerProps, LivePlayer } from "./livePlayer/LivePlayer";
import { changeLivePlayerPlaybackTime } from "./livePlayer/operations/changeLivePlayerPlaybackTime";
import { getPlaybackTimeMs } from "./livePlayer/operations/getPlaybackTimeMs";
import { LivePlayerState } from "./livePlayer/type";
import { SubtitleEditorState } from "./subtitleEditor/type";
import { Timeline } from "./timeline/Timeline";
import { TimelineState } from "./timeline/type";

type State = {
  imageEditorState: ImageEditorState;
  timelineState: TimelineState;
  cameraAngleEditorState: CameraAngleEditorState;
  subtitleEditorState: SubtitleEditorState;
  livePlayer: {
    state: LivePlayerState;
    layout: LivePlayerProps["layout"];
  };
};

export function render(state: State): RenderingTree {
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
  ];
}

const ClipEditor: Render<State> = (state) => {
  const { selectedClip } = state.timelineState;
  if (!selectedClip) {
    return;
  }

  if (isCameraClip(selectedClip)) {
    state.cameraAngleEditorState.cameraAngle = selectedClip.cameraAngle;
    return renderCameraAngleEditor(state.cameraAngleEditorState);
  }

  if (isSubtitleClip(selectedClip)) {
    state.subtitleEditorState.subtitle = selectedClip.subtitle;
    return renderSubtitleEditor(
      {
        subtitleEditor: state.subtitleEditorState,
        timeline: state.timelineState,
      },
      {},
    );
  }

  return;
};
