import { Selection, XywhRect } from "namui";
import { Subtitle } from "../type";

export type SubtitleEditorState = {
  layout: {
    rect: XywhRect;
    videoSize: {
      width: number;
      height: number;
    };
  };
  subtitle: Subtitle;
  textInput: TextInputState;
  colorInput: ColorInputState;
};

export type TextInputState = {
  targetId?: string;
  selection?: Selection;
};

export type ColorInputState = {
  targetId?: string;
  hue: number;
  saturation: number;
  lightness: number;
  alpha: number;
};
