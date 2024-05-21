use crate::*;
use canvas_kit_wasm_bindgen::*;

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
        impl From<$enum_name> for &'static $canvas_enum_name {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$enum_item => $static_enum_item(),
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
        Fill: paint_style_fill,
        Stroke: paint_style_stroke,
    }
);
canvas_kit_enum!(
    StrokeCap,
    CanvasKitStrokeCapEnumValues,
    CanvasKitStrokeCap,
    {
        Butt: stroke_cap_butt,
        Round: stroke_cap_round,
        Square: stroke_cap_square,
    }
);
canvas_kit_enum!(
    StrokeJoin,
    CanvasKitStrokeJoinEnumValues,
    CanvasKitStrokeJoin,
    {
        Bevel: stroke_join_bevel,
        Miter: stroke_join_miter,
        Round: stroke_join_round,
    }
);
canvas_kit_enum!(
    ClipOp,
    CanvasKitClipOpEnumValues,
    CanvasKitClipOp,
    {
        Difference: clip_op_difference,
        Intersect: clip_op_intersect,
    }
);
canvas_kit_enum!(
    AlphaType,
    CanvasKitAlphaTypeEnumValues,
    CanvasKitAlphaType,
    {
        Opaque: alpha_type_opaque,
        Premul: alpha_type_premul,
        Unpremul: alpha_type_unpremul,
    }
);
canvas_kit_enum!(
    ColorType,
    CanvasKitColorTypeEnumValues,
    CanvasKitColorType,
    {
        Alpha8: Alpha_8: color_type_alpha_8,
        Rgb565: RGB_565: color_type_rgb_565,
        Rgba8888: RGBA_8888: color_type_rgba_8888,
        Bgra8888: BGRA_8888: color_type_bgra_8888,
        Rgba1010102: RGBA_1010102: color_type_rgba_1010102,
        Rgb101010x: RGB_101010x: color_type_rgb_101010x,
        Gray8: Gray_8: color_type_gray_8,
        RgbaF16: RGBA_F16: color_type_rgba_f16,
        RgbaF32: RGBA_F32: color_type_rgba_f32,
    }
);
canvas_kit_enum!(
    FilterMode,
    CanvasKitFilterModeEnumValues,
    CanvasKitFilterMode,
    {
        Linear: filter_mode_linear,
        Nearest: filter_mode_nearest,
    }
);
canvas_kit_enum!(
    MipmapMode,
    CanvasKitMipmapModeEnumValues,
    CanvasKitMipmapMode,
    {
        None: mipmap_mode_none,
        Nearest: mipmap_mode_nearest,
        Linear: mipmap_mode_linear,
    }
);
canvas_kit_enum!(
    BlendMode,
    CanvasKitBlendModeEnumValues,
    CanvasKitBlendMode,
    {
        Clear: blend_mode_clear,
        Src: blend_mode_src,
        Dst: blend_mode_dst,
        SrcOver: blend_mode_src_over,
        DstOver: blend_mode_dst_over,
        SrcIn: blend_mode_src_in,
        DstIn: blend_mode_dst_in,
        SrcOut: blend_mode_src_out,
        DstOut: blend_mode_dst_out,
        SrcATop: blend_mode_src_atop,
        DstATop: blend_mode_dst_atop,
        Xor: blend_mode_xor,
        Plus: blend_mode_plus,
        Modulate: blend_mode_modulate,
        Screen: blend_mode_screen,
        Overlay: blend_mode_overlay,
        Darken: blend_mode_darken,
        Lighten: blend_mode_lighten,
        ColorDodge: blend_mode_color_dodge,
        ColorBurn: blend_mode_color_burn,
        HardLight: blend_mode_hard_light,
        SoftLight: blend_mode_soft_light,
        Difference: blend_mode_difference,
        Exclusion: blend_mode_exclusion,
        Multiply: blend_mode_multiply,
        Hue: blend_mode_hue,
        Saturation: blend_mode_saturation,
        Color: blend_mode_color,
        Luminosity: blend_mode_luminosity,
    }
);
canvas_kit_enum!(TileMode, CanvasKitTileModeEnumValues, CanvasKitTileMode, {
    Clamp: tile_mode_clamp,
    Decal: tile_mode_decal,
    Mirror: tile_mode_mirror,
    Repeat: tile_mode_repeat,
});

canvas_kit_enum!(ColorSpace, CanvasKitColorSpaceEnumValues, CanvasKitColorSpace, {
    Srgb: SRGB: color_space_srgb,
    DisplayP3: DISPLAY_P3: color_space_display_p3,
    AdobeRgb: ADOBE_RGB: color_space_adobe_rgb,
});

canvas_kit_enum!(BlurStyle, CanvasKitBlurStyleEnumValues, CanvasKitBlurStyle, {
    Normal: blur_style_normal,
    Solid: blur_style_solid,
    Outer: blur_style_outer,
    Inner: blur_style_inner,
});
