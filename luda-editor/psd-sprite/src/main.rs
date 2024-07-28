use psd_sprite::{skia_util::sk_image_to_webp, PartsSprite, PartsSpriteLock};
use std::collections::BTreeSet;

fn main() {
    let psd_bytes = include_bytes!("test.psd");
    let parts_sprite = PartsSprite::from_psd_bytes(psd_bytes, "test".to_string()).unwrap();
    print_parts_sprite_names(&parts_sprite, 0);

    let lock = PartsSpriteLock {
        name: "test".to_string(),
        part_names: BTreeSet::from_iter(
            [
                "입_s.놀람",
                "홍조_s.레이어 80",
                "코_s.코",
                "눈_s.일반",
                "눈썹_s.슬픔",
            ]
            .map(|name| name.to_string()),
        ),
    };
    let image = parts_sprite.render(&lock).unwrap();
    let webp_bytes = sk_image_to_webp(&image).unwrap();
    std::fs::write("test.webp", webp_bytes).unwrap();
}

fn print_parts_sprite_names(parts_sprite: &PartsSprite, indent: usize) {
    for _ in 0..indent {
        print!("   ");
    }
    println!("{}", parts_sprite.name);
    match &parts_sprite.kind {
        psd_sprite::SpritePartKind::SingleSelect { options: entries }
        | psd_sprite::SpritePartKind::MultiSelect { options: entries }
        | psd_sprite::SpritePartKind::Directory { entries } => {
            for entry in entries {
                print_parts_sprite_names(&entry, indent + 1);
            }
        }
        psd_sprite::SpritePartKind::Fixed { .. } => {}
    }
}
