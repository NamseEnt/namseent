import {
  BorderPosition,
  ColorUtil,
  FontWeight,
  Language,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
  XywhRect,
} from "namui";

export const PlaybackTimeView: Render<
  {},
  {
    playbackTimeMs: number;
    layout: XywhRect;
  }
> = (state, props) => {
  const minutes = Math.floor(props.playbackTimeMs / 60000);
  const seconds = Math.floor((props.playbackTimeMs % 60000) / 1000);
  const milliseconds = Math.floor(props.playbackTimeMs % 1000);

  const MM = minutes.toString().padStart(2, "0");
  const ss = seconds.toString().padStart(2, "0");
  const mmm = milliseconds.toString().padStart(3, "0");
  return [
    Rect({
      ...props.layout,
      style: {
        stroke: {
          borderPosition: BorderPosition.inside,
          color: ColorUtil.Black,
          width: 1,
        },
      },
    }),
    Text({
      x: props.layout.x + props.layout.width / 2,
      y: props.layout.y + props.layout.height / 2,
      align: TextAlign.center,
      baseline: TextBaseline.middle,
      fontType: {
        fontWeight: FontWeight.regular,
        language: Language.ko,
        serif: false,
        size: 20,
      },
      style: {
        color: ColorUtil.Black,
      },
      text: `${MM}:${ss}:${mmm}`,
    }),
  ];
};
