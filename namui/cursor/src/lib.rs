use namui_skia::*;
use namui_type::*;
pub use roxmltree::Error as XmlParserError;
use roxmltree::Node;

pub enum Error {
    XmlParserError(XmlParserError),
    // UnexpectedToken(xmlparser::TextPos),
    FloatParseError(std::num::ParseFloatError),
}
impl From<XmlParserError> for Error {
    fn from(err: XmlParserError) -> Self {
        Error::XmlParserError(err)
    }
}
impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::FloatParseError(err)
    }
}

pub fn parse(svg: &str) -> Result<RenderingTree, Error> {
    let document = roxmltree::Document::parse(svg)?;
    let root = document.root_element();
    // visit(&mut Tokenizer::from(svg))

    todo!()
}

pub fn visit(node: Node) -> Result<RenderingTree, Error> {
    match node.tag_name().name() {
        "svg" => {
            let width = node
                .attribute("width")
                .expect("unimplemented: svg no width")
                .parse::<f32>()?;
            let height = node
                .attribute("height")
                .expect("svg height not found")
                .parse::<f32>()?;
            let mut view_box_attr = node
                .attribute("viewBox")
                .expect("svg viewBox not found")
                .split_whitespace()
                .map(|s| s.parse::<f32>());
            let view_box = Rect::Xywh {
                x: view_box_attr.next().expect("svg viewBox x not found")?,
                y: view_box_attr.next().expect("svg viewBox y not found")?,
                width: view_box_attr.next().expect("svg viewBox width not found")?,
                height: view_box_attr
                    .next()
                    .expect("svg viewBox height not found")?,
            };
            let children_rendering_tree = node
                .children()
                .map(visit)
                .collect::<Result<Vec<RenderingTree>, Error>>()?;
            let scale_x = width / view_box.width();
            let scale_y = height / view_box.height();
            Ok(transform(
                TransformMatrix::from_slice([
                    [scale_x, 0.0, -view_box.x() * scale_x],
                    [0.0, scale_y, -view_box.y() * scale_y],
                ]),
                RenderingTree::Children(children_rendering_tree),
            ))
        }
        _ => panic!("unimplemented: {}", node.tag_name().name()),
    }
}
