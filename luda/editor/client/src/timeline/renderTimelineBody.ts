import {
  AfterDraw,
  Clip,
  ColorUtil,
  engine,
  Key,
  Rect,
  Render,
  RenderingTree,
  Translate,
  BorderPosition,
} from "namui";
import { Track, TimelineState, TrackType } from "./type";
import { DefaultTrackBody } from "./DefaultTrackBody";
import { CameraTrackBody } from "./cameraTrack/CameraTrackBody";
import { renderSubtitleTrackBody } from "./renderSubtitleTrackBody";

export const TimelineBody: Render<
  {
    timelineState: TimelineState;
  },
  {
    width: number;
    height: number;
    tracks: Track[];
  }
> = (state, props) => {
  const trackBodyHeight = 80;
  const trackBodies = props.tracks.map((track, index) => {
    const x = 0;
    const y = trackBodyHeight * index;
    const width = props.width;
    const height = trackBodyHeight;
    return Translate(
      { x, y },
      TrackBody(
        {
          timelineState: state.timelineState,
        },
        { width, height, track },
      ),
    );
  });

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
    }),
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(0, 0, props.width, props.height),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      trackBodies,
    ),
    setWheelZoomHandler(state.timelineState),
    setWheelMoveHandler(state.timelineState),
  ];
};

const TrackBody: Render<
  {
    timelineState: TimelineState;
  },
  {
    width: number;
    height: number;
    track: Track;
  }
> = (state, props) => {
  const { timelineState } = state;
  const { track } = props;
  switch (track.type) {
    case TrackType.camera:
      return CameraTrackBody(
        {
          timelineState,
        },
        {
          width: props.width,
          height: props.height,
          track,
        },
      );
    case TrackType.subtitle:
      return renderSubtitleTrackBody(
        {
          timelineState,
        },
        {
          width: props.width,
          height: props.height,
          track,
        },
      );
    default:
      return DefaultTrackBody(
        {
          timelineState,
        },
        {
          width: props.width,
          height: props.height,
          track,
        },
      );
  }
};

function setWheelMoveHandler(state: TimelineState): RenderingTree {
  engine.wheel.onWheel(({ deltaY }) => {
    if (!engine.keyboard.isKeyPress(Key.Shift)) {
      return;
    }

    const { msPerPixel, startMs } = state.layout;

    const nextStartMs = startMs + deltaY * msPerPixel;

    state.layout.startMs = nextStartMs;
  });
  return;
}

function setWheelZoomHandler(state: TimelineState): RenderingTree {
  return AfterDraw(({ translated }) => {
    const timelineBodyGlobalLeft = translated.x;
    engine.wheel.onWheel(({ deltaY }) => {
      if (!engine.keyboard.isKeyPress(Key.Alt)) {
        return;
      }

      const { msPerPixel, startMs } = state.layout;

      const mousePositionTimelineBodyDeltaX =
        engine.mousePosition.mousePosition.x - timelineBodyGlobalLeft;
      const msOfMousePosition =
        startMs + mousePositionTimelineBodyDeltaX * msPerPixel;

      function zoomByWheel(target: number, delta: number): number {
        const step = 400;
        const min = 10;
        const max = 1000;

        const wheel = step * Math.log2(target / 10);

        const nextWheel = wheel + delta;

        const zoomed = Math.max(
          Math.min(10 * Math.pow(2, nextWheel / step), max),
          min,
        );
        return zoomed;
      }

      const nextMsPerPixel = zoomByWheel(msPerPixel, deltaY);
      const nextStartMs =
        msOfMousePosition - mousePositionTimelineBodyDeltaX * nextMsPerPixel;

      state.layout.msPerPixel = nextMsPerPixel;
      state.layout.startMs = nextStartMs;
    });
  });
}
