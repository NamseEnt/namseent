import {
  Clip,
  ColorUtil,
  Rect,
  RenderingTree,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
  XywhRect,
} from "namui";
import { Subtitle } from "../type";

export function renderPreview(props: {
  layout: {
    rect: XywhRect;
  };
  videoSize: {
    width: number;
    height: number;
  };
  subtitle: Subtitle;
}): RenderingTree {
  const videoAspect = props.videoSize.width / props.videoSize.height;
  const layoutAspect = props.layout.rect.width / props.layout.rect.height;
  const reductionRatio =
    videoAspect > layoutAspect
      ? props.layout.rect.width / props.videoSize.width
      : props.layout.rect.height / props.videoSize.height;

  const previewWidth = props.videoSize.width * reductionRatio;
  const previewHeight = props.videoSize.height * reductionRatio;
  const previewX = (props.layout.rect.width - previewWidth) / 2;
  const previewY = (props.layout.rect.height - previewHeight) / 2;

  return Translate(
    props.layout.rect,
    Clip(
      {
        path: new CanvasKit.Path().addRect(
          CanvasKit.XYWHRect(
            0,
            0,
            props.layout.rect.width,
            props.layout.rect.height,
          ),
        ),
        clipOp: CanvasKit.ClipOp.Intersect,
      },
      [
        Rect({
          ...props.layout.rect,
          x: 0,
          y: 0,
          style: {
            fill: {
              color: ColorUtil.Black,
            },
          },
        }),
        Rect({
          width: previewWidth,
          height: previewHeight,
          x: previewX,
          y: previewY,
          style: {
            fill: {
              color: ColorUtil.White,
            },
          },
        }),
        Text({
          x: previewX + previewWidth / 2,
          y: previewY + previewHeight - 48 * reductionRatio,
          align: TextAlign.center,
          baseline: TextBaseline.bottom,
          fontType: {
            ...props.subtitle.fontType,
            size: props.subtitle.fontType.size * reductionRatio,
          },
          style: {
            ...props.subtitle.style,
            border: {
              ...props.subtitle.style.border,
              width: props.subtitle.style.border.width * reductionRatio,
            },
            dropShadow: {
              ...props.subtitle.style.dropShadow,
              x: props.subtitle.style.dropShadow.x * reductionRatio,
              y: props.subtitle.style.dropShadow.y * reductionRatio,
            },
          },
          text: props.subtitle.text,
        }),
      ],
    ),
  );
}
