use psd_sprite::{skia_util::sk_image_to_webp, PartsSprite, PartsSpriteLock};
use std::collections::BTreeSet;

fn main() {
    let psd_bytes = include_bytes!("test.psd");
    let parts_sprite = PartsSprite::from_psd_bytes(psd_bytes, "test".to_string()).unwrap();
    print_parts_sprite_names(&parts_sprite, 0);

    let lock = PartsSpriteLock {
        name: "test".to_string(),
        part_names: BTreeSet::new(),
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
