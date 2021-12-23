use lazy_static::lazy_static;
use once_cell::sync::Lazy;

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
        $canvas_enum_values_name:ident,
        $canvas_enum_name:ident,
        {
            $($enum_item:ident: $static_enum_item:ident),* $(,)?
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
                pub fn $enum_item(this: &$canvas_enum_values_name) -> $canvas_enum_name;
            )*
        }


        lazy_static! {
            $(
                pub static ref $static_enum_item: Lazy<f32> =
                    Lazy::new(|| canvas_kit().$enum_name().$enum_item().value());
            )*
        }
    };
}

canvas_kit_enum!(
    PaintStyle,
    CanvasKitPaintStyleEnumValues,
    CanvasKitPaintStyle,
    {
        Fill: PAINT_STYLE_FILL_VALUE,
        Stroke: PAINT_STYLE_STROKE_VALUE,
    }
);
canvas_kit_enum!(
    StrokeCap,
    CanvasKitStrokeCapEnumValues,
    CanvasKitStrokeCap,
    {
        Butt: STROKE_CAP_BUTT_VALUE,
        Round: STROKE_CAP_ROUND_VALUE,
        Square: STROKE_CAP_SQUARE_VALUE,
    }
);
canvas_kit_enum!(
    StrokeJoin,
    CanvasKitStrokeJoinEnumValues,
    CanvasKitStrokeJoin,
    {
        Bevel: STROKE_JOIN_BEVEL_VALUE,
        Miter: STROKE_JOIN_MITER_VALUE,
        Round: STROKE_JOIN_ROUND_VALUE,
    }
);
canvas_kit_enum!(
    ClipOp,
    CanvasKitClipOpEnumValues,
    CanvasKitClipOp,
    {
        Difference: CLIP_OP_DIFFERENCE_VALUE,
        Intersect: CLIP_OP_INTERSECT_VALUE,
    }
);
canvas_kit_enum!(
    AlphaType,
    CanvasKitAlphaTypeEnumValues,
    CanvasKitAlphaType,
    {
        Opaque: ALPHA_TYPE_OPAQUE_VALUE,
        Premul: ALPHA_TYPE_PREMUL_VALUE,
        Unpremul: ALPHA_TYPE_UNPREMUL_VALUE,
    }
);
canvas_kit_enum!(
    ColorType,
    CanvasKitColorTypeEnumValues,
    CanvasKitColorType,
    {
        Alpha_8: COLOR_TYPE_ALPHA_8_VALUE,
        Rgb_565: COLOR_TYPE_RGB_565_VALUE,
        Rgba_8888: COLOR_TYPE_RGBA_8888_VALUE,
        Bgra_8888: COLOR_TYPE_BGRA_8888_VALUE,
        Rgba_1010102: COLOR_TYPE_RGBA_1010102_VALUE,
        Rgb_101010x: COLOR_TYPE_RGB_101010X_VALUE,
        Gray_8: COLOR_TYPE_GRAY_8_VALUE,
        Rgba_F16: COLOR_TYPE_RGBA_F16_VALUE,
        Rgba_F32: COLOR_TYPE_RGBA_F32_VALUE,
    }
);
canvas_kit_enum!(
    FilterMode,
    CanvasKitFilterModeEnumValues,
    CanvasKitFilterMode,
    {
        Linear: FILTER_MODE_LINEAR_VALUE,
        Nearest: FILTER_MODE_NEAREST_VALUE,
    }
);
canvas_kit_enum!(
    MipmapMode,
    CanvasKitMipmapModeEnumValues,
    CanvasKitMipmapMode,
    {
        None: MIPMAP_MODE_NONE_VALUE,
        Nearest: MIPMAP_MODE_NEAREST_VALUE,
        Linear: MIPMAP_MODE_LINEAR_VALUE,
    }
);
