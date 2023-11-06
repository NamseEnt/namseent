use super::*;
use lazy_static::lazy_static;
use namui_type::*;
use once_cell::sync::Lazy;

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
        canvas_kit_enum!(
            $enum_name,
            $canvas_enum_values_name,
            $canvas_enum_name,
            {
                $($enum_item: $enum_item: $static_enum_item),*
            }
        );
    };
    (
        $enum_name:ident,
        $canvas_enum_values_name:ident,
        $canvas_enum_name:ident,
        {
            $($enum_item:ident: $canvas_kit_enum_item:ident: $static_enum_item:ident),* $(,)?
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
                pub fn $canvas_kit_enum_item(this: &$canvas_enum_values_name) -> $canvas_enum_name;
            )*
        }


        lazy_static! {
            $(
                pub static ref $static_enum_item: Lazy<f32> =
                    Lazy::new(|| canvas_kit().$enum_name().$canvas_kit_enum_item().value());
            )*
        }


        impl From<$enum_name> for $canvas_enum_name {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$enum_item => canvas_kit().$enum_name().$canvas_kit_enum_item(),
                    )*
                }
            }
        }
    };
}

pub(crate) trait IntoCanvasKitEnum {
    fn as_canvas_kit_enum(&self) -> EmbindEnumEntity;
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
        Alpha8: Alpha_8: COLOR_TYPE_ALPHA_8_VALUE,
        Rgb565: RGB_565: COLOR_TYPE_RGB_565_VALUE,
        Rgba8888: RGBA_8888: COLOR_TYPE_RGBA_8888_VALUE,
        Bgra8888: BGRA_8888: COLOR_TYPE_BGRA_8888_VALUE,
        Rgba1010102: RGBA_1010102: COLOR_TYPE_RGBA_1010102_VALUE,
        Rgb101010x: RGB_101010x: COLOR_TYPE_RGB_101010X_VALUE,
        Gray8: Gray_8: COLOR_TYPE_GRAY_8_VALUE,
        RgbaF16: RGBA_F16: COLOR_TYPE_RGBA_F16_VALUE,
        RgbaF32: RGBA_F32: COLOR_TYPE_RGBA_F32_VALUE,
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
canvas_kit_enum!(
    BlendMode,
    CanvasKitBlendModeEnumValues,
    CanvasKitBlendMode,
    {
        Clear: BLEND_MODE_CLEAR_VALUE,
        Src: BLEND_MODE_SRC_VALUE,
        Dst: BLEND_MODE_DST_VALUE,
        SrcOver: BLEND_MODE_SRC_OVER_VALUE,
        DstOver: BLEND_MODE_DST_OVER_VALUE,
        SrcIn: BLEND_MODE_SRC_IN_VALUE,
        DstIn: BLEND_MODE_DST_IN_VALUE,
        SrcOut: BLEND_MODE_SRC_OUT_VALUE,
        DstOut: BLEND_MODE_DST_OUT_VALUE,
        SrcATop: BLEND_MODE_SRC_ATOP_VALUE,
        DstATop: BLEND_MODE_DST_ATOP_VALUE,
        Xor: BLEND_MODE_XOR_VALUE,
        Plus: BLEND_MODE_PLUS_VALUE,
        Modulate: BLEND_MODE_MODULATE_VALUE,
        Screen: BLEND_MODE_SCREEN_VALUE,
        Overlay: BLEND_MODE_OVERLAY_VALUE,
        Darken: BLEND_MODE_DARKEN_VALUE,
        Lighten: BLEND_MODE_LIGHTEN_VALUE,
        ColorDodge: BLEND_MODE_COLOR_DODGE_VALUE,
        ColorBurn: BLEND_MODE_COLOR_BURN_VALUE,
        HardLight: BLEND_MODE_HARD_LIGHT_VALUE,
        SoftLight: BLEND_MODE_SOFT_LIGHT_VALUE,
        Difference: BLEND_MODE_DIFFERENCE_VALUE,
        Exclusion: BLEND_MODE_EXCLUSION_VALUE,
        Multiply: BLEND_MODE_MULTIPLY_VALUE,
        Hue: BLEND_MODE_HUE_VALUE,
        Saturation: BLEND_MODE_SATURATION_VALUE,
        Color: BLEND_MODE_COLOR_VALUE,
        Luminosity: BLEND_MODE_LUMINOSITY_VALUE,
    }
);
canvas_kit_enum!(TileMode, CanvasKitTileModeEnumValues, CanvasKitTileMode, {
    Clamp: TILE_MODE_CLAMP_VALUE,
    Decal: TILE_MODE_DECAL_VALUE,
    Mirror: TILE_MODE_MIRROR_VALUE,
    Repeat: TILE_MODE_REPEAT_VALUE,
});

canvas_kit_enum!(ColorSpace, CanvasKitColorSpaceEnumValues, CanvasKitColorSpace, {
    Srgb: SRGB: COLOR_SPACE_SRGB_VALUE,
    DisplayP3: DISPLAY_P3: COLOR_SPACE_DISPLAY_P3_VALUE,
    AdobeRgb: ADOBE_RGB: COLOR_SPACE_ADOBE_RGB_VALUE,
});
