use namui_type::*;

fn main() {
    let path = Path::new().add_rect(Rect::Ltrb {
        left: 5.px(),
        top: 5.px(),
        right: 5.px(),
        bottom: 5.px(),
    });

    let bounding_box = path.bounding_box();
    println!("{:?}", bounding_box);
}
