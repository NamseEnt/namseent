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
  InputRect,
  Color,
} from "canvaskit-wasm";
import { BuildErrorNotifier } from "./build/BuildErrorNotifier";
import { BuildServerConnection } from "./build/BuildServerConnection";
import { FontStorage } from "./font/FontStorage";
import { TypefaceStorage } from "./font/TypefaceStorage";
import { DeepReadonly } from "./types/DeepReadonly";
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
  interface Math {
    clamp(value: number, min: number, max: number): number;
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
  paint?: Paint;
};

export function ImageDrawCommand(
  command: Omit<ImageDrawCommand, "type">,
): ImageDrawCommand {
  return {
    ...command,
    type: "image",
  };
}

export type PathDrawCommand = { type: "path"; path: Path; paint: Paint };
export function PathDrawCommand(
  command: Omit<PathDrawCommand, "type">,
): PathDrawCommand {
  return {
    ...command,
    type: "path",
  };
}

export type DrawCommand = PathDrawCommand | ImageDrawCommand | TextDrawCommand;

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
  id?: string;
  onClick?: MouseEventCallback;
  onClickOut?: MouseEventCallback;
  onMouseMoveIn?: MouseEventCallback;
  onMouseMoveOut?: MouseEventCallback;
  onMouseIn?: () => void;
  onMouseDown?: MouseEventCallback;
  onMouseUp?: MouseEventCallback;
};
export function RenderingData(renderingData: RenderingData): RenderingData {
  return renderingData;
}

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

export type Render<TState extends {}, TProps = void> = TProps extends void
  ? (state: TState) => RenderingTree
  : (state: TState, props: DeepReadonly<TProps>) => RenderingTree;

export type RenderExact<TState extends {}, TProps = void> = TProps extends void
  ? (state: TState) => RenderingData
  : (state: TState, props: DeepReadonly<TProps>) => RenderingData;

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
  fontStorage: FontStorage;
};

export type RenderingElement<TArgs> = (args: TArgs) => RenderingTree;

export class Vector {
  constructor(public readonly x: number, public readonly y: number) {}
  public static from({ x, y }: { x: number; y: number }): Vector {
    return new Vector(x, y);
  }
  translate(dx: number, dy: number): Vector {
    return new Vector(this.x + dx, this.y + dy);
  }
  sub(other: Vector): Vector {
    return new Vector(this.x - other.x, this.y - other.y);
  }
  add(other: Vector): Vector {
    return new Vector(this.x + other.x, this.y + other.y);
  }
  cross(other: Vector): number {
    return this.x * other.y - this.y * other.x;
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
  grab = "grab",
  move = "move",
}

// this is static class
export const Convert = {
  ltrbToXywh(rect: LtrbRect): XywhRect {
    return {
      x: rect.left,
      y: rect.top,
      width: rect.right - rect.left,
      height: rect.bottom - rect.top,
    };
  },
  xywhToLtrb(rect: XywhRect): LtrbRect {
    return {
      left: rect.x,
      top: rect.y,
      right: rect.x + rect.width,
      bottom: rect.y + rect.height,
    };
  },
  xywhToCanvasKit(rect: XywhRect): InputRect {
    return CanvasKit.XYWHRect(rect.x, rect.y, rect.width, rect.height);
  },
  ColorToHsl(color: Color) {
    const [r, g, b, a] = color;
    const normalizedR = r || 0;
    const normalizedG = g || 0;
    const normalizedB = b || 0;
    const max = Math.max(normalizedR, normalizedG, normalizedB);
    const min = Math.min(normalizedR, normalizedG, normalizedB);
    const delta = max - min;

    let hue = 0;
    if (delta !== 0) {
      switch (max) {
        case normalizedR: {
          hue = (normalizedG - normalizedB) / delta;
          break;
        }
        case normalizedG: {
          hue = (normalizedB - normalizedR) / delta + 2;
          break;
        }
        case normalizedB: {
          hue = (normalizedR - normalizedG) / delta + 4;
          break;
        }
        default: {
          throw new Error("Can not calculate hue.");
        }
      }
    }
    hue = (hue < 0 ? hue + 6 : hue) / 6;

    const lightness = (max + min) / 2;
    const saturation =
      delta === 0 ? 0 : delta / (1 - Math.abs(2 * lightness - 1));

    return {
      hue,
      saturation,
      lightness,
      alpha: a || 0,
    };
  },
};

export type WhSize = {
  width: number;
  height: number;
};

export enum BorderPosition {
  inside = "inside",
  outside = "outside",
  middle = "middle",
}
