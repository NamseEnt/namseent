import { ColorUtil, MouseButton, Rect, Render, BorderPosition } from "namui";
import { Track, TimelineState } from "./type";
import { ClipComponent } from "./clip/ClipComponent";
import { Sash } from "./clip/Sash";
import { Clip } from "../type";
import { isSubtitleClip } from "../clipTypeGuard";
import { renderClip } from "./clip/renderClip";

export const DefaultTrackBody: Render<
  { timelineState: TimelineState; track: Track },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { clips } = state.track;
  let selectedClip: Clip | undefined = undefined;

  return [
    Rect({
      x: 0,
      y: 0,
      width: props.width,
      height: props.height,
      style: {
        fill: {
          color: ColorUtil.Color01(0.4, 0.4, 0.4),
        },
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
      onMouseUp(event) {
        if (event.button === MouseButton.right) {
          const clickMs =
            state.timelineState.layout.startMs +
            event.translated.x * state.timelineState.layout.msPerPixel;
          state.timelineState.contextMenu = {
            type: "trackBody",
            clickMs,
            x: event.x,
            y: event.y,
            trackId: state.track.id,
          };
        }
      },
    }),
    Rect({
      x: 0,
      y: props.height / 3,
      width: props.width,
      height: props.height / 3,
      style: {
        stroke: {
          color: ColorUtil.Color01(0.25, 0.25, 0.25),
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
    Rect({
      x: 0,
      y: 0,
      width: props.width,
      height: props.height,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.inside,
        },
      },
    }),
    state.timelineState.actionState?.type === "dragClip"
      ? renderClipWithPlacementConstraint({
          clips,
          selectedClipId: state.timelineState.selectedClip?.id,
          noEmit:
            state.timelineState.actionState.terminatePhase !== "terminated",
          clipComponentState: { timelineState: state.timelineState },
          clipComponentProps: {
            height: props.height,
            maxRight: props.width,
            sashComponent: Sash,
          },
        })
      : [
          clips.map((clip) => {
            return renderClip(
              { timelineState: state.timelineState, clip },
              {
                height: props.height,
                maxRight: props.width,
              },
            );
          }),
          selectedClip
            ? renderClip(
                { timelineState: state.timelineState, clip: selectedClip },
                {
                  height: props.height,
                  maxRight: props.width,
                },
              )
            : undefined,
        ],
  ];
};

function renderClipWithPlacementConstraint(props: {
  clips: Clip[];
  selectedClipId?: string;
  noEmit?: boolean;
  clipComponentState: Omit<Parameters<typeof ClipComponent>[0], "clip">;
  clipComponentProps: Parameters<typeof ClipComponent>[1];
}) {
  const {
    clips,
    selectedClipId,
    noEmit,
    clipComponentState,
    clipComponentProps,
  } = props;
  if (!noEmit) {
    console.log(123);
  }
  const sortedClips = clips.sort((a, b) => a.startMs - b.startMs);
  const selectedClipIndex = selectedClipId
    ? sortedClips.findIndex((clip) => clip.id === selectedClipId)
    : -1;

  let previousClipOffset = 0;
  let conflictResolved = false;
  return sortedClips.map((clip, index, clips) => {
    if (index < selectedClipIndex || conflictResolved) {
      return renderClip({ ...clipComponentState, clip }, clipComponentProps);
    }
    const clipOffset = calculateClipOffset(
      clips[index - 1],
      clip,
      noEmit ? previousClipOffset : 0,
    );
    previousClipOffset = clipOffset;

    if (!noEmit) {
      clip.startMs += clipOffset;
      clip.endMs += clipOffset;
    }

    return renderClip(
      {
        ...clipComponentState,
        clip: noEmit
          ? {
              ...clip,
              startMs: clip.startMs + clipOffset,
              endMs: clip.endMs + clipOffset,
            }
          : clip,
      },
      clipComponentProps,
    );
  });
}

function calculateClipOffset(
  previousClip: Clip | undefined,
  currentClip: Clip,
  previousClipOffset: number,
) {
  if (isSubtitleClip(currentClip)) {
    if (!previousClip) {
      return 0;
    }
    const clipOffset =
      previousClip.startMs + previousClipOffset + 200 - currentClip.startMs;
    return Math.max(clipOffset, 0);
  }

  return 0;
}
