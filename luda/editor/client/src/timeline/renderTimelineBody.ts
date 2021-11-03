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
} from "namui";
import { Track, TimelineState, TrackType } from "./type";
import { DefaultTrackBody } from "./DefaultTrackBody";
import { CameraTrackBody } from "./cameraTrack/CameraTrackBody";

export const TimelineBody: Render<
  {
    timelineState: TimelineState;
    tracks: Track[];
  },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const trackBodyHeight = props.height / state.tracks.length;
  const trackBodies = state.tracks.map((track, index) => {
    const x = 0;
    const y = trackBodyHeight * index;
    const width = props.width;
    const height = trackBodyHeight;
    return Translate(
      { x, y },
      TrackBody(
        {
          timelineState: state.timelineState,
          track,
        },
        { width, height },
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
    track: Track;
  },
  {
    width: number;
    height: number;
  }
> = (state, props) => {
  const { timelineState, track } = state;
  switch (track.type) {
    case TrackType.camera:
      return CameraTrackBody(
        {
          timelineState,
          track,
        },
        {
          width: props.width,
          height: props.height,
        },
      );
    default:
      return DefaultTrackBody(timelineState, {
        width: props.width,
        height: props.height,
        track,
      });
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
