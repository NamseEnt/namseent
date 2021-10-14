import {
  AfterDraw,
  Clip,
  ColorUtil,
  engine,
  Key,
  Rect,
  RenderingTree,
  Translate,
} from "namui";
import { Track, TimelineState } from "./type";
import { renderTrackBody } from "./renderTrackBody";

export function renderTimelineBody(
  props: {
    width: number;
    height: number;
    tracks: Track[];
  },
  state: TimelineState,
): RenderingTree {
  const trackBodyHeight = props.height / props.tracks.length;
  const trackBodies = props.tracks.map((track, index) => {
    const x = 0;
    const y = trackBodyHeight * index;
    const width = props.width;
    const height = trackBodyHeight;
    return Translate(
      { x, y },
      renderTrackBody({ width, height, track }, state),
    );
  });

  const settingZoom = AfterDraw(({ translated }) => {
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
    settingZoom,
  ];
}
