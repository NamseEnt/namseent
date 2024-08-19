mod layer_tree;
mod psd_sprite;
mod sk_position_image;
pub mod skia_util;

pub use psd_sprite::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_tree() {
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
        let _image_filter = psd_sprite.render(&scene_sprite).unwrap();
    }
}
