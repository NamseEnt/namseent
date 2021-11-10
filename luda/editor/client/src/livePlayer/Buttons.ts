import {
  BorderPosition,
  ColorUtil,
  Path,
  PathDrawCommand,
  Rect,
  Render,
  Vector,
  XywhRect,
} from "namui";
import { pauseLivePlayer } from "./operations/pauseLivePlayer";
import { playLivePlayer } from "./operations/playLivePlayer";
import { LivePlayerState } from "./type";

export const Buttons: Render<
  LivePlayerState,
  {
    layout: XywhRect;
  }
> = (state, props) => {
  const playPauseToggleButton = state.isPlaying
    ? get1x1PauseButton()
    : get1x1PlayButton();

  const outerMargin = 0.05 * props.layout.height;
  const innerMargin = 0.05 * props.layout.height;
  const buttonSize = props.layout.height - (2 * outerMargin + 2 * innerMargin);

  const buttonCenterVector = new Vector(
    props.layout.x + props.layout.width / 2,
    props.layout.y + props.layout.height / 2,
  );

  playPauseToggleButton
    .transform(CanvasKit.Matrix.scaled(buttonSize, buttonSize))
    .transform(
      CanvasKit.Matrix.translated(
        buttonCenterVector.x - buttonSize / 2,
        buttonCenterVector.y - buttonSize / 2,
      ),
    );

  const buttonPaint = new CanvasKit.Paint();
  buttonPaint.setColor(ColorUtil.Black);
  buttonPaint.setStyle(CanvasKit.PaintStyle.Fill);
  buttonPaint.setStrokeWidth(1);
  buttonPaint.setAntiAlias(true);

  return [
    Rect({
      x: buttonCenterVector.x - buttonSize / 2 - innerMargin,
      y: buttonCenterVector.y - buttonSize / 2 - innerMargin,
      width: buttonSize + 2 * innerMargin,
      height: buttonSize + 2 * innerMargin,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
          borderPosition: BorderPosition.middle,
        },
      },
      onClick: () => {
        if (state.isPlaying) {
          pauseLivePlayer(state);
        } else {
          playLivePlayer(state);
        }
      },
    }),
    {
      drawCalls: [
        {
          commands: [
            PathDrawCommand({
              path: playPauseToggleButton,
              paint: buttonPaint,
            }),
          ],
        },
      ],
    },
  ];
};

function get1x1PlayButton(): Path {
  const path = new CanvasKit.Path();
  path.moveTo(0, 0);
  path.lineTo(1, 0.5);
  path.lineTo(0, 1);
  path.lineTo(0, 0);
  path.close();

  return path;
}

function get1x1PauseButton(): Path {
  const path = new CanvasKit.Path();
  path.addRect(CanvasKit.XYWHRect(0, 0, 0.4, 1));
  path.addRect(CanvasKit.XYWHRect(0.6, 0, 0.4, 1));

  return path;
}
