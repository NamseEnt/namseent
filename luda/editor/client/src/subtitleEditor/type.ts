import { Selection, XywhRect } from "namui";

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

export type Subtitle = {
  // TODO
  // id: string;
  // startMs: number;
  // endMs: number;
  text: string;
  style: SubtitleStyle;
  placement: SubtitlePlacement;
};

export type SubtitleStyle = {
  fontSize: number;
  fontColor: Float32Array;
  backgroundColor: Float32Array;
};

export type SubtitlePlacement = {
  margin: SubtitleMargin;
};

export type SubtitleMargin = {
  vertical: number;
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
