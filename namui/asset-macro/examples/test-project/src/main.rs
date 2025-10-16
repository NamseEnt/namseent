use namui_asset_macro::register_assets;

#[derive(Debug)]
struct Image {
    id: usize,
}

register_assets!();

fn main() {
    println!("Asset macro test!");

    println!("HELLO image id: {}", asset::my::image::HELLO.id);
    println!("WORLD image id: {}", asset::my::image::WORLD.id);
    println!("TEST image id: {}", asset::other::TEST.id);

    println!("Image struct: {:?}", asset::my::image::HELLO);
}
