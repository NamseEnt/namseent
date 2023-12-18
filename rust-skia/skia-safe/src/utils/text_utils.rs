use crate::{prelude::*, Canvas, Font, Paint, Path, Point, TextEncoding};
use skia_bindings::SkTextUtils;

pub use skia_bindings::SkTextUtils_Align as Align;
variant_name!(Align::Center);

pub fn draw_str(
    canvas: &Canvas,
    text: impl AsRef<str>,
    p: impl Into<Point>,
    font: &Font,
    paint: &Paint,
    align: Align,
) {
    let text = text.as_ref().as_bytes();
    let p = p.into();
    unsafe {
        SkTextUtils::Draw(
            canvas.native_mut(),
            text.as_ptr() as _,
            text.len(),
            TextEncoding::UTF8.into_native(),
            p.x,
            p.y,
            font.native(),
            paint.native(),
            align,
        )
    }
}

impl Canvas {
    pub fn draw_str_align(
        &self,
        text: impl AsRef<str>,
        p: impl Into<Point>,
        font: &Font,
        paint: &Paint,
        align: Align,
    ) -> &Self {
        draw_str(self, text, p, font, paint, align);
        self
    }
}

pub fn get_path(text: impl AsRef<str>, p: impl Into<Point>, font: &Font) -> Path {
    let text = text.as_ref().as_bytes();
    let p = p.into();
    let mut path = Path::default();
    unsafe {
        SkTextUtils::GetPath(
            text.as_ptr() as _,
            text.len(),
            TextEncoding::UTF8.into_native(),
            p.x,
            p.y,
            font.native(),
            path.native_mut(),
        )
    }
    path
}

impl Path {
    pub fn from_str(text: impl AsRef<str>, p: impl Into<Point>, font: &Font) -> Self {
        get_path(text, p, font)
    }
}
