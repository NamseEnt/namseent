use crate::*;
use bincode::{Decode, de::Decoder, error::DecodeError};

type Ctx = ();

fn decode_ref<D, T>(decoder: &mut D) -> Result<&'static T, DecodeError>
where
    D: Decoder<Context = Ctx>,
    T: Decode<Ctx>,
{
    Ok(arena_alloc(T::decode(decoder)?))
}

impl Decode<Ctx> for RenderingTree {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(match u32::decode(d)? {
            0 => RenderingTree::Empty,
            1 => RenderingTree::Node(DrawCommand::decode(d)?),
            2 => {
                let children: Vec<RenderingTree> = Vec::decode(d)?;
                RenderingTree::Children(arena_alloc_slice(children))
            }
            3 => RenderingTree::Special(SpecialRenderingNode::decode(d)?),
            _ => return Err(DecodeError::Other("invalid RenderingTree variant")),
        })
    }
}

impl Decode<Ctx> for DrawCommand {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(match u32::decode(d)? {
            0 => DrawCommand::Path {
                command: decode_ref(d)?,
            },
            1 => DrawCommand::Text {
                command: decode_ref(d)?,
            },
            2 => DrawCommand::Image {
                command: decode_ref(d)?,
            },
            _ => return Err(DecodeError::Other("invalid DrawCommand variant")),
        })
    }
}

impl Decode<Ctx> for SpecialRenderingNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(match u32::decode(d)? {
            0 => SpecialRenderingNode::Translate(TranslateNode::decode(d)?),
            1 => SpecialRenderingNode::Clip(ClipNode::decode(d)?),
            2 => SpecialRenderingNode::Absolute(AbsoluteNode::decode(d)?),
            3 => SpecialRenderingNode::Rotate(RotateNode::decode(d)?),
            4 => SpecialRenderingNode::Scale(ScaleNode::decode(d)?),
            5 => SpecialRenderingNode::Transform(TransformNode::decode(d)?),
            6 => SpecialRenderingNode::OnTop(OnTopNode::decode(d)?),
            7 => SpecialRenderingNode::MouseCursor(MouseCursorNode::decode(d)?),
            _ => return Err(DecodeError::Other("invalid SpecialRenderingNode variant")),
        })
    }
}

impl Decode<Ctx> for TranslateNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(TranslateNode {
            x: Decode::decode(d)?,
            y: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for AbsoluteNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(AbsoluteNode {
            x: Decode::decode(d)?,
            y: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for RotateNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(RotateNode {
            angle: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for ScaleNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(ScaleNode {
            x: Decode::decode(d)?,
            y: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for TransformNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(TransformNode {
            matrix: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for OnTopNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(OnTopNode {
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for ClipNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(ClipNode {
            path: decode_ref(d)?,
            clip_op: Decode::decode(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

impl Decode<Ctx> for MouseCursor {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(match u32::decode(d)? {
            0 => MouseCursor::Standard(Decode::decode(d)?),
            1 => MouseCursor::Custom(RenderingTree::decode(d)?),
            _ => return Err(DecodeError::Other("invalid MouseCursor variant")),
        })
    }
}

impl Decode<Ctx> for MouseCursorNode {
    fn decode<D: Decoder<Context = Ctx>>(d: &mut D) -> Result<Self, DecodeError> {
        Ok(MouseCursorNode {
            cursor: decode_ref(d)?,
            rendering_tree: decode_ref(d)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use namui_type::*;

    #[test]
    fn rendering_tree_encode_decode_round_trip() {
        let tree = RenderingTree::Children(arena_alloc_slice(vec![
            RenderingTree::Empty,
            RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
                x: 3.px(),
                y: 4.px(),
                rendering_tree: arena_alloc(RenderingTree::Special(SpecialRenderingNode::OnTop(
                    OnTopNode {
                        rendering_tree: arena_alloc(RenderingTree::Empty),
                    },
                ))),
            })),
        ]));

        let bytes = bincode::encode_to_vec(tree, bincode::config::standard()).unwrap();
        let (decoded, _): (RenderingTree, usize) =
            bincode::decode_from_slice(&bytes, bincode::config::standard()).unwrap();

        assert_eq!(tree, decoded);
    }
}
