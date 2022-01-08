use super::super::*;
use super::*;

#[wasm_bindgen]
extern "C" {
    pub type EmbindEnumEntity;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &EmbindEnumEntity) -> f32;
}

macro_rules! canvas_kit_enum {
    (
        $enum_name:ident,
        $static_mod_name:ident,
        $canvas_enum_values_name:ident,
        $canvas_enum_name:ident,
        {
            $($enum_item:ident: $canvas_kit_enum_value_name:ident),* $(,)?
        }
    ) => {
        #[wasm_bindgen]
        extern "C" {
            pub type $canvas_enum_values_name;
            #[wasm_bindgen(extends = EmbindEnumEntity)]
            pub type $canvas_enum_name;

            #[wasm_bindgen(method, getter)]
            pub fn $enum_name(this: &CanvasKit) -> $canvas_enum_values_name;

            $(
                #[wasm_bindgen(method, getter)]
                pub fn $canvas_kit_enum_value_name(this: &$canvas_enum_values_name) -> $canvas_enum_name;
            )*
        }

        pub(crate) mod $static_mod_name {
            use once_cell::sync::Lazy;
            lazy_static::lazy_static! {
                $(
                    pub static ref $enum_item: Lazy<f32> =
                        Lazy::new(|| $crate::namui::skia::canvas_kit().$enum_name().$canvas_kit_enum_value_name().value());
                )*
            }
        }

        impl $enum_name {
            pub(crate) fn into_canvas_kit(&self) -> $canvas_enum_name {
                match self {
                    $(
                        $enum_name::$enum_item => canvas_kit().$enum_name().$canvas_kit_enum_value_name(),
                    )*
                }
            }
        }
    };
}

canvas_kit_enum!(
    PaintStyle,
    paint_style,
    CanvasKitPaintStyleEnumValues,
    CanvasKitPaintStyle,
    {
        Fill: Fill,
        Stroke: Stroke,
    }
);
canvas_kit_enum!(
    StrokeCap,
    stroke_cap,
    CanvasKitStrokeCapEnumValues,
    CanvasKitStrokeCap,
    {
        Butt: Butt,
        Round: Round,
        Square: Square,
    }
);
canvas_kit_enum!(
    StrokeJoin,
    stroke_join,
    CanvasKitStrokeJoinEnumValues,
    CanvasKitStrokeJoin,
    {
        Bevel: Bevel,
        Miter: Miter,
        Round: Round,
    }
);
canvas_kit_enum!(
    ClipOp,
    clip_op,
    CanvasKitClipOpEnumValues,
    CanvasKitClipOp,
    {
        Difference: Difference,
        Intersect: Intersect,
    }
);
canvas_kit_enum!(
    AlphaType,
    alpha_type,
    CanvasKitAlphaTypeEnumValues,
    CanvasKitAlphaType,
    {
        Opaque: Opaque,
        Premul: Premul,
        Unpremul: Unpremul,
    }
);
canvas_kit_enum!(
    ColorType,
    color_type,
    CanvasKitColorTypeEnumValues,
    CanvasKitColorType,
    {
        Alpha8: Alpha_8,
        Rgb565: RGB_565,
        Rgba8888: RGBA_8888,
        Bgra8888: BGRA_8888,
        Rgba1010102: RGBA_1010102,
        Rgb101010x: RGB_101010x,
        Gray8: Gray_8,
        RgbaF16: RGBA_F16,
        RgbaF32: RGBA_F32,
    }
);
canvas_kit_enum!(
    FilterMode,
    filter_mode,
    CanvasKitFilterModeEnumValues,
    CanvasKitFilterMode,
    {
        Linear: Linear,
        Nearest: Nearest,
    }
);
canvas_kit_enum!(
    MipmapMode,
    mipmap_mode,
    CanvasKitMipmapModeEnumValues,
    CanvasKitMipmapMode,
    {
        None: None,
        Nearest: Nearest,
        Linear: Linear,
    }
);
canvas_kit_enum!(
    BlendMode,
    blend_mode,
    CanvasKitBlendModeEnumValues,
    CanvasKitBlendMode,
    {
        Clear: Clear,
        Src: Src,
        Dst: Dst,
        SrcOver: SrcOver,
        DstOver: DstOver,
        SrcIn: SrcIn,
        DstIn: DstIn,
        SrcOut: SrcOut,
        DstOut: DstOut,
        SrcATop: SrcATop,
        DstATop: DstATop,
        Xor: Xor,
        Plus: Plus,
        Modulate: Modulate,
        Screen: Screen,
        Overlay: Overlay,
        Darken: Darken,
        Lighten: Lighten,
        ColorDodge: ColorDodge,
        ColorBurn: ColorBurn,
        HardLight: HardLight,
        SoftLight: SoftLight,
        Difference: Difference,
        Exclusion: Exclusion,
        Multiply: Multiply,
        Hue: Hue,
        Saturation: Saturation,
        Color: Color,
        Luminosity: Luminosity,
    }
);
canvas_kit_enum!(
    ColorSpace,
    color_space,
    CanvasKitColorSpaceEnumValues,
    CanvasKitColorSpace,
    {
        Srgb: SRGB,
        DisplayP3: DISPLAY_P3,
        AdobeRgb: ADOBE_RGB,
    }
);
