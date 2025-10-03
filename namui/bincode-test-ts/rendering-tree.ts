import {
  Struct,
  Enum,
  u8,
  u32,
  i32,
  f32,
  Option,
  Vec,
  bool as Bool,
  _ as Variant,
} from "bincode-ts";

// OrderedFloat는 본질적으로 f32
const OrderedFloat = f32;

// Px는 OrderedFloat wrapper
const Px = OrderedFloat;

// IntPx는 i32 wrapper
const IntPx = i32;

// Percent
const Percent = OrderedFloat;

// Color
const Color = Struct({
  r: u8,
  g: u8,
  b: u8,
  a: u8,
});

// Xy<T>
function XyOf<T>(inner: T) {
  return Struct({
    x: inner,
    y: inner,
  });
}
const XyPx = XyOf(Px);
const XyOrderedFloat = XyOf(OrderedFloat);

// Angle
const Angle = Struct({
  radians: OrderedFloat,
});

// TransformMatrix
const TransformMatrix = Struct({
  values: [
    [OrderedFloat, OrderedFloat, OrderedFloat],
    [OrderedFloat, OrderedFloat, OrderedFloat],
  ],
});

// Rect<T>
function RectOf<T>(inner: T) {
  return Enum({
    Xywh: Struct({
      x: inner,
      y: inner,
      width: inner,
      height: inner,
    }),
    Ltrb: Struct({
      left: inner,
      top: inner,
      right: inner,
      bottom: inner,
    }),
  });
}
const RectPx = RectOf(Px);

// Enums for Paint
const PaintStyle = Enum({
  Fill: Variant(0),
  Stroke: Variant(1),
});

const StrokeCap = Enum({
  Butt: Variant(0),
  Round: Variant(1),
  Square: Variant(2),
});

const StrokeJoin = Enum({
  Bevel: Variant(0),
  Miter: Variant(1),
  Round: Variant(2),
});

const BlendMode = Enum({
  Clear: Variant(0),
  Src: Variant(1),
  Dst: Variant(2),
  SrcOver: Variant(3),
  DstOver: Variant(4),
  SrcIn: Variant(5),
  DstIn: Variant(6),
  SrcOut: Variant(7),
  DstOut: Variant(8),
  SrcATop: Variant(9),
  DstATop: Variant(10),
  Xor: Variant(11),
  Plus: Variant(12),
  Modulate: Variant(13),
  Screen: Variant(14),
  Overlay: Variant(15),
  Darken: Variant(16),
  Lighten: Variant(17),
  ColorDodge: Variant(18),
  ColorBurn: Variant(19),
  HardLight: Variant(20),
  SoftLight: Variant(21),
  Difference: Variant(22),
  Exclusion: Variant(23),
  Multiply: Variant(24),
  Hue: Variant(25),
  Saturation: Variant(26),
  Color: Variant(27),
  Luminosity: Variant(28),
});

const TileMode = Enum({
  Clamp: Variant(0),
  Decal: Variant(1),
  Mirror: Variant(2),
  Repeat: Variant(3),
});

const FilterMode = Enum({
  Linear: Variant(0),
  Nearest: Variant(1),
});

const MipmapMode = Enum({
  None: Variant(0),
  Nearest: Variant(1),
  Linear: Variant(2),
});

const ColorType = Enum({
  Alpha8: Variant(0),
  Rgb565: Variant(1),
  Rgba8888: Variant(2),
  Bgra8888: Variant(3),
  Rgba1010102: Variant(4),
  Rgb101010x: Variant(5),
  Gray8: Variant(6),
  RgbaF16: Variant(7),
  RgbaF32: Variant(8),
});

const AlphaType = Enum({
  Opaque: Variant(0),
  Premul: Variant(1),
  Unpremul: Variant(2),
});

const ColorSpace = Enum({
  Srgb: Variant(0),
  DisplayP3: Variant(1),
  AdobeRgb: Variant(2),
});

// ColorFilter - simplified placeholder
// For now, we'll use a placeholder since we don't know the exact structure
let ColorFilter: any;
let Shader: any;
let MaskFilter: any;
let ImageFilter: any;

// Paint
const Paint = Struct({
  color: Color,
  paint_style: Option(PaintStyle),
  anti_alias: Option(Bool),
  stroke_width: Px,
  stroke_cap: Option(StrokeCap),
  stroke_join: Option(StrokeJoin),
  stroke_miter: Px,
  color_filter: Option(() => ColorFilter),
  blend_mode: Option(BlendMode),
  shader: Option(() => Shader),
  mask_filter: Option(() => MaskFilter),
  image_filter: Option(() => ImageFilter),
});

