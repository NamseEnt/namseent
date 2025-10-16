// include!("src/game_state/monster/monster_kind.rs");

// const MONSTER_KINDS: [MonsterKind; 42] = [
//     MonsterKind::Mob01,
//     MonsterKind::Mob02,
//     MonsterKind::Mob03,
//     MonsterKind::Mob04,
//     MonsterKind::Mob05,
//     MonsterKind::Mob06,
//     MonsterKind::Mob07,
//     MonsterKind::Mob08,
//     MonsterKind::Mob09,
//     MonsterKind::Mob10,
//     MonsterKind::Mob11,
//     MonsterKind::Mob12,
//     MonsterKind::Mob13,
//     MonsterKind::Mob14,
//     MonsterKind::Mob15,
//     MonsterKind::Named01,
//     MonsterKind::Named02,
//     MonsterKind::Named03,
//     MonsterKind::Named04,
//     MonsterKind::Named05,
//     MonsterKind::Named06,
//     MonsterKind::Named07,
//     MonsterKind::Named08,
//     MonsterKind::Named09,
//     MonsterKind::Named10,
//     MonsterKind::Named11,
//     MonsterKind::Named12,
//     MonsterKind::Named13,
//     MonsterKind::Named14,
//     MonsterKind::Named15,
//     MonsterKind::Named16,
//     MonsterKind::Boss01,
//     MonsterKind::Boss02,
//     MonsterKind::Boss03,
//     MonsterKind::Boss04,
//     MonsterKind::Boss05,
//     MonsterKind::Boss06,
//     MonsterKind::Boss07,
//     MonsterKind::Boss08,
//     MonsterKind::Boss09,
//     MonsterKind::Boss10,
//     MonsterKind::Boss11,
// ];

// const ORIGINAL_SIZE: f32 = 36.0;
// const OUTPUT_SIZE: u32 = 128;

fn main() {
    // TODO
    //     println!("cargo:rerun-if-changed=src/game_state/monster/monster_kind.rs");

    //     let monster_directory = Path::new("asset/image/monster");
    //     fs::create_dir_all(monster_directory).unwrap();

    //     MONSTER_KINDS.par_iter().for_each(|kind| {
    //         let filename = format!("{}.png", kind.asset_id());
    //         let output_path = monster_directory.join(&filename);
    //         if output_path.exists() {
    //             return;
    //         }

    //         let svg = SvgTwemojiAsset::from_emoji(kind.emoji())
    //             .unwrap_or_else(|| panic!("Emoji not found: {}", kind.emoji()));
    //         save_svg_to_png(svg.as_bytes(), &output_path);
    //     });
}

// fn save_svg_to_png(svg_data: &[u8], output_path: &Path) {
//     let svg_text = String::from_utf8_lossy(svg_data);
//     let options = resvg::usvg::Options {
//         default_size: resvg::usvg::Size::from_wh(ORIGINAL_SIZE, ORIGINAL_SIZE).unwrap(),
//         ..Default::default()
//     };
//     let tree = resvg::usvg::Tree::from_str(&svg_text, &options).unwrap();

//     let mut pixmap = resvg::tiny_skia::Pixmap::new(OUTPUT_SIZE, OUTPUT_SIZE).unwrap();
//     let transform = resvg::tiny_skia::Transform::from_scale(
//         OUTPUT_SIZE as f32 / ORIGINAL_SIZE,
//         OUTPUT_SIZE as f32 / ORIGINAL_SIZE,
//     );
//     resvg::render(&tree, transform, &mut pixmap.as_mut());
//     pixmap.save_png(output_path).unwrap();
// }
