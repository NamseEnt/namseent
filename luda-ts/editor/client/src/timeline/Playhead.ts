import {
  Clip,
  ColorUtil,
  PathDrawCommand,
  Rect,
  Render,
  RenderingData,
  WhSize,
} from "namui";

export const Playhead: Render<
  {
    playbackTimeMs: number;
  },
  {
    trackBodyWhSize: WhSize;
    pixelPerMs: number;
    startMs: number;
    timeRulerHeight: number;
    changePlaybackTimeMs: (playbackTimeMs: number) => void;
  }
> = (state, props) => {
  const centerX = (state.playbackTimeMs - props.startMs) * props.pixelPerMs;
  return Clip(
    {
      path: new CanvasKit.Path().addRect(
        CanvasKit.XYWHRect(
          0,
          0,
          props.trackBodyWhSize.width,
          props.trackBodyWhSize.height,
        ),
      ),
      clipOp: CanvasKit.ClipOp.Intersect,
    },
    [
      Rect({
        x: 0,
        y: 0,
        width: props.trackBodyWhSize.width,
        height: props.timeRulerHeight,
        style: {
          fill: {
            color: ColorUtil.Transparent,
          },
        },
        onMouseDown(mouseEvent) {
          if (!mouseEvent.isLeftButtonDown) {
            return;
          }
          const mouseMs =
            mouseEvent.translated.x / props.pixelPerMs + props.startMs;
          props.changePlaybackTimeMs(mouseMs);
        },
        onMouseMoveIn(mouseEvent) {
          if (!mouseEvent.isLeftButtonDown) {
            return;
          }
          const mouseMs =
            mouseEvent.translated.x / props.pixelPerMs + props.startMs;
          props.changePlaybackTimeMs(mouseMs);
        },
      }),
      TopHead(
        {},
        {
          centerX,
          height: props.timeRulerHeight,
          changePlaybackTimeMs: props.changePlaybackTimeMs,
        },
      ),
      Body(
        {},
        {
          centerX,
          y: props.timeRulerHeight,
          height: props.trackBodyWhSize.height,
        },
      ),
    ],
  );
};

const TopHead: Render<
  {},
  {
    centerX: number;
    height: number;
    changePlaybackTimeMs: (playbackTimeMs: number) => void;
  }
> = (state, props) => {
  const width = props.height;
  const path = new CanvasKit.Path();
  path.moveTo(props.centerX - width / 2, 0);
  path.lineTo(props.centerX + width / 2, 0);
  path.lineTo(props.centerX, props.height);
  path.close();

  const paint = new CanvasKit.Paint();
  paint.setColor(ColorUtil.Red);
  paint.setStyle(CanvasKit.PaintStyle.Fill);
  paint.setAntiAlias(true);

  return [
    RenderingData({
      drawCalls: [
        {
          commands: [
            PathDrawCommand({
              path,
              paint,
            }),
          ],
        },
      ],
    }),
  ];
};

const Body: Render<
  {},
  {
    centerX: number;
    y: number;
    height: number;
  }
> = (state, props) => {
  const path = new CanvasKit.Path();
  path.moveTo(props.centerX, props.y);
  path.lineTo(props.centerX, props.y + props.height);

  const paint = new CanvasKit.Paint();
  paint.setColor(ColorUtil.Red);
  paint.setStyle(CanvasKit.PaintStyle.Stroke);
  paint.setStrokeWidth(1);
  paint.setAntiAlias(true);

  return [
    RenderingData({
      drawCalls: [
        {
          commands: [
            PathDrawCommand({
              path,
              paint,
            }),
          ],
        },
      ],
    }),
  ];
};
