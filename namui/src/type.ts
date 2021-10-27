import {
  Path,
  ClipOp,
  Paint,
  Paragraph,
  ParagraphStyle,
  CanvasKit,
  Surface,
  Canvas,
  Font,
} from "canvaskit-wasm";
import { BuildErrorNotifier } from "./build/BuildErrorNotifier";
import { BuildServerConnection } from "./build/BuildServerConnection";
import { FontStorage } from "./font/FontStorage";
import { TypefaceStorage } from "./font/TypefaceStorage";
import { IImageLoader } from "./image/ImageLoader";
import { SpecialRenderingCommand } from "./types/SpecialRenderingCommand";

declare global {
  // It is GcCollectedCanvasKit
  var CanvasKit: CanvasKit;
  var typefaceStorage: TypefaceStorage;
  var fontStorage: FontStorage;
  var buildServerConnection: BuildServerConnection;
  var buildErrorNotifier: BuildErrorNotifier;
  interface Array<T> {
    remove(o: T): void;
  }
}

Array.prototype.remove = function (element) {
  const index = this.indexOf(element);
  if (index > -1) {
    this.splice(index, 1);
  }
};

export enum TextAlign {
  left = "left",
  right = "right",
  center = "center",
}
export enum TextBaseline {
  top = "top",
  bottom = "bottom",
  middle = "middle",
}

export type TextDrawCommand = {
  type: "text";
  text: string;
  font: Font;
  x: number;
  y: number;
  paint: Paint;
  align: TextAlign;
  baseline: TextBaseline;
};

export function TextDrawCommand(
  command: Omit<TextDrawCommand, "type">,
): TextDrawCommand {
  return {
    ...command,
    type: "text",
  };
}

export enum ImageFit {
  fill = "fill",
  contain = "contain",
  cover = "cover",
  scaleDown = "scaleDown",
  none = "none",
}

export type ImageDrawCommand = {
  type: "image";
  x: number;
  y: number;
  url: string;
  size: {
    width: number;
    height: number;
  };
  fit: ImageFit;
};

export function ImageDrawCommand(
  command: Omit<ImageDrawCommand, "type">,
): ImageDrawCommand {
  return {
    ...command,
    type: "image",
  };
}

export type DrawCommand =
  | { type: "path"; path: Path; paint: Paint }
  // | { paragraph: Paragraph; x: number; y: number }
  | ImageDrawCommand
  | TextDrawCommand;

export type DrawCall = {
  commands: DrawCommand[];
};

export enum MouseButton {
  left = 0,
  center = 1,
  right = 2,
}

export type MouseEvent = {
  x: number;
  y: number;
  translated: {
    x: number;
    y: number;
  };
  button: MouseButton;
  isLeftButtonDown: boolean;
  isRightButtonDown: boolean;
};

export type MouseEventExceptTranslated = Omit<MouseEvent, "translated">;

export type WheelEvent = {
  deltaX: number;
  deltaY: number;
};

export type MouseEventCallback = (event: MouseEvent) => void;
export type WheelEventCallback = (event: WheelEvent) => void;

export type RenderingData = {
  drawCalls: DrawCall[];
  onClick?: MouseEventCallback;
  onClickOut?: MouseEventCallback;
  onMouseMoveIn?: MouseEventCallback;
  onMouseMoveOut?: MouseEventCallback;
  onMouseDown?: MouseEventCallback;
  onMouseUp?: MouseEventCallback;
};

export type MakeParagraph = (
  style: ParagraphStyle,
  font: string,
  text: string,
) => Paragraph;

export type RenderingFunctionArgs = {
  canvasKit: CanvasKit;
  makeParagraph: MakeParagraph;
};

export type XywhRect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

export type LtrbRect = {
  left: number;
  top: number;
  right: number;
  bottom: number;
};

export type Rect = XywhRect | LtrbRect;

export type RenderingTree =
  | RenderingTree[]
  | RenderingData
  | SpecialRenderingCommand
  | undefined
  | false;

export type Render<TState> = (state: TState) => RenderingTree;

export type EngineContext<TState = any> = {
  render: Render<TState>;
  canvasKit: CanvasKit;
  deleteGarbages: () => void;
  surface: Surface;
  canvas: Canvas;
  state: TState;
  lastRenderedTree?: RenderingTree;
  fpsInfo: {
    fps: number;
    frameCount: number;
    last60FrameTimeMs: number;
  };
  isStopped: boolean;
  imageLoader: IImageLoader;
  fontStorage: FontStorage;
};

export type RenderingElement<TArgs> = (args: TArgs) => RenderingTree;

export class Vector {
  constructor(public readonly x: number, public readonly y: number) {}
  translate(dx: number, dy: number): Vector {
    return new Vector(this.x + dx, this.y + dy);
  }
  toString(): string {
    return `(${this.x}, ${this.y})`;
  }
}

export enum Cursor {
  topBottomResize = "topBottomResize",
  leftRightResize = "leftRightResize",
  leftTopRightBottomResize = "leftTopRightBottomResize",
  rightTopLeftBottomResize = "rightTopLeftBottomResize",
  default = "default",
  text = "text",
}
