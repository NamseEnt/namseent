// 기본 타입들
export type Px = number;
export type Angle = number;
export type Percent = number;

export interface Xy<T> {
  x: T;
  y: T;
}

export interface Wh<T> {
  width: T;
  height: T;
}

export interface Rect<T> {
  left: T;
  top: T;
  right: T;
  bottom: T;
}

export interface TransformMatrix {
  values: [[number, number, number], [number, number, number]];
}

// Color
export interface Color {
  r: number;
  g: number;
  b: number;
  a: number;
}

// Paint
export interface Paint {
  // bincode로 deserialize될 실제 구조에 맞게 정의 필요
  // 일단 기본 구조만
  [key: string]: any;
}

// Path
export interface Path {
  // bincode로 deserialize될 실제 구조에 맞게 정의 필요
  [key: string]: any;
}

// Image
export interface Image {
  // bincode로 deserialize될 실제 구조에 맞게 정의 필요
  [key: string]: any;
}

// Font
export interface Font {
  size: number;
  name: string;
  // 기타 필드들...
  [key: string]: any;
}

// Enums
export enum TextAlign {
  Left = 0,
  Center = 1,
  Right = 2,
}

export enum TextBaseline {
  Top = 0,
  Middle = 1,
  Bottom = 2,
  Alphabetic = 3,
}

export enum ImageFit {
  None = 0,
  Contain = 1,
  Cover = 2,
  Fill = 3,
}

export enum ClipOp {
  Difference = 0,
  Intersect = 1,
}

// DrawCommand 타입들
export interface PathDrawCommand {
  path: Path;
  paint: Paint;
}

export interface TextDrawCommand {
  text: string;
  font: Font;
  x: Px;
  y: Px;
  paint: Paint;
  align: TextAlign;
  baseline: TextBaseline;
  max_width: Px | null;
  line_height_percent: Percent;
  underline: Paint | null;
}

export interface ImageDrawCommand {
  rect: Rect<Px>;
  image: Image;
  fit: ImageFit;
  paint: Paint | null;
}

export type DrawCommand =
  | { type: "Path"; command: PathDrawCommand }
  | { type: "Text"; command: TextDrawCommand }
  | { type: "Image"; command: ImageDrawCommand };

// Special Rendering Nodes
export interface TranslateNode {
  x: Px;
  y: Px;
  rendering_tree: RenderingTree;
}

export interface ClipNode {
  path: Path;
  clip_op: ClipOp;
  rendering_tree: RenderingTree;
}

export interface WithIdNode {
  id: string;
  rendering_tree: RenderingTree;
}

export interface AbsoluteNode {
  x: Px;
  y: Px;
  rendering_tree: RenderingTree;
}

export interface RotateNode {
  angle: Angle;
  rendering_tree: RenderingTree;
}

export interface ScaleNode {
  x: number;
  y: number;
  rendering_tree: RenderingTree;
}

export interface TransformNode {
  matrix: TransformMatrix;
  rendering_tree: RenderingTree;
}

export interface OnTopNode {
  rendering_tree: RenderingTree;
}

export interface MouseCursorNode {
  cursor: any; // MouseCursor 타입 정의 필요
  rendering_tree: RenderingTree;
}

export type SpecialRenderingNode =
  | { type: "Translate"; node: TranslateNode }
  | { type: "Clip"; node: ClipNode }
  | { type: "WithId"; node: WithIdNode }
  | { type: "Absolute"; node: AbsoluteNode }
  | { type: "Rotate"; node: RotateNode }
  | { type: "Scale"; node: ScaleNode }
  | { type: "Transform"; node: TransformNode }
  | { type: "OnTop"; node: OnTopNode }
  | { type: "MouseCursor"; node: MouseCursorNode };

// RenderingTree
export type RenderingTree =
  | { type: "Empty" }
  | { type: "Node"; command: DrawCommand }
  | { type: "Children"; children: RenderingTree[] }
  | { type: "Special"; special: SpecialRenderingNode }
  | { type: "Boxed"; boxed: RenderingTree }
  | { type: "BoxedChildren"; children: RenderingTree[] };
