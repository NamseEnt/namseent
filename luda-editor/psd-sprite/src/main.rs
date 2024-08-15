use namui::{PercentExt, Xy};
use psd_sprite::{skia_util::sk_image_to_webp, PsdSprite};
use schema_0::{Circumcircle, SceneSprite};
use std::collections::{HashMap, HashSet};

fn main() {
    let psd_bytes = include_bytes!("test.psd");
    let psd_sprite = PsdSprite::from_psd_bytes(psd_bytes).unwrap();
    let scene_sprite = SceneSprite {
        sprite_id: None,
        circumcircle: Circumcircle {
            xy: Xy::zero(),
            radius: 0.percent(),
        },
        part_option_selections: HashMap::from_iter([
            (
                "눈_s".to_string(),
                HashSet::from_iter(["눈_s.옆보는".to_string()]),
            ),
            (
                "눈썹_s".to_string(),
                HashSet::from_iter(["눈썹_s.슬픔".to_string()]),
            ),
            (
                "입_s".to_string(),
                HashSet::from_iter(["입_s.놀람".to_string()]),
            ),
            (
                "코_s".to_string(),
                HashSet::from_iter(["코_s.코".to_string()]),
            ),
            (
                "홍조_s".to_string(),
                HashSet::from_iter(["홍조_s.레이어 80".to_string()]),
            ),
        ]),
    };
    let image_filter = psd_sprite.render(&scene_sprite).unwrap().unwrap();
    let image_info =
        skia_safe::ImageInfo::new_n32((1024, 2048), skia_safe::AlphaType::Unpremul, None);
    let mut surface = skia_safe::surfaces::raster(&image_info, None, None).unwrap();
    let canvas = surface.canvas();
    let mut paint = skia_safe::Paint::default();
    paint.set_image_filter(image_filter);
    canvas.draw_rect(skia_safe::Rect::from_isize((1024, 2048)), &paint);
    let image = surface.image_snapshot();
    let webp_bytes = sk_image_to_webp(&image).unwrap();
    std::fs::write("test.webp", webp_bytes).unwrap();
}