// StrokeOptions
const StrokeOptions = Struct({
  width: Option(Px),
  miter_limit: Option(Px),
  precision: Option(OrderedFloat),
  join: Option(StrokeJoin),
  cap: Option(StrokeCap),
});

// PathCommand
const PathCommand = Enum({
  AddRect: Variant(0, Struct({ rect: RectPx })),
  AddRrect: Variant(1, Struct({ rect: RectPx, rx: Px, ry: Px })),
  Stroke: Variant(2, Struct({ stroke_options: StrokeOptions })),
  MoveTo: Variant(3, Struct({ xy: XyPx })),
  LineTo: Variant(4, Struct({ xy: XyPx })),
  CubicTo: Variant(
    5,
    Struct({
      first_xy: XyPx,
      second_xy: XyPx,
      end_xy: XyPx,
    })
  ),
  ArcTo: Variant(
    6,
    Struct({
      oval: RectPx,
      start_angle: Angle,
      delta_angle: Angle,
    })
  ),
  Scale: Variant(7, Struct({ xy: XyOrderedFloat })),
  Translate: Variant(8, Struct({ xy: XyPx })),
  Transform: Variant(9, Struct({ matrix: TransformMatrix })),
  AddOval: Variant(10, Struct({ rect: RectPx })),
  AddArc: Variant(
    11,
    Struct({
      oval: RectPx,
      start_angle: Angle,
      delta_angle: Angle,
    })
  ),
  AddPoly: Variant(
    12,
    Struct({
      xys: Vec(XyPx),
      close: Bool,
    })
  ),
  Close: Variant(13),
});

// Path
const Path = Struct({
  commands: Vec(PathCommand),
});

// PathDrawCommand
const PathDrawCommand = Struct({
  path: Path,
  paint: Paint,
});

// Font
const Font = Struct({
  size: IntPx,
  name: String,
});

// TextAlign
const TextAlign = Enum({
  Left: Variant(0),
  Center: Variant(1),
  Right: Variant(2),
});

// TextBaseline
const TextBaseline = Enum({
  Top: Variant(0),
  Middle: Variant(1),
  Bottom: Variant(2),
});

// TextDrawCommand - simplified
const TextDrawCommand = Struct({
  text: String,
  font: Font,
  x: Px,
  y: Px,
  paint: Paint,
  align: TextAlign,
  baseline: TextBaseline,
  max_width: Option(Px),
  line_height_percent: Percent,
  underline: Option(Paint),
});

// ImageFit
const ImageFit = Enum({
  Fill: Variant(0),
  Contain: Variant(1),
  Cover: Variant(2),
  ScaleDown: Variant(3),
  None: Variant(4),
});

// Image - simplified placeholder
const Image: any = Struct({
  // We'll add fields as needed
});

// ImageDrawCommand - simplified
const ImageDrawCommand = Struct({
  rect: RectPx,
  image: Image,
  fit: ImageFit,
  paint: Option(Paint),
});

// DrawCommand
const DrawCommand = Enum({
  Path: Variant(0, Struct({ command: PathDrawCommand })),
  Text: Variant(1, Struct({ command: TextDrawCommand })),
  Image: Variant(2, Struct({ command: ImageDrawCommand })),
});

// Forward declaration for recursive type
let RenderingTree: any;
let SpecialRenderingNode: any;

// SpecialRenderingNode - simplified placeholder
SpecialRenderingNode = Enum({
  Translate: Variant(
    0,
    Struct({ rendering_tree: () => RenderingTree, xy: XyPx })
  ),
  Clip: Variant(1, Struct({ rendering_tree: () => RenderingTree })),
  Absolute: Variant(2, Struct({ rendering_tree: () => RenderingTree })),
  Rotate: Variant(3, Struct({ rendering_tree: () => RenderingTree })),
  Scale: Variant(4, Struct({ rendering_tree: () => RenderingTree })),
  Transform: Variant(5, Struct({ rendering_tree: () => RenderingTree })),
  OnTop: Variant(6, Struct({ rendering_tree: () => RenderingTree })),
  MouseCursor: Variant(7, Struct({ rendering_tree: () => RenderingTree })),
});

// RenderingTree
RenderingTree = Enum({
  Empty: Variant(0),
  Node: Variant(1, DrawCommand),
  Children: Variant(2, Vec(() => RenderingTree)),
  Special: Variant(3, SpecialRenderingNode),
});

export { RenderingTree };
