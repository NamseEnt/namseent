import {
  Render,
  WhSize,
  XywhRect,
  ColorUtil,
  Rect,
  TextAlign,
  TextBaseline,
  FontWeight,
  Language,
  Text,
  Image,
  ImageFit,
} from "namui";
import { ImageBrowserState } from "./type";

export const BrowserItem: Render<
  ImageBrowserState,
  {
    itemSize: WhSize;
    thumbnailRect: XywhRect;
    thumbnailUrl: string;
    name: string;
    onSelect: () => void;
  }
> = (state, props) => {
  const { itemSize, thumbnailRect } = props;

  return [
    Rect({
      x: 0,
      y: 0,
      ...itemSize,
      style: {
        stroke: {
          color: ColorUtil.Black,
          width: 1,
        },
        round: {
          radius: 5,
        },
        fill: {
          color: ColorUtil.White,
        },
      },
      onClick: props.onSelect,
    }),
    Text({
      x: itemSize.width / 2,
      y: itemSize.height - 20,
      text: props.name,
      align: TextAlign.center,
      baseline: TextBaseline.top,
      fontType: {
        fontWeight: FontWeight.regular,
        language: Language.ko,
        serif: false,
        size: 16,
      },
      style: {
        color: ColorUtil.Black,
      },
    }),
    Image({
      position: thumbnailRect,
      size: thumbnailRect,
      url: props.thumbnailUrl,
      style: {
        fit: ImageFit.contain,
      },
    }),
  ];
};
