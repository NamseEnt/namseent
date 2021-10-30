import { Language, Selection, XywhRect } from "namui";
import { FontWeight } from "namui/lib/font/FontStorage";

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
  fontType: SubtitleFontType;
  style: SubtitleStyle;
};

export type SubtitleFontType = {
  serif: boolean;
  size: SubtitleFontSize;
  language: Language;
  fontWeight: FontWeight;
};

export type SubtitleStyle = {
  color: Float32Array;
  background: {
    color: Float32Array;
  };
  border: {
    color: Float32Array;
    width: number;
  };
  dropShadow: {
    x: number;
    y: number;
    color: Float32Array;
  };
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

export enum SubtitleFontSize {
  small = 24,
  regular = 48,
  large = 64,
}
