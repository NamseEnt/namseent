use psd_sprite::PartsSprite;

fn main() {
    let psd_bytes = include_bytes!("test.psd");
    let parts_sprite = PartsSprite::from_psd_bytes(psd_bytes).unwrap();
    for (name, part) in parts_sprite.parts {
        match part.kind {
            psd_sprite::SpritePartKind::Fixed { image } => {
                std::fs::write(format!("test/{name}.webp"), image.webp).unwrap();
            }
            psd_sprite::SpritePartKind::SingleSelect { options }
            | psd_sprite::SpritePartKind::MultiSelect { options } => {
                for option in options {
                    std::fs::create_dir_all(format!("test/{name}")).unwrap();
                    std::fs::write(
                        format!("test/{name}/{}.webp", option.name),
                        option.image.webp,
                    )
                    .unwrap();
                }
            }
        }
    }
}
