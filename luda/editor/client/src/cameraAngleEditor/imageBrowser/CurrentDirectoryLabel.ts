import {
  Render,
  TextAlign,
  TextBaseline,
  Language,
  FontWeight,
  ColorUtil,
  Text,
} from "namui";
import { ImageBrowserState } from "./type";

export const CurrentDirectoryLabel: Render<ImageBrowserState, {}> = (
  state,
  props,
) => {
  return [
    Text({
      ...state.layout.currentDirectoryLabel,
      text: state.directoryKey,
      align: TextAlign.left,
      baseline: TextBaseline.top,
      fontType: {
        size: 16,
        serif: false,
        language: Language.ko,
        fontWeight: FontWeight.regular,
      },
      style: {
        color: ColorUtil.Black,
      },
    }),
  ];
};
