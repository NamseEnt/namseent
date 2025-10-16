#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod bounding_box {
    mod draw_command {
        use crate::*;
        use namui_type::*;
        impl BoundingBox for &DrawCommand {
            fn bounding_box(self) -> Option<Rect<Px>> {
                match self {
                    DrawCommand::Path { command } => command.bounding_box(),
                    DrawCommand::Text { command } => command.bounding_box(),
                    DrawCommand::Image { command } => command.bounding_box(),
                }
            }
        }
        impl BoundingBox for &PathDrawCommand {
            fn bounding_box(self) -> Option<Rect<Px>> {
                NativePath::get(&self.path).bounding_box(Some(&self.paint))
            }
        }
        impl BoundingBox for &TextDrawCommand {
            fn bounding_box(self) -> Option<Rect<Px>> {
                if self.text.is_empty() {
                    return None;
                }
                let paragraph = Paragraph::new(
                    &self.text,
                    self.font.clone(),
                    self.paint.clone(),
                    self.max_width,
                );
                let line_height = self.line_height_px();
                let multiline_y_baseline_offset = get_multiline_y_baseline_offset(
                    self.baseline,
                    line_height,
                    paragraph.line_len(),
                );
                paragraph
                    .iter_str()
                    .enumerate()
                    .map(|(index, line_text)| {
                        (
                            self.y + multiline_y_baseline_offset + line_height * index,
                            line_text,
                        )
                    })
                    .map(|(y, line_text)| {
                        self.font
                            .bounds(&line_text, &self.paint)
                            .iter()
                            .map(|bound| (bound.top(), bound.bottom()))
                            .reduce(|acc, (top, bottom)| (acc.0.min(top), acc.1.max(bottom)))
                            .map(|(top, bottom)| {
                                let widths = self.font.widths(&line_text, &self.paint);
                                let width = widths.iter().fold(px(0.0), |prev, curr| prev + curr);
                                let x_axis_anchor = get_left_in_align(self.x, self.align, width);
                                let metrics = self.font.font_metrics();
                                let y_axis_anchor =
                                    y + get_bottom_of_baseline(self.baseline, metrics);
                                Rect::Ltrb {
                                    left: x_axis_anchor,
                                    top: top + y_axis_anchor,
                                    right: x_axis_anchor + width,
                                    bottom: bottom + y_axis_anchor,
                                }
                            })
                    })
                    .fold(None, |acc, rect| {
                        if let Some(rect) = rect {
                            if let Some(acc) = acc {
                                Some(Rect::Ltrb {
                                    left: acc.left().min(rect.left()),
                                    top: acc.top().min(rect.top()),
                                    right: acc.right().max(rect.right()),
                                    bottom: acc.bottom().max(rect.bottom()),
                                })
                            } else {
                                Some(rect)
                            }
                        } else {
                            acc
                        }
                    })
            }
        }
        impl BoundingBox for &ImageDrawCommand {
            fn bounding_box(self) -> Option<Rect<Px>> {
                match &self.paint {
                    Some(paint) => {
                        NativePath::get(&Path::new().add_rect(self.rect)).bounding_box(Some(paint))
                    }
                    _ => Some(self.rect),
                }
            }
        }
    }
    mod rendering_tree {
        use crate::*;
        use namui_type::*;
        use std::borrow::Borrow;
        impl BoundingBox for &RenderingTree {
            fn bounding_box(self) -> Option<Rect<Px>> {
                static CACHE: LruCache<RenderingTree, Rect<Px>> = LruCache::new();
                if let Some(cached) = CACHE.get(self) {
                    return Some(*cached);
                }
                struct BoundingBoxContext {
                    bounding_boxes_on_top: Vec<Option<Rect<Px>>>,
                }
                fn get_bounding_box_with_matrix(
                    rendering_tree: &RenderingTree,
                    matrix: &TransformMatrix,
                    bounding_box_context: &mut BoundingBoxContext,
                ) -> Option<Rect<Px>> {
                    fn get_bounding_box_with_matrix_of_rendering_trees<'a>(
                        rendering_trees: impl IntoIterator<Item = &'a RenderingTree>,
                        matrix: &TransformMatrix,
                        bounding_box_context: &mut BoundingBoxContext,
                    ) -> Option<Rect<Px>> {
                        rendering_trees
                            .into_iter()
                            .filter_map(|child| {
                                get_bounding_box_with_matrix(child, matrix, bounding_box_context)
                            })
                            .reduce(|acc, bounding_box| {
                                Rect::get_minimum_rectangle_containing(&acc, bounding_box)
                            })
                    }
                    match rendering_tree {
                        RenderingTree::Children(children) => {
                            get_bounding_box_with_matrix_of_rendering_trees(
                                children,
                                matrix,
                                bounding_box_context,
                            )
                        }
                        RenderingTree::Node(draw_command) => draw_command
                            .bounding_box()
                            .map(|bounding_box| matrix.transform_rect(bounding_box)),
                        RenderingTree::Special(special) => match special {
                            SpecialRenderingNode::Translate(translate) => {
                                let matrix = matrix * translate.get_matrix();
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [translate.rendering_tree.borrow()],
                                    &matrix,
                                    bounding_box_context,
                                )
                            }
                            SpecialRenderingNode::Clip(clip) => {
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [clip.rendering_tree.borrow()],
                                    matrix,
                                    bounding_box_context,
                                )
                                .and_then(|bounding_box| {
                                    let clip_bounding_box = clip
                                        .path
                                        .bounding_box()
                                        .map(|bounding_box| matrix.transform_rect(bounding_box));
                                    match clip.clip_op {
                                        ClipOp::Intersect => {
                                            clip_bounding_box.and_then(|clip_bounding_box| {
                                                bounding_box.intersect(clip_bounding_box)
                                            })
                                        }
                                        ClipOp::Difference => match clip_bounding_box {
                                            Some(clip_bounding_box) => {
                                                if bounding_box == clip_bounding_box {
                                                    return None;
                                                }
                                                let xs = [
                                                    bounding_box.left(),
                                                    bounding_box.right(),
                                                    clip_bounding_box.left(),
                                                    clip_bounding_box.right(),
                                                ];
                                                let ys = [
                                                    bounding_box.top(),
                                                    bounding_box.bottom(),
                                                    clip_bounding_box.top(),
                                                    clip_bounding_box.bottom(),
                                                ];
                                                let sixteen_xys = xs
                                                    .iter()
                                                    .zip(ys.iter())
                                                    .map(|(x, y)| Xy { x: *x, y: *y });
                                                let difference_area_xys =
                                                    sixteen_xys.filter(|xy| {
                                                        (clip_bounding_box.is_xy_outside(*xy)
                                                            || clip_bounding_box
                                                                .is_xy_on_border(*xy))
                                                            && !bounding_box.is_xy_outside(*xy)
                                                    });
                                                difference_area_xys.fold(
                                                    None,
                                                    |acc, xy| match acc {
                                                        Some(rect) => Some(Rect::Ltrb {
                                                            left: rect.left().min(xy.x),
                                                            top: rect.top().min(xy.y),
                                                            right: rect.right().max(xy.x),
                                                            bottom: rect.bottom().max(xy.y),
                                                        }),
                                                        None => Some(Rect::Ltrb {
                                                            left: xy.x,
                                                            top: xy.y,
                                                            right: xy.x,
                                                            bottom: xy.y,
                                                        }),
                                                    },
                                                )
                                            }
                                            None => Some(bounding_box),
                                        },
                                    }
                                })
                            }
                            SpecialRenderingNode::Absolute(absolute) => {
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [absolute.rendering_tree.borrow()],
                                    &absolute.get_matrix(),
                                    bounding_box_context,
                                )
                            }
                            SpecialRenderingNode::Rotate(rotate) => {
                                let matrix = matrix * rotate.get_matrix();
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [rotate.rendering_tree.borrow()],
                                    &matrix,
                                    bounding_box_context,
                                )
                            }
                            SpecialRenderingNode::Scale(scale) => {
                                let matrix = matrix * scale.get_matrix();
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [scale.rendering_tree.borrow()],
                                    &matrix,
                                    bounding_box_context,
                                )
                            }
                            SpecialRenderingNode::Transform(transform) => {
                                let matrix = matrix * transform.matrix;
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [transform.rendering_tree.borrow()],
                                    &matrix,
                                    bounding_box_context,
                                )
                            }
                            SpecialRenderingNode::OnTop(on_top) => {
                                let bounding_box = get_bounding_box_with_matrix_of_rendering_trees(
                                    [on_top.rendering_tree.borrow()],
                                    matrix,
                                    bounding_box_context,
                                );
                                bounding_box_context
                                    .bounding_boxes_on_top
                                    .push(bounding_box);
                                bounding_box
                            }
                            SpecialRenderingNode::MouseCursor(_) => {
                                get_bounding_box_with_matrix_of_rendering_trees(
                                    [special.inner_rendering_tree_ref()],
                                    matrix,
                                    bounding_box_context,
                                )
                            }
                        },
                        RenderingTree::Empty => None,
                    }
                }
                let mut bounding_box_context = BoundingBoxContext {
                    bounding_boxes_on_top: ::alloc::vec::Vec::new(),
                };
                let bounding_box = get_bounding_box_with_matrix(
                    self,
                    &TransformMatrix::identity(),
                    &mut bounding_box_context,
                );
                let bounding_box = bounding_box_context
                    .bounding_boxes_on_top
                    .into_iter()
                    .flatten()
                    .fold(bounding_box, |acc, bounding_box| {
                        acc.map(|acc| Rect::get_minimum_rectangle_containing(&acc, bounding_box))
                    });
                if let Some(bounding_box) = bounding_box {
                    CACHE.put(self.clone(), bounding_box);
                }
                bounding_box
            }
        }
        impl<'a, T> BoundingBox for T
        where
            T: Iterator<Item = &'a RenderingTree>,
        {
            fn bounding_box(self) -> Option<Rect<Px>> {
                self.filter_map(|rendering_tree| rendering_tree.bounding_box())
                    .reduce(|acc, bounding_box| {
                        Rect::get_minimum_rectangle_containing(&acc, bounding_box)
                    })
            }
        }
    }
    use crate::*;
    use namui_type::*;
    pub trait BoundingBox {
        fn bounding_box(self) -> Option<Rect<Px>>;
    }
    impl BoundingBox for &Path {
        fn bounding_box(self) -> Option<Rect<Px>> {
            NativePath::get(self).bounding_box(None)
        }
    }
}
mod command {
    mod image {
        use crate::*;
        pub struct ImageDrawCommand {
            pub rect: Rect<Px>,
            pub image: Image,
            pub fit: ImageFit,
            pub paint: Option<Paint>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ImageDrawCommand {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ImageDrawCommand",
                    "rect",
                    &self.rect,
                    "image",
                    &self.image,
                    "fit",
                    &self.fit,
                    "paint",
                    &&self.paint,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ImageDrawCommand {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ImageDrawCommand {
            #[inline]
            fn eq(&self, other: &ImageDrawCommand) -> bool {
                self.rect == other.rect
                    && self.image == other.image
                    && self.fit == other.fit
                    && self.paint == other.paint
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ImageDrawCommand {
            #[inline]
            fn clone(&self) -> ImageDrawCommand {
                ImageDrawCommand {
                    rect: ::core::clone::Clone::clone(&self.rect),
                    image: ::core::clone::Clone::clone(&self.image),
                    fit: ::core::clone::Clone::clone(&self.fit),
                    paint: ::core::clone::Clone::clone(&self.paint),
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ImageDrawCommand {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.rect, state);
                ::core::hash::Hash::hash(&self.image, state);
                ::core::hash::Hash::hash(&self.fit, state);
                ::core::hash::Hash::hash(&self.paint, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ImageDrawCommand {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Image>;
                let _: ::core::cmp::AssertParamIsEq<ImageFit>;
                let _: ::core::cmp::AssertParamIsEq<Option<Paint>>;
            }
        }
        impl bincode::Encode for ImageDrawCommand {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.rect, encoder)?;
                bincode::Encode::encode(&self.image, encoder)?;
                bincode::Encode::encode(&self.fit, encoder)?;
                bincode::Encode::encode(&self.paint, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for ImageDrawCommand {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    rect: bincode::Decode::decode(decoder)?,
                    image: bincode::Decode::decode(decoder)?,
                    fit: bincode::Decode::decode(decoder)?,
                    paint: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for ImageDrawCommand {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("rect");
                let field_bytes = Serialize::serialize(&self.rect);
                buffer.put_slice(&field_bytes);
                buffer.write_string("image");
                let field_bytes = Serialize::serialize(&self.image);
                buffer.put_slice(&field_bytes);
                buffer.write_string("fit");
                let field_bytes = Serialize::serialize(&self.fit);
                buffer.put_slice(&field_bytes);
                buffer.write_string("paint");
                let field_bytes = Serialize::serialize(&self.paint);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for ImageDrawCommand {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("rect")?;
                let rect = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("image")?;
                let image = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("fit")?;
                let fit = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("paint")?;
                let paint = Deserialize::deserialize(buf)?;
                Ok(Self {
                    rect,
                    image,
                    fit,
                    paint,
                })
            }
        }
    }
    mod path {
        use crate::*;
        pub struct PathDrawCommand {
            pub path: Path,
            pub paint: Paint,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PathDrawCommand {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PathDrawCommand",
                    "path",
                    &self.path,
                    "paint",
                    &&self.paint,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PathDrawCommand {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PathDrawCommand {
            #[inline]
            fn eq(&self, other: &PathDrawCommand) -> bool {
                self.path == other.path && self.paint == other.paint
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PathDrawCommand {
            #[inline]
            fn clone(&self) -> PathDrawCommand {
                PathDrawCommand {
                    path: ::core::clone::Clone::clone(&self.path),
                    paint: ::core::clone::Clone::clone(&self.paint),
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for PathDrawCommand {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.path, state);
                ::core::hash::Hash::hash(&self.paint, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for PathDrawCommand {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Path>;
                let _: ::core::cmp::AssertParamIsEq<Paint>;
            }
        }
        impl bincode::Encode for PathDrawCommand {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.path, encoder)?;
                bincode::Encode::encode(&self.paint, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for PathDrawCommand {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    path: bincode::Decode::decode(decoder)?,
                    paint: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for PathDrawCommand {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("path");
                let field_bytes = Serialize::serialize(&self.path);
                buffer.put_slice(&field_bytes);
                buffer.write_string("paint");
                let field_bytes = Serialize::serialize(&self.paint);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for PathDrawCommand {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("path")?;
                let path = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("paint")?;
                let paint = Deserialize::deserialize(buf)?;
                Ok(Self { path, paint })
            }
        }
    }
    mod text {
        use crate::*;
        pub struct TextDrawCommand {
            pub text: String,
            pub font: Font,
            pub x: Px,
            pub y: Px,
            pub paint: Paint,
            pub align: TextAlign,
            pub baseline: TextBaseline,
            pub max_width: Option<Px>,
            pub line_height_percent: Percent,
            pub underline: Option<Box<Paint>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TextDrawCommand {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "text",
                    "font",
                    "x",
                    "y",
                    "paint",
                    "align",
                    "baseline",
                    "max_width",
                    "line_height_percent",
                    "underline",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.text,
                    &self.font,
                    &self.x,
                    &self.y,
                    &self.paint,
                    &self.align,
                    &self.baseline,
                    &self.max_width,
                    &self.line_height_percent,
                    &&self.underline,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "TextDrawCommand",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TextDrawCommand {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TextDrawCommand {
            #[inline]
            fn eq(&self, other: &TextDrawCommand) -> bool {
                self.text == other.text
                    && self.font == other.font
                    && self.x == other.x
                    && self.y == other.y
                    && self.paint == other.paint
                    && self.align == other.align
                    && self.baseline == other.baseline
                    && self.max_width == other.max_width
                    && self.line_height_percent == other.line_height_percent
                    && self.underline == other.underline
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TextDrawCommand {
            #[inline]
            fn clone(&self) -> TextDrawCommand {
                TextDrawCommand {
                    text: ::core::clone::Clone::clone(&self.text),
                    font: ::core::clone::Clone::clone(&self.font),
                    x: ::core::clone::Clone::clone(&self.x),
                    y: ::core::clone::Clone::clone(&self.y),
                    paint: ::core::clone::Clone::clone(&self.paint),
                    align: ::core::clone::Clone::clone(&self.align),
                    baseline: ::core::clone::Clone::clone(&self.baseline),
                    max_width: ::core::clone::Clone::clone(&self.max_width),
                    line_height_percent: ::core::clone::Clone::clone(&self.line_height_percent),
                    underline: ::core::clone::Clone::clone(&self.underline),
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for TextDrawCommand {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.text, state);
                ::core::hash::Hash::hash(&self.font, state);
                ::core::hash::Hash::hash(&self.x, state);
                ::core::hash::Hash::hash(&self.y, state);
                ::core::hash::Hash::hash(&self.paint, state);
                ::core::hash::Hash::hash(&self.align, state);
                ::core::hash::Hash::hash(&self.baseline, state);
                ::core::hash::Hash::hash(&self.max_width, state);
                ::core::hash::Hash::hash(&self.line_height_percent, state);
                ::core::hash::Hash::hash(&self.underline, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for TextDrawCommand {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<Font>;
                let _: ::core::cmp::AssertParamIsEq<Px>;
                let _: ::core::cmp::AssertParamIsEq<Paint>;
                let _: ::core::cmp::AssertParamIsEq<TextAlign>;
                let _: ::core::cmp::AssertParamIsEq<TextBaseline>;
                let _: ::core::cmp::AssertParamIsEq<Option<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Percent>;
                let _: ::core::cmp::AssertParamIsEq<Option<Box<Paint>>>;
            }
        }
        impl bincode::Encode for TextDrawCommand {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.text, encoder)?;
                bincode::Encode::encode(&self.font, encoder)?;
                bincode::Encode::encode(&self.x, encoder)?;
                bincode::Encode::encode(&self.y, encoder)?;
                bincode::Encode::encode(&self.paint, encoder)?;
                bincode::Encode::encode(&self.align, encoder)?;
                bincode::Encode::encode(&self.baseline, encoder)?;
                bincode::Encode::encode(&self.max_width, encoder)?;
                bincode::Encode::encode(&self.line_height_percent, encoder)?;
                bincode::Encode::encode(&self.underline, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for TextDrawCommand {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    text: bincode::Decode::decode(decoder)?,
                    font: bincode::Decode::decode(decoder)?,
                    x: bincode::Decode::decode(decoder)?,
                    y: bincode::Decode::decode(decoder)?,
                    paint: bincode::Decode::decode(decoder)?,
                    align: bincode::Decode::decode(decoder)?,
                    baseline: bincode::Decode::decode(decoder)?,
                    max_width: bincode::Decode::decode(decoder)?,
                    line_height_percent: bincode::Decode::decode(decoder)?,
                    underline: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for TextDrawCommand {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("text");
                let field_bytes = Serialize::serialize(&self.text);
                buffer.put_slice(&field_bytes);
                buffer.write_string("font");
                let field_bytes = Serialize::serialize(&self.font);
                buffer.put_slice(&field_bytes);
                buffer.write_string("x");
                let field_bytes = Serialize::serialize(&self.x);
                buffer.put_slice(&field_bytes);
                buffer.write_string("y");
                let field_bytes = Serialize::serialize(&self.y);
                buffer.put_slice(&field_bytes);
                buffer.write_string("paint");
                let field_bytes = Serialize::serialize(&self.paint);
                buffer.put_slice(&field_bytes);
                buffer.write_string("align");
                let field_bytes = Serialize::serialize(&self.align);
                buffer.put_slice(&field_bytes);
                buffer.write_string("baseline");
                let field_bytes = Serialize::serialize(&self.baseline);
                buffer.put_slice(&field_bytes);
                buffer.write_string("max_width");
                let field_bytes = Serialize::serialize(&self.max_width);
                buffer.put_slice(&field_bytes);
                buffer.write_string("line_height_percent");
                let field_bytes = Serialize::serialize(&self.line_height_percent);
                buffer.put_slice(&field_bytes);
                buffer.write_string("underline");
                let field_bytes = Serialize::serialize(&self.underline);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for TextDrawCommand {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("text")?;
                let text = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("font")?;
                let font = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("x")?;
                let x = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("y")?;
                let y = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("paint")?;
                let paint = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("align")?;
                let align = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("baseline")?;
                let baseline = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("max_width")?;
                let max_width = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("line_height_percent")?;
                let line_height_percent = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("underline")?;
                let underline = Deserialize::deserialize(buf)?;
                Ok(Self {
                    text,
                    font,
                    x,
                    y,
                    paint,
                    align,
                    baseline,
                    max_width,
                    line_height_percent,
                    underline,
                })
            }
        }
        impl TextDrawCommand {
            pub fn line_height_px(&self) -> Px {
                self.font.size.into_px() * self.line_height_percent
            }
        }
    }
    use crate::*;
    pub use image::*;
    pub use path::*;
    pub use text::*;
    pub enum DrawCommand {
        Path { command: Box<PathDrawCommand> },
        Text { command: Box<TextDrawCommand> },
        Image { command: Box<ImageDrawCommand> },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DrawCommand {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DrawCommand::Path { command: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "Path", "command", &__self_0,
                    )
                }
                DrawCommand::Text { command: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "Text", "command", &__self_0,
                    )
                }
                DrawCommand::Image { command: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "Image", "command", &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DrawCommand {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DrawCommand {
        #[inline]
        fn eq(&self, other: &DrawCommand) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        DrawCommand::Path { command: __self_0 },
                        DrawCommand::Path { command: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        DrawCommand::Text { command: __self_0 },
                        DrawCommand::Text { command: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        DrawCommand::Image { command: __self_0 },
                        DrawCommand::Image { command: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DrawCommand {
        #[inline]
        fn clone(&self) -> DrawCommand {
            match self {
                DrawCommand::Path { command: __self_0 } => DrawCommand::Path {
                    command: ::core::clone::Clone::clone(__self_0),
                },
                DrawCommand::Text { command: __self_0 } => DrawCommand::Text {
                    command: ::core::clone::Clone::clone(__self_0),
                },
                DrawCommand::Image { command: __self_0 } => DrawCommand::Image {
                    command: ::core::clone::Clone::clone(__self_0),
                },
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DrawCommand {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                DrawCommand::Path { command: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                DrawCommand::Text { command: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                DrawCommand::Image { command: __self_0 } => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for DrawCommand {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Box<PathDrawCommand>>;
            let _: ::core::cmp::AssertParamIsEq<Box<TextDrawCommand>>;
            let _: ::core::cmp::AssertParamIsEq<Box<ImageDrawCommand>>;
        }
    }
    impl bincode::Encode for DrawCommand {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Path { command } => {
                    bincode::Encode::encode(&0u32, encoder)?;
                    bincode::Encode::encode(command, encoder)?;
                }
                Self::Text { command } => {
                    bincode::Encode::encode(&1u32, encoder)?;
                    bincode::Encode::encode(command, encoder)?;
                }
                Self::Image { command } => {
                    bincode::Encode::encode(&2u32, encoder)?;
                    bincode::Encode::encode(command, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for DrawCommand {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Path {
                    command: bincode::Decode::decode(decoder)?,
                }),
                1u32 => Ok(Self::Text {
                    command: bincode::Decode::decode(decoder)?,
                }),
                2u32 => Ok(Self::Image {
                    command: bincode::Decode::decode(decoder)?,
                }),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for DrawCommand {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Path { command } => {
                    buffer.write_string("Path");
                    buffer.write_string("command");
                    let field_bytes = Serialize::serialize(command);
                    buffer.put_slice(&field_bytes);
                }
                Self::Text { command } => {
                    buffer.write_string("Text");
                    buffer.write_string("command");
                    let field_bytes = Serialize::serialize(command);
                    buffer.put_slice(&field_bytes);
                }
                Self::Image { command } => {
                    buffer.write_string("Image");
                    buffer.write_string("command");
                    let field_bytes = Serialize::serialize(command);
                    buffer.put_slice(&field_bytes);
                }
            }
            buffer
        }
    }
    impl Deserialize for DrawCommand {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Path" => {
                    let field_name = buf.read_name("command")?;
                    let command = Deserialize::deserialize(buf)?;
                    Ok(Self::Path { command })
                }
                "Text" => {
                    let field_name = buf.read_name("command")?;
                    let command = Deserialize::deserialize(buf)?;
                    Ok(Self::Text { command })
                }
                "Image" => {
                    let field_name = buf.read_name("command")?;
                    let command = Deserialize::deserialize(buf)?;
                    Ok(Self::Image { command })
                }
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
}
mod event {
    mod raw {
        use super::*;
        pub enum RawEvent {
            MouseDown { event: RawMouseEvent },
            MouseMove { event: RawMouseEvent },
            MouseUp { event: RawMouseEvent },
            Wheel { event: RawWheelEvent },
            KeyDown { event: RawKeyboardEvent },
            KeyUp { event: RawKeyboardEvent },
            Blur,
            VisibilityChange,
            ScreenResize { wh: Wh<IntPx> },
            ScreenRedraw,
            TextInput { event: RawTextInputEvent },
            TextInputKeyDown { event: RawTextInputKeyDownEvent },
            TextInputSelectionChange { event: RawTextInputEvent },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    RawEvent::MouseDown { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "MouseDown",
                            "event",
                            &__self_0,
                        )
                    }
                    RawEvent::MouseMove { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "MouseMove",
                            "event",
                            &__self_0,
                        )
                    }
                    RawEvent::MouseUp { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "MouseUp", "event", &__self_0,
                        )
                    }
                    RawEvent::Wheel { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "Wheel", "event", &__self_0,
                        )
                    }
                    RawEvent::KeyDown { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "KeyDown", "event", &__self_0,
                        )
                    }
                    RawEvent::KeyUp { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "KeyUp", "event", &__self_0,
                        )
                    }
                    RawEvent::Blur => ::core::fmt::Formatter::write_str(f, "Blur"),
                    RawEvent::VisibilityChange => {
                        ::core::fmt::Formatter::write_str(f, "VisibilityChange")
                    }
                    RawEvent::ScreenResize { wh: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ScreenResize",
                            "wh",
                            &__self_0,
                        )
                    }
                    RawEvent::ScreenRedraw => ::core::fmt::Formatter::write_str(f, "ScreenRedraw"),
                    RawEvent::TextInput { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "TextInput",
                            "event",
                            &__self_0,
                        )
                    }
                    RawEvent::TextInputKeyDown { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "TextInputKeyDown",
                            "event",
                            &__self_0,
                        )
                    }
                    RawEvent::TextInputSelectionChange { event: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "TextInputSelectionChange",
                            "event",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawEvent {
            #[inline]
            fn clone(&self) -> RawEvent {
                match self {
                    RawEvent::MouseDown { event: __self_0 } => RawEvent::MouseDown {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::MouseMove { event: __self_0 } => RawEvent::MouseMove {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::MouseUp { event: __self_0 } => RawEvent::MouseUp {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::Wheel { event: __self_0 } => RawEvent::Wheel {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::KeyDown { event: __self_0 } => RawEvent::KeyDown {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::KeyUp { event: __self_0 } => RawEvent::KeyUp {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::Blur => RawEvent::Blur,
                    RawEvent::VisibilityChange => RawEvent::VisibilityChange,
                    RawEvent::ScreenResize { wh: __self_0 } => RawEvent::ScreenResize {
                        wh: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::ScreenRedraw => RawEvent::ScreenRedraw,
                    RawEvent::TextInput { event: __self_0 } => RawEvent::TextInput {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::TextInputKeyDown { event: __self_0 } => RawEvent::TextInputKeyDown {
                        event: ::core::clone::Clone::clone(__self_0),
                    },
                    RawEvent::TextInputSelectionChange { event: __self_0 } => {
                        RawEvent::TextInputSelectionChange {
                            event: ::core::clone::Clone::clone(__self_0),
                        }
                    }
                }
            }
        }
        pub struct RawMouseEvent {
            pub xy: Xy<Px>,
            pub pressing_buttons: HashSet<MouseButton>,
            pub button: Option<MouseButton>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawMouseEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "RawMouseEvent",
                    "xy",
                    &self.xy,
                    "pressing_buttons",
                    &self.pressing_buttons,
                    "button",
                    &&self.button,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawMouseEvent {
            #[inline]
            fn clone(&self) -> RawMouseEvent {
                RawMouseEvent {
                    xy: ::core::clone::Clone::clone(&self.xy),
                    pressing_buttons: ::core::clone::Clone::clone(&self.pressing_buttons),
                    button: ::core::clone::Clone::clone(&self.button),
                }
            }
        }
        pub struct RawWheelEvent {
            /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
            pub delta_xy: Xy<f32>,
            pub mouse_xy: Xy<Px>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawWheelEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RawWheelEvent",
                    "delta_xy",
                    &self.delta_xy,
                    "mouse_xy",
                    &&self.mouse_xy,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawWheelEvent {
            #[inline]
            fn clone(&self) -> RawWheelEvent {
                RawWheelEvent {
                    delta_xy: ::core::clone::Clone::clone(&self.delta_xy),
                    mouse_xy: ::core::clone::Clone::clone(&self.mouse_xy),
                }
            }
        }
        pub struct RawKeyboardEvent {
            pub code: Code,
            pub pressing_codes: HashSet<Code>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawKeyboardEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RawKeyboardEvent",
                    "code",
                    &self.code,
                    "pressing_codes",
                    &&self.pressing_codes,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawKeyboardEvent {
            #[inline]
            fn clone(&self) -> RawKeyboardEvent {
                RawKeyboardEvent {
                    code: ::core::clone::Clone::clone(&self.code),
                    pressing_codes: ::core::clone::Clone::clone(&self.pressing_codes),
                }
            }
        }
        pub struct RawTextInputEvent {
            pub text: String,
            pub selection_direction: SelectionDirection,
            pub selection_start: usize,
            pub selection_end: usize,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawTextInputEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "RawTextInputEvent",
                    "text",
                    &self.text,
                    "selection_direction",
                    &self.selection_direction,
                    "selection_start",
                    &self.selection_start,
                    "selection_end",
                    &&self.selection_end,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawTextInputEvent {
            #[inline]
            fn clone(&self) -> RawTextInputEvent {
                RawTextInputEvent {
                    text: ::core::clone::Clone::clone(&self.text),
                    selection_direction: ::core::clone::Clone::clone(&self.selection_direction),
                    selection_start: ::core::clone::Clone::clone(&self.selection_start),
                    selection_end: ::core::clone::Clone::clone(&self.selection_end),
                }
            }
        }
        pub struct RawTextInputKeyDownEvent {
            pub text: String,
            pub selection_direction: SelectionDirection,
            pub selection_start: usize,
            pub selection_end: usize,
            pub code: Code,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RawTextInputKeyDownEvent {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "RawTextInputKeyDownEvent",
                    "text",
                    &self.text,
                    "selection_direction",
                    &self.selection_direction,
                    "selection_start",
                    &self.selection_start,
                    "selection_end",
                    &self.selection_end,
                    "code",
                    &&self.code,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RawTextInputKeyDownEvent {
            #[inline]
            fn clone(&self) -> RawTextInputKeyDownEvent {
                RawTextInputKeyDownEvent {
                    text: ::core::clone::Clone::clone(&self.text),
                    selection_direction: ::core::clone::Clone::clone(&self.selection_direction),
                    selection_start: ::core::clone::Clone::clone(&self.selection_start),
                    selection_end: ::core::clone::Clone::clone(&self.selection_end),
                    code: ::core::clone::Clone::clone(&self.code),
                }
            }
        }
        pub enum SelectionDirection {
            None = 0,
            Forward,
            Backward,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SelectionDirection {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        SelectionDirection::None => "None",
                        SelectionDirection::Forward => "Forward",
                        SelectionDirection::Backward => "Backward",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SelectionDirection {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SelectionDirection {
            #[inline]
            fn eq(&self, other: &SelectionDirection) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SelectionDirection {
            #[inline]
            fn clone(&self) -> SelectionDirection {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for SelectionDirection {}
    }
    use crate::*;
    pub use raw::*;
    use std::{collections::HashSet, fmt::Debug, sync::atomic::AtomicBool};
    pub enum Event<'a> {
        MouseDown { event: MouseEvent<'a> },
        MouseMove { event: MouseEvent<'a> },
        MouseUp { event: MouseEvent<'a> },
        Wheel { event: WheelEvent<'a> },
        KeyDown { event: KeyboardEvent<'a> },
        KeyUp { event: KeyboardEvent<'a> },
        Blur,
        VisibilityChange,
        ScreenResize { wh: Wh<IntPx> },
        ScreenRedraw,
        TextInput { event: &'a RawTextInputEvent },
        TextInputKeyDown { event: &'a RawTextInputKeyDownEvent },
        TextInputSelectionChange { event: &'a RawTextInputEvent },
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Event<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Event::MouseDown { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MouseDown",
                        "event",
                        &__self_0,
                    )
                }
                Event::MouseMove { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "MouseMove",
                        "event",
                        &__self_0,
                    )
                }
                Event::MouseUp { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "MouseUp", "event", &__self_0,
                    )
                }
                Event::Wheel { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "Wheel", "event", &__self_0,
                    )
                }
                Event::KeyDown { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "KeyDown", "event", &__self_0,
                    )
                }
                Event::KeyUp { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f, "KeyUp", "event", &__self_0,
                    )
                }
                Event::Blur => ::core::fmt::Formatter::write_str(f, "Blur"),
                Event::VisibilityChange => ::core::fmt::Formatter::write_str(f, "VisibilityChange"),
                Event::ScreenResize { wh: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ScreenResize",
                        "wh",
                        &__self_0,
                    )
                }
                Event::ScreenRedraw => ::core::fmt::Formatter::write_str(f, "ScreenRedraw"),
                Event::TextInput { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "TextInput",
                        "event",
                        &__self_0,
                    )
                }
                Event::TextInputKeyDown { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "TextInputKeyDown",
                        "event",
                        &__self_0,
                    )
                }
                Event::TextInputSelectionChange { event: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "TextInputSelectionChange",
                        "event",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub trait EventExt {
        fn stop_propagation(&self);
    }
    pub struct MouseEvent<'a> {
        pub local_xy: &'a dyn Fn() -> Xy<Px>,
        pub is_local_xy_in: &'a dyn Fn() -> bool,
        pub global_xy: Xy<Px>,
        pub pressing_buttons: &'a HashSet<MouseButton>,
        pub button: Option<MouseButton>,
        pub event_type: MouseEventType,
        pub is_stop_event_propagation: &'a AtomicBool,
    }
    impl Debug for MouseEvent<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MouseEvent")
                .field("global_xy", &self.global_xy)
                .field("pressing_buttons", &self.pressing_buttons)
                .field("button", &self.button)
                .field("event_type", &self.event_type)
                .field("is_stop_event_propagation", &self.is_stop_event_propagation)
                .finish()
        }
    }
    impl EventExt for MouseEvent<'_> {
        fn stop_propagation(&self) {
            self.is_stop_event_propagation
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
    impl MouseEvent<'_> {
        pub fn local_xy(&self) -> Xy<Px> {
            (self.local_xy)()
        }
        pub fn is_local_xy_in(&self) -> bool {
            (self.is_local_xy_in)()
        }
    }
    pub enum MouseEventType {
        Down,
        Up,
        Move,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MouseEventType {
        #[inline]
        fn clone(&self) -> MouseEventType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MouseEventType {}
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MouseEventType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MouseEventType {
        #[inline]
        fn eq(&self, other: &MouseEventType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MouseEventType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MouseEventType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MouseEventType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MouseEventType::Down => "Down",
                    MouseEventType::Up => "Up",
                    MouseEventType::Move => "Move",
                },
            )
        }
    }
    pub struct WheelEvent<'a> {
        /// NOTE: https://devblogs.microsoft.com/oldnewthing/20130123-00/?p=5473
        pub delta_xy: Xy<f32>,
        pub is_local_xy_in: &'a dyn Fn() -> bool,
        pub local_xy: &'a dyn Fn() -> Xy<Px>,
        pub is_stop_event_propagation: &'a AtomicBool,
    }
    impl Debug for WheelEvent<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("WheelEvent")
                .field("delta_xy", &self.delta_xy)
                .field("is_stop_event_propagation", &self.is_stop_event_propagation)
                .finish()
        }
    }
    impl EventExt for WheelEvent<'_> {
        fn stop_propagation(&self) {
            self.is_stop_event_propagation
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
    impl WheelEvent<'_> {
        pub fn local_xy(&self) -> Xy<Px> {
            (self.local_xy)()
        }
        pub fn is_local_xy_in(&self) -> bool {
            (self.is_local_xy_in)()
        }
    }
    pub struct KeyboardEvent<'a> {
        pub code: Code,
        pub pressing_codes: &'a HashSet<Code>,
        pub is_stop_event_propagation: &'a AtomicBool,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for KeyboardEvent<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "KeyboardEvent",
                "code",
                &self.code,
                "pressing_codes",
                &self.pressing_codes,
                "is_stop_event_propagation",
                &&self.is_stop_event_propagation,
            )
        }
    }
    impl EventExt for KeyboardEvent<'_> {
        fn stop_propagation(&self) {
            self.is_stop_event_propagation
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
mod paragraph {
    mod caret {
        use crate::*;
        pub struct Caret<'a> {
            pub line_index: usize,
            pub caret_index_in_line: usize,
            pub paragraph: &'a Paragraph,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Caret<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Caret",
                    "line_index",
                    &self.line_index,
                    "caret_index_in_line",
                    &self.caret_index_in_line,
                    "paragraph",
                    &&self.paragraph,
                )
            }
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for Caret<'a> {
            #[inline]
            fn clone(&self) -> Caret<'a> {
                Caret {
                    line_index: ::core::clone::Clone::clone(&self.line_index),
                    caret_index_in_line: ::core::clone::Clone::clone(&self.caret_index_in_line),
                    paragraph: ::core::clone::Clone::clone(&self.paragraph),
                }
            }
        }
        pub fn get_caret(selection_index: usize, paragraph: &Paragraph) -> Caret<'_> {
            let mut line_index = 0;
            let mut left_index = selection_index;
            for line in paragraph.iter_lines() {
                match left_index.cmp(&line.chars.len()) {
                    std::cmp::Ordering::Less => {
                        return Caret {
                            line_index,
                            caret_index_in_line: left_index,
                            paragraph,
                        };
                    }
                    std::cmp::Ordering::Equal => match line.new_line_by {
                        Some(new_line_by) => match new_line_by {
                            NewLineBy::Wrap => {
                                return Caret {
                                    line_index: line_index + 1,
                                    caret_index_in_line: 0,
                                    paragraph,
                                };
                            }
                            NewLineBy::LineFeed => {
                                return Caret {
                                    line_index,
                                    caret_index_in_line: left_index,
                                    paragraph,
                                };
                            }
                        },
                        None => {
                            return Caret {
                                line_index,
                                caret_index_in_line: left_index,
                                paragraph,
                            };
                        }
                    },
                    std::cmp::Ordering::Greater => {
                        left_index -= line.chars.len();
                        line_index += 1;
                        if let Some(NewLineBy::LineFeed) = line.new_line_by {
                            left_index -= 1;
                        }
                    }
                }
            }
            Caret {
                line_index,
                caret_index_in_line: left_index,
                paragraph,
            }
        }
        pub enum CaretKey {
            ArrowUp,
            ArrowDown,
            Home,
            End,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CaretKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        CaretKey::ArrowUp => "ArrowUp",
                        CaretKey::ArrowDown => "ArrowDown",
                        CaretKey::Home => "Home",
                        CaretKey::End => "End",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CaretKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CaretKey {
            #[inline]
            fn eq(&self, other: &CaretKey) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CaretKey {
            #[inline]
            fn clone(&self) -> CaretKey {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CaretKey {}
        impl Caret<'_> {
            pub fn get_caret_on_key(
                &self,
                key: CaretKey,
                text_align: TextAlign,
                container_width: Px,
            ) -> Caret<'_> {
                let (line_index, x) = match key {
                    CaretKey::ArrowUp => {
                        if self.line_index == 0 {
                            return Caret {
                                line_index: 0,
                                caret_index_in_line: 0,
                                paragraph: self.paragraph,
                            };
                        }
                        (self.line_index - 1, self.get_x(text_align, container_width))
                    }
                    CaretKey::ArrowDown => {
                        if self.is_at_bottom() {
                            return Caret {
                                line_index: self.line_index,
                                caret_index_in_line: self
                                    .paragraph
                                    .iter_chars()
                                    .nth(self.line_index)
                                    .unwrap()
                                    .len(),
                                paragraph: self.paragraph,
                            };
                        }
                        (self.line_index + 1, self.get_x(text_align, container_width))
                    }
                    CaretKey::Home => (self.line_index, 0.px()),
                    CaretKey::End => (
                        self.line_index,
                        self.paragraph.font.width(
                            &self.line_text(self.line_index).unwrap(),
                            &self.paragraph.paint,
                        ),
                    ),
                };
                let caret_index_on_direction =
                    self.get_caret_index_on_x(x, line_index, text_align, container_width);
                Caret {
                    line_index,
                    caret_index_in_line: caret_index_on_direction,
                    paragraph: self.paragraph,
                }
            }
            pub fn to_selection_index(&self) -> usize {
                let index_before_line = self.paragraph.char_index_before_line(self.line_index);
                index_before_line + self.caret_index_in_line
            }
            fn line_text(&self, index: usize) -> Option<String> {
                self.paragraph.iter_str().nth(index)
            }
            fn get_x(&self, text_align: TextAlign, container_width: Px) -> Px {
                let line_text = self.line_text(self.line_index).unwrap();
                let widths = self
                    .paragraph
                    .font
                    .widths(line_text.as_str(), &self.paragraph.paint);
                impl_get_x(
                    text_align,
                    &widths,
                    self.caret_index_in_line,
                    container_width,
                )
            }
            fn get_caret_index_on_x(
                &self,
                x: Px,
                line_index: usize,
                text_align: TextAlign,
                container_width: Px,
            ) -> usize {
                let line_text = self.line_text(line_index).unwrap();
                let widths = self
                    .paragraph
                    .font
                    .widths(line_text.as_str(), &self.paragraph.paint);
                let mut cloest_distance = x;
                let mut closest_caret_index = 0;
                for caret_index in 0..widths.len() {
                    let x_in_line = impl_get_x(text_align, &widths, caret_index, container_width);
                    let distance = (x - x_in_line).abs();
                    if distance < cloest_distance {
                        cloest_distance = distance;
                        closest_caret_index = caret_index;
                    }
                }
                closest_caret_index
            }
            pub fn is_at_bottom(&self) -> bool {
                let line_count = self.paragraph.iter_str().count();
                if line_count == 0 {
                    return true;
                }
                self.line_index == line_count - 1
            }
        }
        fn impl_get_x(
            text_align: TextAlign,
            widths: &[Px],
            caret_index_in_line: usize,
            container_width: Px,
        ) -> Px {
            match text_align {
                TextAlign::Left => widths.iter().take(caret_index_in_line).sum::<Px>(),
                TextAlign::Center => {
                    (container_width - widths.iter().sum::<Px>()) / 2.0
                        + widths.iter().take(caret_index_in_line).sum::<Px>()
                }
                TextAlign::Right => {
                    container_width - widths.iter().skip(caret_index_in_line).sum::<Px>()
                }
            }
        }
    }
    mod glyph {
        use crate::*;
        pub struct GlyphGroup {
            pub glyphs: Vec<Glyph>,
            pub font: Font,
            pub width: Px,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GlyphGroup {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GlyphGroup",
                    "glyphs",
                    &self.glyphs,
                    "font",
                    &self.font,
                    "width",
                    &&self.width,
                )
            }
        }
        pub struct Glyph {
            pub id: GlyphId,
            pub width: Px,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Glyph {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Glyph",
                    "id",
                    &self.id,
                    "width",
                    &&self.width,
                )
            }
        }
    }
    use crate::*;
    pub use caret::*;
    pub use glyph::*;
    use textwrap::{
        core::Fragment, word_splitters::split_words, wrap_algorithms::wrap_first_fit, *,
    };
    use unicode_segmentation::UnicodeSegmentation;
    pub struct Paragraph {
        pub vec: Vec<Line>,
        pub font: Font,
        pub paint: Paint,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Paragraph {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Paragraph",
                "vec",
                &self.vec,
                "font",
                &self.font,
                "paint",
                &&self.paint,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Paragraph {
        #[inline]
        fn clone(&self) -> Paragraph {
            Paragraph {
                vec: ::core::clone::Clone::clone(&self.vec),
                font: ::core::clone::Clone::clone(&self.font),
                paint: ::core::clone::Clone::clone(&self.paint),
            }
        }
    }
    impl Paragraph {
        pub fn new(text: &str, font: Font, paint: Paint, max_width: Option<Px>) -> Paragraph {
            let vec = if let Some(max_width) = max_width {
                let word_separator = WordSeparator::UnicodeBreakProperties;
                let word_splitter = WordSplitter::NoHyphenation;
                let mut vec: Vec<Line> = ::alloc::vec::Vec::new();
                for (index, line) in text.split('\n').enumerate() {
                    if index > 0 {
                        vec.last_mut().unwrap().new_line_by = Some(NewLineBy::LineFeed);
                    }
                    let words = word_separator.find_words(line);
                    let split_words = split_words(words, &word_splitter);
                    let namui_words = split_words
                        .flat_map(|word| {
                            NamuiWord::from_word(word, &font, &paint)
                                .break_apart(max_width, &font, &paint)
                        })
                        .collect::<Vec<_>>();
                    let line_lengths = [max_width.as_f32() as f64];
                    let wrapped_words = wrap_first_fit(&namui_words, &line_lengths);
                    for (index, words_in_line) in wrapped_words.iter().enumerate() {
                        if index > 0 {
                            vec.last_mut().unwrap().new_line_by = Some(NewLineBy::Wrap);
                        }
                        let mut line = "".to_string();
                        for word in words_in_line.iter() {
                            line += word.word;
                            line += word.whitespace;
                        }
                        vec.push(Line {
                            chars: line.chars().collect(),
                            new_line_by: None,
                        });
                    }
                }
                vec
            } else {
                let lines = text.lines();
                let mut vec: Vec<Line> = ::alloc::vec::Vec::new();
                for (index, line) in lines.enumerate() {
                    if index > 0 {
                        vec.last_mut().unwrap().new_line_by = Some(NewLineBy::LineFeed);
                    }
                    vec.push(Line {
                        chars: line.chars().collect(),
                        new_line_by: None,
                    });
                }
                vec
            };
            Self { vec, font, paint }
        }
        pub fn line_len(&self) -> usize {
            self.vec.len()
        }
        pub fn iter_str(&self) -> impl Iterator<Item = String> + '_ {
            self.vec.iter().map(|line| line.chars.iter().collect())
        }
        pub fn iter_chars(&self) -> impl Iterator<Item = &Vec<char>> {
            self.vec.iter().map(|line| &line.chars)
        }
        pub fn iter_lines(&self) -> impl Iterator<Item = &Line> {
            self.vec.iter()
        }
        pub fn get_line(&self, line_index: usize) -> Option<&Line> {
            self.vec.get(line_index)
        }
        pub fn char_index_before_line(&self, line_index: usize) -> usize {
            self.vec
                .iter()
                .take(line_index)
                .map(|line| {
                    line.chars.len()
                        + match line.new_line_by {
                            Some(new_line_by) => match new_line_by {
                                NewLineBy::Wrap => 0,
                                NewLineBy::LineFeed => 1,
                            },
                            None => 0,
                        }
                })
                .sum()
        }
        pub fn caret(&self, caret_index: usize) -> Caret<'_> {
            get_caret(caret_index, self)
        }
        pub fn selection_index_of_xy(
            &self,
            xy: Xy<Px>,
            font_size: IntPx,
            line_height_percent: Percent,
            text_baseline: TextBaseline,
            text_align: TextAlign,
        ) -> usize {
            let line_len = self.line_len();
            if line_len == 0 {
                return 0;
            }
            let line_index = {
                let line_height = font_size.into_px() * line_height_percent;
                let top_y = xy.y
                    + line_height
                        * match text_baseline {
                            TextBaseline::Top => 0.0,
                            TextBaseline::Middle => line_len as f32 / 2.0,
                            TextBaseline::Bottom => line_len as f32,
                        };
                let line_index = if top_y <= 0.px() {
                    0
                } else {
                    (top_y / line_height).floor() as usize
                };
                let line_max_index = line_len - 1;
                line_index.min(line_max_index)
            };
            let str_index_before_line = self.char_index_before_line(line_index);
            let line = self.iter_str().nth(line_index).unwrap();
            let glyph_widths = self.font.widths(line.as_ref(), &self.paint);
            let line_width = glyph_widths.iter().sum::<Px>();
            let aligned_x = match text_align {
                TextAlign::Left => xy.x,
                TextAlign::Center => xy.x + line_width / 2.0,
                TextAlign::Right => xy.x + line_width,
            };
            let mut left = px(0.0);
            let index = glyph_widths
                .iter()
                .position(|width| {
                    let center = left + width / 2.0;
                    if aligned_x < center {
                        return true;
                    }
                    left += *width;
                    false
                })
                .unwrap_or(line.chars().count());
            str_index_before_line + index
        }
    }
    pub struct NamuiWord<'a> {
        word: &'a str,
        width: Px,
        whitespace: &'a str,
        whitespace_width: Px,
        penalty: &'a str,
        penalty_width: Px,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for NamuiWord<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "word",
                "width",
                "whitespace",
                "whitespace_width",
                "penalty",
                "penalty_width",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.word,
                &self.width,
                &self.whitespace,
                &self.whitespace_width,
                &self.penalty,
                &&self.penalty_width,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "NamuiWord", names, values)
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::Copy for NamuiWord<'a> {}
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for NamuiWord<'a> {
        #[inline]
        fn clone(&self) -> NamuiWord<'a> {
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<Px>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            let _: ::core::clone::AssertParamIsClone<&'a str>;
            *self
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for NamuiWord<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for NamuiWord<'a> {
        #[inline]
        fn eq(&self, other: &NamuiWord<'a>) -> bool {
            self.word == other.word
                && self.width == other.width
                && self.whitespace == other.whitespace
                && self.whitespace_width == other.whitespace_width
                && self.penalty == other.penalty
                && self.penalty_width == other.penalty_width
        }
    }
    impl Fragment for NamuiWord<'_> {
        fn width(&self) -> f64 {
            self.width.as_f32() as f64
        }
        fn whitespace_width(&self) -> f64 {
            self.whitespace_width.as_f32() as f64
        }
        fn penalty_width(&self) -> f64 {
            self.penalty_width.as_f32() as f64
        }
    }
    impl<'a> NamuiWord<'a> {
        fn from_word(word: core::Word<'a>, font: &Font, paint: &Paint) -> Self {
            Self {
                word: word.word,
                width: font.width(word.word, paint),
                whitespace: word.whitespace,
                whitespace_width: font.width(word.whitespace, paint),
                penalty: word.penalty,
                penalty_width: font.width(word.penalty, paint),
            }
        }
        fn break_apart(self, max_width: Px, font: &Font, paint: &Paint) -> Vec<NamuiWord<'a>> {
            if self.width <= max_width {
                return <[_]>::into_vec(::alloc::boxed::box_new([self]));
            }
            let mut start = 0;
            let mut words = Vec::new();
            for (idx, grapheme) in self.word.grapheme_indices(true) {
                let with_grapheme = &self.word[start..idx + grapheme.len()];
                let without_grapheme = &self.word[start..idx];
                if idx > 0 && font.width(with_grapheme, paint) > max_width {
                    let natural_width = font.width(without_grapheme, paint);
                    words.push(NamuiWord {
                        word: without_grapheme,
                        width: max_width.max(natural_width),
                        whitespace: "",
                        whitespace_width: 0.px(),
                        penalty: "",
                        penalty_width: 0.px(),
                    });
                    start = idx;
                }
            }
            words.push(NamuiWord {
                word: &self.word[start..],
                width: font.width(&self.word[start..], paint),
                whitespace: self.whitespace,
                whitespace_width: self.whitespace_width,
                penalty: self.penalty,
                penalty_width: self.penalty_width,
            });
            words
        }
    }
    pub fn get_left_in_align(x: Px, align: TextAlign, width: Px) -> Px {
        match align {
            TextAlign::Left => x,
            TextAlign::Center => x - width / 2.0,
            TextAlign::Right => x - width,
        }
    }
    pub fn get_bottom_of_baseline(baseline: TextBaseline, font_metrics: FontMetrics) -> Px {
        match baseline {
            TextBaseline::Top => -font_metrics.ascent - font_metrics.descent,
            TextBaseline::Middle => (-font_metrics.ascent - font_metrics.descent) / 2.0,
            TextBaseline::Bottom => -font_metrics.descent,
        }
    }
    pub fn get_multiline_y_baseline_offset(
        baseline: TextBaseline,
        line_height: Px,
        paragraph_len: usize,
    ) -> Px {
        match baseline {
            TextBaseline::Top => 0.px(),
            TextBaseline::Middle => -line_height * 0.5 * (paragraph_len - 1),
            TextBaseline::Bottom => -line_height * (paragraph_len - 1),
        }
    }
    pub enum NewLineBy {
        Wrap,
        /// `\n`
        LineFeed,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NewLineBy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    NewLineBy::Wrap => "Wrap",
                    NewLineBy::LineFeed => "LineFeed",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NewLineBy {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NewLineBy {
        #[inline]
        fn eq(&self, other: &NewLineBy) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NewLineBy {
        #[inline]
        fn clone(&self) -> NewLineBy {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for NewLineBy {}
    pub struct Line {
        /// Should not have `\n`
        pub chars: Vec<char>,
        pub new_line_by: Option<NewLineBy>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Line {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Line",
                "chars",
                &self.chars,
                "new_line_by",
                &&self.new_line_by,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Line {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Line {
        #[inline]
        fn eq(&self, other: &Line) -> bool {
            self.chars == other.chars && self.new_line_by == other.new_line_by
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Line {
        #[inline]
        fn clone(&self) -> Line {
            Line {
                chars: ::core::clone::Clone::clone(&self.chars),
                new_line_by: ::core::clone::Clone::clone(&self.new_line_by),
            }
        }
    }
}
mod rendering_tree {
    mod special {
        pub mod absolute {
            use super::*;
            pub struct AbsoluteNode {
                pub x: Px,
                pub y: Px,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for AbsoluteNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "AbsoluteNode",
                        "x",
                        &self.x,
                        "y",
                        &self.y,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for AbsoluteNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for AbsoluteNode {
                #[inline]
                fn eq(&self, other: &AbsoluteNode) -> bool {
                    self.x == other.x
                        && self.y == other.y
                        && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for AbsoluteNode {
                #[inline]
                fn clone(&self) -> AbsoluteNode {
                    AbsoluteNode {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for AbsoluteNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.x, state);
                    ::core::hash::Hash::hash(&self.y, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for AbsoluteNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Px>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for AbsoluteNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.x, encoder)?;
                    bincode::Encode::encode(&self.y, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for AbsoluteNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        x: bincode::Decode::decode(decoder)?,
                        y: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for AbsoluteNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("x");
                    let field_bytes = Serialize::serialize(&self.x);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("y");
                    let field_bytes = Serialize::serialize(&self.y);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for AbsoluteNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("x")?;
                    let x = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("y")?;
                    let y = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        x,
                        y,
                        rendering_tree,
                    })
                }
            }
            impl AbsoluteNode {
                pub fn get_matrix(&self) -> TransformMatrix {
                    TransformMatrix::from_translate(self.x.as_f32(), self.y.as_f32())
                }
            }
            pub fn absolute(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::Absolute(AbsoluteNode {
                    x,
                    y,
                    rendering_tree: rendering_tree.into(),
                }))
            }
        }
        pub mod clip {
            use super::*;
            pub struct ClipNode {
                pub path: Path,
                pub clip_op: ClipOp,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ClipNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ClipNode",
                        "path",
                        &self.path,
                        "clip_op",
                        &self.clip_op,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ClipNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ClipNode {
                #[inline]
                fn eq(&self, other: &ClipNode) -> bool {
                    self.path == other.path
                        && self.clip_op == other.clip_op
                        && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ClipNode {
                #[inline]
                fn clone(&self) -> ClipNode {
                    ClipNode {
                        path: ::core::clone::Clone::clone(&self.path),
                        clip_op: ::core::clone::Clone::clone(&self.clip_op),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for ClipNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.path, state);
                    ::core::hash::Hash::hash(&self.clip_op, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ClipNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Path>;
                    let _: ::core::cmp::AssertParamIsEq<ClipOp>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for ClipNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.path, encoder)?;
                    bincode::Encode::encode(&self.clip_op, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for ClipNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        path: bincode::Decode::decode(decoder)?,
                        clip_op: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for ClipNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("path");
                    let field_bytes = Serialize::serialize(&self.path);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("clip_op");
                    let field_bytes = Serialize::serialize(&self.clip_op);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for ClipNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("path")?;
                    let path = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("clip_op")?;
                    let clip_op = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        path,
                        clip_op,
                        rendering_tree,
                    })
                }
            }
        }
        pub mod mouse_cursor {
            use super::*;
            use std::collections::BTreeMap;
            pub struct MouseCursorNode {
                pub cursor: Box<MouseCursor>,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MouseCursorNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "MouseCursorNode",
                        "cursor",
                        &self.cursor,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for MouseCursorNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for MouseCursorNode {
                #[inline]
                fn eq(&self, other: &MouseCursorNode) -> bool {
                    self.cursor == other.cursor && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MouseCursorNode {
                #[inline]
                fn clone(&self) -> MouseCursorNode {
                    MouseCursorNode {
                        cursor: ::core::clone::Clone::clone(&self.cursor),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for MouseCursorNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.cursor, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for MouseCursorNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<MouseCursor>>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for MouseCursorNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.cursor, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for MouseCursorNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        cursor: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for MouseCursorNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("cursor");
                    let field_bytes = Serialize::serialize(&self.cursor);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for MouseCursorNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("cursor")?;
                    let cursor = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        cursor,
                        rendering_tree,
                    })
                }
            }
            pub enum MouseCursor {
                Standard(StandardCursor),
                Custom(RenderingTree),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for MouseCursor {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        MouseCursor::Standard(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Standard", &__self_0,
                            )
                        }
                        MouseCursor::Custom(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f, "Custom", &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for MouseCursor {
                #[inline]
                fn clone(&self) -> MouseCursor {
                    match self {
                        MouseCursor::Standard(__self_0) => {
                            MouseCursor::Standard(::core::clone::Clone::clone(__self_0))
                        }
                        MouseCursor::Custom(__self_0) => {
                            MouseCursor::Custom(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for MouseCursor {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for MouseCursor {
                #[inline]
                fn eq(&self, other: &MouseCursor) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (MouseCursor::Standard(__self_0), MouseCursor::Standard(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            (MouseCursor::Custom(__self_0), MouseCursor::Custom(__arg1_0)) => {
                                __self_0 == __arg1_0
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for MouseCursor {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<StandardCursor>;
                    let _: ::core::cmp::AssertParamIsEq<RenderingTree>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for MouseCursor {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state);
                    match self {
                        MouseCursor::Standard(__self_0) => {
                            ::core::hash::Hash::hash(__self_0, state)
                        }
                        MouseCursor::Custom(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    }
                }
            }
            impl bincode::Encode for MouseCursor {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    match self {
                        Self::Standard(field0) => {
                            bincode::Encode::encode(&0u32, encoder)?;
                            bincode::Encode::encode(field0, encoder)?;
                        }
                        Self::Custom(field0) => {
                            bincode::Encode::encode(&1u32, encoder)?;
                            bincode::Encode::encode(field0, encoder)?;
                        }
                    }
                    Ok(())
                }
            }
            impl bincode::Decode<()> for MouseCursor {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    let discriminant: u32 = bincode::Decode::decode(decoder)?;
                    match discriminant {
                        0u32 => Ok(Self::Standard(bincode::Decode::decode(decoder)?)),
                        1u32 => Ok(Self::Custom(bincode::Decode::decode(decoder)?)),
                        _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                            type_name: core::any::type_name::<Self>(),
                            allowed: &bincode::error::AllowedEnumVariants::Range {
                                min: 0,
                                max: 1u32,
                            },
                            found: discriminant,
                        }),
                    }
                }
            }
            impl Serialize for MouseCursor {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    match self {
                        Self::Standard { field0 } => {
                            buffer.write_string("Standard");
                            let field_bytes = Serialize::serialize(field0);
                            buffer.put_slice(&field_bytes);
                        }
                        Self::Custom { field0 } => {
                            buffer.write_string("Custom");
                            let field_bytes = Serialize::serialize(field0);
                            buffer.put_slice(&field_bytes);
                        }
                    }
                    buffer
                }
            }
            impl Deserialize for MouseCursor {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    use bytes::Buf;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let variant_name = buf.read_string();
                    match variant_name.as_ref() {
                        "Standard" => {
                            let field0 = { Deserialize::deserialize(buf)? };
                            Ok(Self::Standard(field0))
                        }
                        "Custom" => {
                            let field0 = { Deserialize::deserialize(buf)? };
                            Ok(Self::Custom(field0))
                        }
                        _ => Err(DeserializeError::InvalidEnumVariant {
                            expected: std::any::type_name::<Self>().to_string(),
                            actual: variant_name,
                        }),
                    }
                }
            }
            pub enum StandardCursor {
                #[default]
                Default,
                Pointer,
                Wait,
                Progress,
                Help,
                Text,
                VerticalText,
                NResize,
                SResize,
                EResize,
                WResize,
                NeResize,
                NwResize,
                SeResize,
                SwResize,
                EwResize,
                NsResize,
                NeswResize,
                NwseResize,
                ColResize,
                RowResize,
                Move,
                AllScroll,
                Grab,
                Copy,
                Alias,
                NoDrop,
                NotAllowed,
                Crosshair,
                Cell,
                ContextMenu,
                ZoomIn,
                ZoomOut,
                ColorPicker,
                Pencil,
                UpArrow,
                DownArrow,
                LeftArrow,
                RightArrow,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for StandardCursor {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            StandardCursor::Default => "Default",
                            StandardCursor::Pointer => "Pointer",
                            StandardCursor::Wait => "Wait",
                            StandardCursor::Progress => "Progress",
                            StandardCursor::Help => "Help",
                            StandardCursor::Text => "Text",
                            StandardCursor::VerticalText => "VerticalText",
                            StandardCursor::NResize => "NResize",
                            StandardCursor::SResize => "SResize",
                            StandardCursor::EResize => "EResize",
                            StandardCursor::WResize => "WResize",
                            StandardCursor::NeResize => "NeResize",
                            StandardCursor::NwResize => "NwResize",
                            StandardCursor::SeResize => "SeResize",
                            StandardCursor::SwResize => "SwResize",
                            StandardCursor::EwResize => "EwResize",
                            StandardCursor::NsResize => "NsResize",
                            StandardCursor::NeswResize => "NeswResize",
                            StandardCursor::NwseResize => "NwseResize",
                            StandardCursor::ColResize => "ColResize",
                            StandardCursor::RowResize => "RowResize",
                            StandardCursor::Move => "Move",
                            StandardCursor::AllScroll => "AllScroll",
                            StandardCursor::Grab => "Grab",
                            StandardCursor::Copy => "Copy",
                            StandardCursor::Alias => "Alias",
                            StandardCursor::NoDrop => "NoDrop",
                            StandardCursor::NotAllowed => "NotAllowed",
                            StandardCursor::Crosshair => "Crosshair",
                            StandardCursor::Cell => "Cell",
                            StandardCursor::ContextMenu => "ContextMenu",
                            StandardCursor::ZoomIn => "ZoomIn",
                            StandardCursor::ZoomOut => "ZoomOut",
                            StandardCursor::ColorPicker => "ColorPicker",
                            StandardCursor::Pencil => "Pencil",
                            StandardCursor::UpArrow => "UpArrow",
                            StandardCursor::DownArrow => "DownArrow",
                            StandardCursor::LeftArrow => "LeftArrow",
                            StandardCursor::RightArrow => "RightArrow",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StandardCursor {
                #[inline]
                fn clone(&self) -> StandardCursor {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for StandardCursor {}
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for StandardCursor {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for StandardCursor {
                #[inline]
                fn eq(&self, other: &StandardCursor) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for StandardCursor {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::hash::Hash for StandardCursor {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for StandardCursor {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &StandardCursor,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for StandardCursor {
                #[inline]
                fn cmp(&self, other: &StandardCursor) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for StandardCursor {
                #[inline]
                fn default() -> StandardCursor {
                    Self::Default
                }
            }
            impl bincode::Encode for StandardCursor {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    match self {
                        Self::Default => {
                            bincode::Encode::encode(&0u32, encoder)?;
                        }
                        Self::Pointer => {
                            bincode::Encode::encode(&1u32, encoder)?;
                        }
                        Self::Wait => {
                            bincode::Encode::encode(&2u32, encoder)?;
                        }
                        Self::Progress => {
                            bincode::Encode::encode(&3u32, encoder)?;
                        }
                        Self::Help => {
                            bincode::Encode::encode(&4u32, encoder)?;
                        }
                        Self::Text => {
                            bincode::Encode::encode(&5u32, encoder)?;
                        }
                        Self::VerticalText => {
                            bincode::Encode::encode(&6u32, encoder)?;
                        }
                        Self::NResize => {
                            bincode::Encode::encode(&7u32, encoder)?;
                        }
                        Self::SResize => {
                            bincode::Encode::encode(&8u32, encoder)?;
                        }
                        Self::EResize => {
                            bincode::Encode::encode(&9u32, encoder)?;
                        }
                        Self::WResize => {
                            bincode::Encode::encode(&10u32, encoder)?;
                        }
                        Self::NeResize => {
                            bincode::Encode::encode(&11u32, encoder)?;
                        }
                        Self::NwResize => {
                            bincode::Encode::encode(&12u32, encoder)?;
                        }
                        Self::SeResize => {
                            bincode::Encode::encode(&13u32, encoder)?;
                        }
                        Self::SwResize => {
                            bincode::Encode::encode(&14u32, encoder)?;
                        }
                        Self::EwResize => {
                            bincode::Encode::encode(&15u32, encoder)?;
                        }
                        Self::NsResize => {
                            bincode::Encode::encode(&16u32, encoder)?;
                        }
                        Self::NeswResize => {
                            bincode::Encode::encode(&17u32, encoder)?;
                        }
                        Self::NwseResize => {
                            bincode::Encode::encode(&18u32, encoder)?;
                        }
                        Self::ColResize => {
                            bincode::Encode::encode(&19u32, encoder)?;
                        }
                        Self::RowResize => {
                            bincode::Encode::encode(&20u32, encoder)?;
                        }
                        Self::Move => {
                            bincode::Encode::encode(&21u32, encoder)?;
                        }
                        Self::AllScroll => {
                            bincode::Encode::encode(&22u32, encoder)?;
                        }
                        Self::Grab => {
                            bincode::Encode::encode(&23u32, encoder)?;
                        }
                        Self::Copy => {
                            bincode::Encode::encode(&24u32, encoder)?;
                        }
                        Self::Alias => {
                            bincode::Encode::encode(&25u32, encoder)?;
                        }
                        Self::NoDrop => {
                            bincode::Encode::encode(&26u32, encoder)?;
                        }
                        Self::NotAllowed => {
                            bincode::Encode::encode(&27u32, encoder)?;
                        }
                        Self::Crosshair => {
                            bincode::Encode::encode(&28u32, encoder)?;
                        }
                        Self::Cell => {
                            bincode::Encode::encode(&29u32, encoder)?;
                        }
                        Self::ContextMenu => {
                            bincode::Encode::encode(&30u32, encoder)?;
                        }
                        Self::ZoomIn => {
                            bincode::Encode::encode(&31u32, encoder)?;
                        }
                        Self::ZoomOut => {
                            bincode::Encode::encode(&32u32, encoder)?;
                        }
                        Self::ColorPicker => {
                            bincode::Encode::encode(&33u32, encoder)?;
                        }
                        Self::Pencil => {
                            bincode::Encode::encode(&34u32, encoder)?;
                        }
                        Self::UpArrow => {
                            bincode::Encode::encode(&35u32, encoder)?;
                        }
                        Self::DownArrow => {
                            bincode::Encode::encode(&36u32, encoder)?;
                        }
                        Self::LeftArrow => {
                            bincode::Encode::encode(&37u32, encoder)?;
                        }
                        Self::RightArrow => {
                            bincode::Encode::encode(&38u32, encoder)?;
                        }
                    }
                    Ok(())
                }
            }
            impl bincode::Decode<()> for StandardCursor {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    let discriminant: u32 = bincode::Decode::decode(decoder)?;
                    match discriminant {
                        0u32 => Ok(Self::Default),
                        1u32 => Ok(Self::Pointer),
                        2u32 => Ok(Self::Wait),
                        3u32 => Ok(Self::Progress),
                        4u32 => Ok(Self::Help),
                        5u32 => Ok(Self::Text),
                        6u32 => Ok(Self::VerticalText),
                        7u32 => Ok(Self::NResize),
                        8u32 => Ok(Self::SResize),
                        9u32 => Ok(Self::EResize),
                        10u32 => Ok(Self::WResize),
                        11u32 => Ok(Self::NeResize),
                        12u32 => Ok(Self::NwResize),
                        13u32 => Ok(Self::SeResize),
                        14u32 => Ok(Self::SwResize),
                        15u32 => Ok(Self::EwResize),
                        16u32 => Ok(Self::NsResize),
                        17u32 => Ok(Self::NeswResize),
                        18u32 => Ok(Self::NwseResize),
                        19u32 => Ok(Self::ColResize),
                        20u32 => Ok(Self::RowResize),
                        21u32 => Ok(Self::Move),
                        22u32 => Ok(Self::AllScroll),
                        23u32 => Ok(Self::Grab),
                        24u32 => Ok(Self::Copy),
                        25u32 => Ok(Self::Alias),
                        26u32 => Ok(Self::NoDrop),
                        27u32 => Ok(Self::NotAllowed),
                        28u32 => Ok(Self::Crosshair),
                        29u32 => Ok(Self::Cell),
                        30u32 => Ok(Self::ContextMenu),
                        31u32 => Ok(Self::ZoomIn),
                        32u32 => Ok(Self::ZoomOut),
                        33u32 => Ok(Self::ColorPicker),
                        34u32 => Ok(Self::Pencil),
                        35u32 => Ok(Self::UpArrow),
                        36u32 => Ok(Self::DownArrow),
                        37u32 => Ok(Self::LeftArrow),
                        38u32 => Ok(Self::RightArrow),
                        _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                            type_name: core::any::type_name::<Self>(),
                            allowed: &bincode::error::AllowedEnumVariants::Range {
                                min: 0,
                                max: 38u32,
                            },
                            found: discriminant,
                        }),
                    }
                }
            }
            impl Serialize for StandardCursor {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    match self {
                        Self::Default {} => {
                            buffer.write_string("Default");
                        }
                        Self::Pointer {} => {
                            buffer.write_string("Pointer");
                        }
                        Self::Wait {} => {
                            buffer.write_string("Wait");
                        }
                        Self::Progress {} => {
                            buffer.write_string("Progress");
                        }
                        Self::Help {} => {
                            buffer.write_string("Help");
                        }
                        Self::Text {} => {
                            buffer.write_string("Text");
                        }
                        Self::VerticalText {} => {
                            buffer.write_string("VerticalText");
                        }
                        Self::NResize {} => {
                            buffer.write_string("NResize");
                        }
                        Self::SResize {} => {
                            buffer.write_string("SResize");
                        }
                        Self::EResize {} => {
                            buffer.write_string("EResize");
                        }
                        Self::WResize {} => {
                            buffer.write_string("WResize");
                        }
                        Self::NeResize {} => {
                            buffer.write_string("NeResize");
                        }
                        Self::NwResize {} => {
                            buffer.write_string("NwResize");
                        }
                        Self::SeResize {} => {
                            buffer.write_string("SeResize");
                        }
                        Self::SwResize {} => {
                            buffer.write_string("SwResize");
                        }
                        Self::EwResize {} => {
                            buffer.write_string("EwResize");
                        }
                        Self::NsResize {} => {
                            buffer.write_string("NsResize");
                        }
                        Self::NeswResize {} => {
                            buffer.write_string("NeswResize");
                        }
                        Self::NwseResize {} => {
                            buffer.write_string("NwseResize");
                        }
                        Self::ColResize {} => {
                            buffer.write_string("ColResize");
                        }
                        Self::RowResize {} => {
                            buffer.write_string("RowResize");
                        }
                        Self::Move {} => {
                            buffer.write_string("Move");
                        }
                        Self::AllScroll {} => {
                            buffer.write_string("AllScroll");
                        }
                        Self::Grab {} => {
                            buffer.write_string("Grab");
                        }
                        Self::Copy {} => {
                            buffer.write_string("Copy");
                        }
                        Self::Alias {} => {
                            buffer.write_string("Alias");
                        }
                        Self::NoDrop {} => {
                            buffer.write_string("NoDrop");
                        }
                        Self::NotAllowed {} => {
                            buffer.write_string("NotAllowed");
                        }
                        Self::Crosshair {} => {
                            buffer.write_string("Crosshair");
                        }
                        Self::Cell {} => {
                            buffer.write_string("Cell");
                        }
                        Self::ContextMenu {} => {
                            buffer.write_string("ContextMenu");
                        }
                        Self::ZoomIn {} => {
                            buffer.write_string("ZoomIn");
                        }
                        Self::ZoomOut {} => {
                            buffer.write_string("ZoomOut");
                        }
                        Self::ColorPicker {} => {
                            buffer.write_string("ColorPicker");
                        }
                        Self::Pencil {} => {
                            buffer.write_string("Pencil");
                        }
                        Self::UpArrow {} => {
                            buffer.write_string("UpArrow");
                        }
                        Self::DownArrow {} => {
                            buffer.write_string("DownArrow");
                        }
                        Self::LeftArrow {} => {
                            buffer.write_string("LeftArrow");
                        }
                        Self::RightArrow {} => {
                            buffer.write_string("RightArrow");
                        }
                    }
                    buffer
                }
            }
            impl Deserialize for StandardCursor {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    use bytes::Buf;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let variant_name = buf.read_string();
                    match variant_name.as_ref() {
                        "Default" => Ok(Self::Default),
                        "Pointer" => Ok(Self::Pointer),
                        "Wait" => Ok(Self::Wait),
                        "Progress" => Ok(Self::Progress),
                        "Help" => Ok(Self::Help),
                        "Text" => Ok(Self::Text),
                        "VerticalText" => Ok(Self::VerticalText),
                        "NResize" => Ok(Self::NResize),
                        "SResize" => Ok(Self::SResize),
                        "EResize" => Ok(Self::EResize),
                        "WResize" => Ok(Self::WResize),
                        "NeResize" => Ok(Self::NeResize),
                        "NwResize" => Ok(Self::NwResize),
                        "SeResize" => Ok(Self::SeResize),
                        "SwResize" => Ok(Self::SwResize),
                        "EwResize" => Ok(Self::EwResize),
                        "NsResize" => Ok(Self::NsResize),
                        "NeswResize" => Ok(Self::NeswResize),
                        "NwseResize" => Ok(Self::NwseResize),
                        "ColResize" => Ok(Self::ColResize),
                        "RowResize" => Ok(Self::RowResize),
                        "Move" => Ok(Self::Move),
                        "AllScroll" => Ok(Self::AllScroll),
                        "Grab" => Ok(Self::Grab),
                        "Copy" => Ok(Self::Copy),
                        "Alias" => Ok(Self::Alias),
                        "NoDrop" => Ok(Self::NoDrop),
                        "NotAllowed" => Ok(Self::NotAllowed),
                        "Crosshair" => Ok(Self::Crosshair),
                        "Cell" => Ok(Self::Cell),
                        "ContextMenu" => Ok(Self::ContextMenu),
                        "ZoomIn" => Ok(Self::ZoomIn),
                        "ZoomOut" => Ok(Self::ZoomOut),
                        "ColorPicker" => Ok(Self::ColorPicker),
                        "Pencil" => Ok(Self::Pencil),
                        "UpArrow" => Ok(Self::UpArrow),
                        "DownArrow" => Ok(Self::DownArrow),
                        "LeftArrow" => Ok(Self::LeftArrow),
                        "RightArrow" => Ok(Self::RightArrow),
                        _ => Err(DeserializeError::InvalidEnumVariant {
                            expected: std::any::type_name::<Self>().to_string(),
                            actual: variant_name,
                        }),
                    }
                }
            }
            impl StandardCursor {
                pub fn from_css_cursor_value(value: &str) -> Option<Self> {
                    Some(match value {
                        "default" => StandardCursor::Default,
                        "pointer" => StandardCursor::Pointer,
                        "wait" => StandardCursor::Wait,
                        "progress" => StandardCursor::Progress,
                        "help" => StandardCursor::Help,
                        "text" => StandardCursor::Text,
                        "vertical-text" => StandardCursor::VerticalText,
                        "n-resize" => StandardCursor::NResize,
                        "s-resize" => StandardCursor::SResize,
                        "e-resize" => StandardCursor::EResize,
                        "w-resize" => StandardCursor::WResize,
                        "ne-resize" => StandardCursor::NeResize,
                        "nw-resize" => StandardCursor::NwResize,
                        "se-resize" => StandardCursor::SeResize,
                        "sw-resize" => StandardCursor::SwResize,
                        "ew-resize" => StandardCursor::EwResize,
                        "ns-resize" => StandardCursor::NsResize,
                        "nesw-resize" => StandardCursor::NeswResize,
                        "nwse-resize" => StandardCursor::NwseResize,
                        "col-resize" => StandardCursor::ColResize,
                        "row-resize" => StandardCursor::RowResize,
                        "move" => StandardCursor::Move,
                        "all-scroll" => StandardCursor::AllScroll,
                        "grab" => StandardCursor::Grab,
                        "copy" => StandardCursor::Copy,
                        "alias" => StandardCursor::Alias,
                        "no-drop" => StandardCursor::NoDrop,
                        "not-allowed" => StandardCursor::NotAllowed,
                        "crosshair" => StandardCursor::Crosshair,
                        "cell" => StandardCursor::Cell,
                        "context-menu" => StandardCursor::ContextMenu,
                        "zoom-in" => StandardCursor::ZoomIn,
                        "zoom-out" => StandardCursor::ZoomOut,
                        "color-picker" => StandardCursor::ColorPicker,
                        "pencil" => StandardCursor::Pencil,
                        "up-arrow" => StandardCursor::UpArrow,
                        "down-arrow" => StandardCursor::DownArrow,
                        "left-arrow" => StandardCursor::LeftArrow,
                        "right-arrow" => StandardCursor::RightArrow,
                        _ => return None,
                    })
                }
            }
            /// Metadata format:
            /// - first line: number of columns, number of rows, cursor width, cursor height
            /// - each subsequent line: cursor name, hotspot x, hotspot y, frame count (optional), frame duration (optional)
            ///
            /// All cursor name is for CSS cursor value.
            ///
            /// ```text
            /// {columns} {rows} {cursor width} {cursor height}
            /// {cursor name} {hotspot x} {hotspot y} {frame count} {frame duration} // for animated cursors
            /// {cursor name} {hotspot x} {hotspot y} // for static cursors
            /// ...
            /// ```
            pub struct StandardCursorSpriteSet {
                pub sheet: Image,
                pub columns: usize,
                pub rows: usize,
                pub cursor_wh: Wh<Px>,
                pub sprites: BTreeMap<StandardCursor, CursorSprite>,
            }
            impl bincode::Encode for StandardCursorSpriteSet {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.sheet, encoder)?;
                    bincode::Encode::encode(&self.columns, encoder)?;
                    bincode::Encode::encode(&self.rows, encoder)?;
                    bincode::Encode::encode(&self.cursor_wh, encoder)?;
                    bincode::Encode::encode(&self.sprites, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for StandardCursorSpriteSet {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        sheet: bincode::Decode::decode(decoder)?,
                        columns: bincode::Decode::decode(decoder)?,
                        rows: bincode::Decode::decode(decoder)?,
                        cursor_wh: bincode::Decode::decode(decoder)?,
                        sprites: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for StandardCursorSpriteSet {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("sheet");
                    let field_bytes = Serialize::serialize(&self.sheet);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("columns");
                    let field_bytes = Serialize::serialize(&self.columns);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rows");
                    let field_bytes = Serialize::serialize(&self.rows);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("cursor_wh");
                    let field_bytes = Serialize::serialize(&self.cursor_wh);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("sprites");
                    let field_bytes = Serialize::serialize(&self.sprites);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for StandardCursorSpriteSet {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("sheet")?;
                    let sheet = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("columns")?;
                    let columns = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rows")?;
                    let rows = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("cursor_wh")?;
                    let cursor_wh = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("sprites")?;
                    let sprites = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        sheet,
                        columns,
                        rows,
                        cursor_wh,
                        sprites,
                    })
                }
            }
            impl StandardCursorSpriteSet {
                pub fn parse(sheet: Image, metadata_text: &str) -> anyhow::Result<Self> {
                    let lines = &mut metadata_text.lines();
                    let line = lines.next().ok_or_else(|| {
                        ::anyhow::__private::must_use({
                            let error =
                                ::anyhow::__private::format_err(format_args!("Empty metadata"));
                            error
                        })
                    })?;
                    let parts = &mut line.split_whitespace();
                    let columns = parse_parts(parts, "columns")?;
                    let rows = parse_parts(parts, "rows")?;
                    let cursor_wh = Wh::new(
                        parse_parts::<i32>(parts, "cursor width")?.px(),
                        parse_parts::<i32>(parts, "cursor height")?.px(),
                    );
                    let mut sprites = BTreeMap::new();
                    let mut index = 0;
                    for line in lines {
                        if line.is_empty() {
                            continue;
                        }
                        let parts = &mut line.split_whitespace().peekable();
                        let name = parts.next().ok_or_else(|| {
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(format_args!(
                                    "Missing cursor name"
                                ));
                                error
                            })
                        })?;
                        let Some(standard_cursor) = StandardCursor::from_css_cursor_value(name)
                        else {
                            {
                                ::std::io::_print(format_args!(
                                    "Unknown cursor name: {0} at line {1}, skipping.\n",
                                    name, index,
                                ));
                            };
                            continue;
                        };
                        let hotspot_xy = Xy::new(
                            parse_parts::<i32>(parts, "hotspot x")?.px(),
                            parse_parts::<i32>(parts, "hotspot y")?.px(),
                        );
                        let animation_frame_count_duration = if parts.peek().is_some() {
                            Some((
                                parse_parts::<usize>(parts, "frame count")?,
                                parse_parts::<i32>(parts, "frame duration")?.ms(),
                            ))
                        } else {
                            None
                        };
                        sprites.insert(
                            standard_cursor,
                            if let Some((frame_count, frame_duration)) =
                                animation_frame_count_duration
                            {
                                CursorSprite::Animated {
                                    start_index: index,
                                    hotspot_xy,
                                    frame_count,
                                    frame_duration,
                                }
                            } else {
                                CursorSprite::Static { index, hotspot_xy }
                            },
                        );
                        match animation_frame_count_duration {
                            Some((frame_count, _)) => {
                                index += frame_count;
                            }
                            None => {
                                index += 1;
                            }
                        }
                    }
                    Ok(StandardCursorSpriteSet {
                        sheet,
                        columns,
                        rows,
                        cursor_wh,
                        sprites,
                    })
                }
            }
            pub enum CursorSprite {
                Static {
                    index: usize,
                    hotspot_xy: Xy<Px>,
                },
                Animated {
                    start_index: usize,
                    hotspot_xy: Xy<Px>,
                    frame_count: usize,
                    frame_duration: Duration,
                },
            }
            impl bincode::Encode for CursorSprite {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    match self {
                        Self::Static { index, hotspot_xy } => {
                            bincode::Encode::encode(&0u32, encoder)?;
                            bincode::Encode::encode(index, encoder)?;
                            bincode::Encode::encode(hotspot_xy, encoder)?;
                        }
                        Self::Animated {
                            start_index,
                            hotspot_xy,
                            frame_count,
                            frame_duration,
                        } => {
                            bincode::Encode::encode(&1u32, encoder)?;
                            bincode::Encode::encode(start_index, encoder)?;
                            bincode::Encode::encode(hotspot_xy, encoder)?;
                            bincode::Encode::encode(frame_count, encoder)?;
                            bincode::Encode::encode(frame_duration, encoder)?;
                        }
                    }
                    Ok(())
                }
            }
            impl bincode::Decode<()> for CursorSprite {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    let discriminant: u32 = bincode::Decode::decode(decoder)?;
                    match discriminant {
                        0u32 => Ok(Self::Static {
                            index: bincode::Decode::decode(decoder)?,
                            hotspot_xy: bincode::Decode::decode(decoder)?,
                        }),
                        1u32 => Ok(Self::Animated {
                            start_index: bincode::Decode::decode(decoder)?,
                            hotspot_xy: bincode::Decode::decode(decoder)?,
                            frame_count: bincode::Decode::decode(decoder)?,
                            frame_duration: bincode::Decode::decode(decoder)?,
                        }),
                        _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                            type_name: core::any::type_name::<Self>(),
                            allowed: &bincode::error::AllowedEnumVariants::Range {
                                min: 0,
                                max: 1u32,
                            },
                            found: discriminant,
                        }),
                    }
                }
            }
            impl Serialize for CursorSprite {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    match self {
                        Self::Static { index, hotspot_xy } => {
                            buffer.write_string("Static");
                            buffer.write_string("index");
                            let field_bytes = Serialize::serialize(index);
                            buffer.put_slice(&field_bytes);
                            buffer.write_string("hotspot_xy");
                            let field_bytes = Serialize::serialize(hotspot_xy);
                            buffer.put_slice(&field_bytes);
                        }
                        Self::Animated {
                            start_index,
                            hotspot_xy,
                            frame_count,
                            frame_duration,
                        } => {
                            buffer.write_string("Animated");
                            buffer.write_string("start_index");
                            let field_bytes = Serialize::serialize(start_index);
                            buffer.put_slice(&field_bytes);
                            buffer.write_string("hotspot_xy");
                            let field_bytes = Serialize::serialize(hotspot_xy);
                            buffer.put_slice(&field_bytes);
                            buffer.write_string("frame_count");
                            let field_bytes = Serialize::serialize(frame_count);
                            buffer.put_slice(&field_bytes);
                            buffer.write_string("frame_duration");
                            let field_bytes = Serialize::serialize(frame_duration);
                            buffer.put_slice(&field_bytes);
                        }
                    }
                    buffer
                }
            }
            impl Deserialize for CursorSprite {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    use bytes::Buf;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let variant_name = buf.read_string();
                    match variant_name.as_ref() {
                        "Static" => {
                            let field_name = buf.read_name("index")?;
                            let index = Deserialize::deserialize(buf)?;
                            let field_name = buf.read_name("hotspot_xy")?;
                            let hotspot_xy = Deserialize::deserialize(buf)?;
                            Ok(Self::Static { index, hotspot_xy })
                        }
                        "Animated" => {
                            let field_name = buf.read_name("start_index")?;
                            let start_index = Deserialize::deserialize(buf)?;
                            let field_name = buf.read_name("hotspot_xy")?;
                            let hotspot_xy = Deserialize::deserialize(buf)?;
                            let field_name = buf.read_name("frame_count")?;
                            let frame_count = Deserialize::deserialize(buf)?;
                            let field_name = buf.read_name("frame_duration")?;
                            let frame_duration = Deserialize::deserialize(buf)?;
                            Ok(Self::Animated {
                                start_index,
                                hotspot_xy,
                                frame_count,
                                frame_duration,
                            })
                        }
                        _ => Err(DeserializeError::InvalidEnumVariant {
                            expected: std::any::type_name::<Self>().to_string(),
                            actual: variant_name,
                        }),
                    }
                }
            }
            fn parse_parts<'a, T: std::str::FromStr>(
                parts: &mut impl Iterator<Item = &'a str>,
                name: &str,
            ) -> anyhow::Result<T> {
                parts
                    .next()
                    .ok_or_else(|| {
                        ::anyhow::__private::must_use({
                            let error =
                                ::anyhow::__private::format_err(format_args!("Missing {0}", name));
                            error
                        })
                    })?
                    .parse::<T>()
                    .map_err(|_| {
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(format_args!(
                                "Failed to parse {0}",
                                name
                            ));
                            error
                        })
                    })
            }
        }
        pub mod on_top {
            use super::*;
            /// `OnTopNode` ignores clip and draw on top of other nodes.
            pub struct OnTopNode {
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for OnTopNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "OnTopNode",
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for OnTopNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for OnTopNode {
                #[inline]
                fn eq(&self, other: &OnTopNode) -> bool {
                    self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for OnTopNode {
                #[inline]
                fn clone(&self) -> OnTopNode {
                    OnTopNode {
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for OnTopNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for OnTopNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for OnTopNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for OnTopNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for OnTopNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for OnTopNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self { rendering_tree })
                }
            }
            /// `on_top` ignores clip and draw on top of other nodes.
            /// If you want to attach event to on_top, make sure that you put `attach_event` inside `on_top`.
            /// ```ignore
            /// // X - wrong
            /// namui::on_top(render([])).attach_event(|_| {});
            /// // O - right
            /// namui::on_top(render([]).attach_event(|_| {}));
            /// ```
            pub fn on_top(rendering_tree: RenderingTree) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
                    rendering_tree: rendering_tree.into(),
                }))
            }
        }
        pub mod rotate {
            use super::*;
            pub struct RotateNode {
                pub angle: Angle,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for RotateNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "RotateNode",
                        "angle",
                        &self.angle,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for RotateNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for RotateNode {
                #[inline]
                fn eq(&self, other: &RotateNode) -> bool {
                    self.angle == other.angle && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for RotateNode {
                #[inline]
                fn clone(&self) -> RotateNode {
                    RotateNode {
                        angle: ::core::clone::Clone::clone(&self.angle),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for RotateNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.angle, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for RotateNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Angle>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for RotateNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.angle, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for RotateNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        angle: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for RotateNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("angle");
                    let field_bytes = Serialize::serialize(&self.angle);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for RotateNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("angle")?;
                    let angle = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        angle,
                        rendering_tree,
                    })
                }
            }
            /// angle is in **cw** direction.
            pub fn rotate(angle: Angle, rendering_tree: RenderingTree) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
                    angle,
                    rendering_tree: rendering_tree.into(),
                }))
            }
            impl RotateNode {
                pub fn get_matrix(&self) -> TransformMatrix {
                    TransformMatrix::from_rotate(self.angle)
                }
                pub fn get_counter_wise_matrix(&self) -> TransformMatrix {
                    TransformMatrix::from_rotate(-self.angle)
                }
            }
        }
        pub mod scale {
            use super::*;
            pub struct ScaleNode {
                pub x: OrderedFloat,
                pub y: OrderedFloat,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ScaleNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ScaleNode",
                        "x",
                        &self.x,
                        "y",
                        &self.y,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ScaleNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ScaleNode {
                #[inline]
                fn eq(&self, other: &ScaleNode) -> bool {
                    self.x == other.x
                        && self.y == other.y
                        && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ScaleNode {
                #[inline]
                fn clone(&self) -> ScaleNode {
                    ScaleNode {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for ScaleNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.x, state);
                    ::core::hash::Hash::hash(&self.y, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ScaleNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<OrderedFloat>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for ScaleNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.x, encoder)?;
                    bincode::Encode::encode(&self.y, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for ScaleNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        x: bincode::Decode::decode(decoder)?,
                        y: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for ScaleNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("x");
                    let field_bytes = Serialize::serialize(&self.x);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("y");
                    let field_bytes = Serialize::serialize(&self.y);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for ScaleNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("x")?;
                    let x = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("y")?;
                    let y = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        x,
                        y,
                        rendering_tree,
                    })
                }
            }
            pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
                    x: x.into(),
                    y: y.into(),
                    rendering_tree: rendering_tree.into(),
                }))
            }
            impl ScaleNode {
                pub fn get_matrix(&self) -> TransformMatrix {
                    TransformMatrix::from_scale(*self.x, *self.y)
                }
            }
        }
        pub mod transform {
            use super::*;
            pub struct TransformNode {
                pub matrix: TransformMatrix,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TransformNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TransformNode",
                        "matrix",
                        &self.matrix,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TransformNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TransformNode {
                #[inline]
                fn eq(&self, other: &TransformNode) -> bool {
                    self.matrix == other.matrix && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TransformNode {
                #[inline]
                fn clone(&self) -> TransformNode {
                    TransformNode {
                        matrix: ::core::clone::Clone::clone(&self.matrix),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for TransformNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.matrix, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for TransformNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<TransformMatrix>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for TransformNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.matrix, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for TransformNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        matrix: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for TransformNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("matrix");
                    let field_bytes = Serialize::serialize(&self.matrix);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for TransformNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("matrix")?;
                    let matrix = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        matrix,
                        rendering_tree,
                    })
                }
            }
            pub fn transform(
                matrix: TransformMatrix,
                rendering_tree: RenderingTree,
            ) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::Transform(TransformNode {
                    matrix,
                    rendering_tree: rendering_tree.into(),
                }))
            }
        }
        pub mod translate {
            use super::*;
            pub struct TranslateNode {
                pub x: Px,
                pub y: Px,
                pub rendering_tree: Box<RenderingTree>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TranslateNode {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "TranslateNode",
                        "x",
                        &self.x,
                        "y",
                        &self.y,
                        "rendering_tree",
                        &&self.rendering_tree,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TranslateNode {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TranslateNode {
                #[inline]
                fn eq(&self, other: &TranslateNode) -> bool {
                    self.x == other.x
                        && self.y == other.y
                        && self.rendering_tree == other.rendering_tree
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TranslateNode {
                #[inline]
                fn clone(&self) -> TranslateNode {
                    TranslateNode {
                        x: ::core::clone::Clone::clone(&self.x),
                        y: ::core::clone::Clone::clone(&self.y),
                        rendering_tree: ::core::clone::Clone::clone(&self.rendering_tree),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for TranslateNode {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.x, state);
                    ::core::hash::Hash::hash(&self.y, state);
                    ::core::hash::Hash::hash(&self.rendering_tree, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for TranslateNode {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Px>;
                    let _: ::core::cmp::AssertParamIsEq<Box<RenderingTree>>;
                }
            }
            impl bincode::Encode for TranslateNode {
                fn encode<__E: bincode::enc::Encoder>(
                    &self,
                    encoder: &mut __E,
                ) -> core::result::Result<(), bincode::error::EncodeError> {
                    bincode::Encode::encode(&self.x, encoder)?;
                    bincode::Encode::encode(&self.y, encoder)?;
                    bincode::Encode::encode(&self.rendering_tree, encoder)?;
                    Ok(())
                }
            }
            impl bincode::Decode<()> for TranslateNode {
                fn decode<__D: bincode::de::Decoder<Context = ()>>(
                    decoder: &mut __D,
                ) -> core::result::Result<Self, bincode::error::DecodeError> {
                    Ok(Self {
                        x: bincode::Decode::decode(decoder)?,
                        y: bincode::Decode::decode(decoder)?,
                        rendering_tree: bincode::Decode::decode(decoder)?,
                    })
                }
            }
            impl Serialize for TranslateNode {
                fn serialize(&self) -> Vec<u8> {
                    use BufMutExt;
                    use bytes::BufMut;
                    let mut buffer = ::alloc::vec::Vec::new();
                    buffer.write_string(std::any::type_name::<Self>());
                    buffer.write_string("x");
                    let field_bytes = Serialize::serialize(&self.x);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("y");
                    let field_bytes = Serialize::serialize(&self.y);
                    buffer.put_slice(&field_bytes);
                    buffer.write_string("rendering_tree");
                    let field_bytes = Serialize::serialize(&self.rendering_tree);
                    buffer.put_slice(&field_bytes);
                    buffer
                }
            }
            impl Deserialize for TranslateNode {
                fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                    use BufExt;
                    buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {            let field_name = buf.read_name("x")?;
                    let x = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("y")?;
                    let y = Deserialize::deserialize(buf)?;
                    let field_name = buf.read_name("rendering_tree")?;
                    let rendering_tree = Deserialize::deserialize(buf)?;
                    Ok(Self {
                        x,
                        y,
                        rendering_tree,
                    })
                }
            }
            impl TranslateNode {
                pub fn get_matrix(&self) -> TransformMatrix {
                    TransformMatrix::from_translate(self.x.as_f32(), self.y.as_f32())
                }
            }
            pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
                if rendering_tree == RenderingTree::Empty {
                    return RenderingTree::Empty;
                }
                RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
                    x,
                    y,
                    rendering_tree: rendering_tree.into(),
                }))
            }
        }
        use crate::*;
        pub use absolute::*;
        pub use clip::*;
        pub use mouse_cursor::*;
        pub use on_top::*;
        pub use rotate::*;
        pub use scale::*;
        pub use transform::*;
        pub use translate::*;
        pub enum SpecialRenderingNode {
            Translate(TranslateNode),
            Clip(ClipNode),
            Absolute(AbsoluteNode),
            Rotate(RotateNode),
            Scale(ScaleNode),
            Transform(TransformNode),
            OnTop(OnTopNode),
            MouseCursor(MouseCursorNode),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SpecialRenderingNode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    SpecialRenderingNode::Translate(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Translate", &__self_0)
                    }
                    SpecialRenderingNode::Clip(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Clip", &__self_0)
                    }
                    SpecialRenderingNode::Absolute(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Absolute", &__self_0)
                    }
                    SpecialRenderingNode::Rotate(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Rotate", &__self_0)
                    }
                    SpecialRenderingNode::Scale(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Scale", &__self_0)
                    }
                    SpecialRenderingNode::Transform(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Transform", &__self_0)
                    }
                    SpecialRenderingNode::OnTop(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "OnTop", &__self_0)
                    }
                    SpecialRenderingNode::MouseCursor(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "MouseCursor",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SpecialRenderingNode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SpecialRenderingNode {
            #[inline]
            fn eq(&self, other: &SpecialRenderingNode) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            SpecialRenderingNode::Translate(__self_0),
                            SpecialRenderingNode::Translate(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::Clip(__self_0),
                            SpecialRenderingNode::Clip(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::Absolute(__self_0),
                            SpecialRenderingNode::Absolute(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::Rotate(__self_0),
                            SpecialRenderingNode::Rotate(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::Scale(__self_0),
                            SpecialRenderingNode::Scale(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::Transform(__self_0),
                            SpecialRenderingNode::Transform(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::OnTop(__self_0),
                            SpecialRenderingNode::OnTop(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            SpecialRenderingNode::MouseCursor(__self_0),
                            SpecialRenderingNode::MouseCursor(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SpecialRenderingNode {
            #[inline]
            fn clone(&self) -> SpecialRenderingNode {
                match self {
                    SpecialRenderingNode::Translate(__self_0) => {
                        SpecialRenderingNode::Translate(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::Clip(__self_0) => {
                        SpecialRenderingNode::Clip(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::Absolute(__self_0) => {
                        SpecialRenderingNode::Absolute(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::Rotate(__self_0) => {
                        SpecialRenderingNode::Rotate(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::Scale(__self_0) => {
                        SpecialRenderingNode::Scale(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::Transform(__self_0) => {
                        SpecialRenderingNode::Transform(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::OnTop(__self_0) => {
                        SpecialRenderingNode::OnTop(::core::clone::Clone::clone(__self_0))
                    }
                    SpecialRenderingNode::MouseCursor(__self_0) => {
                        SpecialRenderingNode::MouseCursor(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for SpecialRenderingNode {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    SpecialRenderingNode::Translate(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::Clip(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::Absolute(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::Rotate(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::Scale(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::Transform(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::OnTop(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    SpecialRenderingNode::MouseCursor(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SpecialRenderingNode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<TranslateNode>;
                let _: ::core::cmp::AssertParamIsEq<ClipNode>;
                let _: ::core::cmp::AssertParamIsEq<AbsoluteNode>;
                let _: ::core::cmp::AssertParamIsEq<RotateNode>;
                let _: ::core::cmp::AssertParamIsEq<ScaleNode>;
                let _: ::core::cmp::AssertParamIsEq<TransformNode>;
                let _: ::core::cmp::AssertParamIsEq<OnTopNode>;
                let _: ::core::cmp::AssertParamIsEq<MouseCursorNode>;
            }
        }
        impl bincode::Encode for SpecialRenderingNode {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Translate(field0) => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Clip(field0) => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Absolute(field0) => {
                        bincode::Encode::encode(&2u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Rotate(field0) => {
                        bincode::Encode::encode(&3u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Scale(field0) => {
                        bincode::Encode::encode(&4u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Transform(field0) => {
                        bincode::Encode::encode(&5u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::OnTop(field0) => {
                        bincode::Encode::encode(&6u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::MouseCursor(field0) => {
                        bincode::Encode::encode(&7u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for SpecialRenderingNode {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Translate(bincode::Decode::decode(decoder)?)),
                    1u32 => Ok(Self::Clip(bincode::Decode::decode(decoder)?)),
                    2u32 => Ok(Self::Absolute(bincode::Decode::decode(decoder)?)),
                    3u32 => Ok(Self::Rotate(bincode::Decode::decode(decoder)?)),
                    4u32 => Ok(Self::Scale(bincode::Decode::decode(decoder)?)),
                    5u32 => Ok(Self::Transform(bincode::Decode::decode(decoder)?)),
                    6u32 => Ok(Self::OnTop(bincode::Decode::decode(decoder)?)),
                    7u32 => Ok(Self::MouseCursor(bincode::Decode::decode(decoder)?)),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 7u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for SpecialRenderingNode {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Translate { field0 } => {
                        buffer.write_string("Translate");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Clip { field0 } => {
                        buffer.write_string("Clip");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Absolute { field0 } => {
                        buffer.write_string("Absolute");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Rotate { field0 } => {
                        buffer.write_string("Rotate");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Scale { field0 } => {
                        buffer.write_string("Scale");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Transform { field0 } => {
                        buffer.write_string("Transform");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::OnTop { field0 } => {
                        buffer.write_string("OnTop");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::MouseCursor { field0 } => {
                        buffer.write_string("MouseCursor");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for SpecialRenderingNode {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Translate" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Translate(field0))
                    }
                    "Clip" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Clip(field0))
                    }
                    "Absolute" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Absolute(field0))
                    }
                    "Rotate" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Rotate(field0))
                    }
                    "Scale" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Scale(field0))
                    }
                    "Transform" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Transform(field0))
                    }
                    "OnTop" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::OnTop(field0))
                    }
                    "MouseCursor" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::MouseCursor(field0))
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl SpecialRenderingNode {
            pub fn inner_rendering_tree_ref(&self) -> &RenderingTree {
                match self {
                    SpecialRenderingNode::Translate(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::Clip(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::Absolute(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::Rotate(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::Scale(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::Transform(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::OnTop(node) => node.rendering_tree.as_ref(),
                    SpecialRenderingNode::MouseCursor(node) => node.rendering_tree.as_ref(),
                }
            }
            pub fn inner_rendering_tree(self) -> RenderingTree {
                match self {
                    SpecialRenderingNode::Translate(node) => *node.rendering_tree,
                    SpecialRenderingNode::Clip(node) => *node.rendering_tree,
                    SpecialRenderingNode::Absolute(node) => *node.rendering_tree,
                    SpecialRenderingNode::Rotate(node) => *node.rendering_tree,
                    SpecialRenderingNode::Scale(node) => *node.rendering_tree,
                    SpecialRenderingNode::Transform(node) => *node.rendering_tree,
                    SpecialRenderingNode::OnTop(node) => *node.rendering_tree,
                    SpecialRenderingNode::MouseCursor(node) => *node.rendering_tree,
                }
            }
        }
    }
    use crate::*;
    pub use special::*;
    pub enum RenderingTree {
        #[default]
        Empty,
        Node(DrawCommand),
        Children(Vec<RenderingTree>),
        Special(SpecialRenderingNode),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RenderingTree {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                RenderingTree::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
                RenderingTree::Node(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Node", &__self_0)
                }
                RenderingTree::Children(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Children", &__self_0)
                }
                RenderingTree::Special(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Special", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RenderingTree {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RenderingTree {
        #[inline]
        fn eq(&self, other: &RenderingTree) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (RenderingTree::Node(__self_0), RenderingTree::Node(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (RenderingTree::Children(__self_0), RenderingTree::Children(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (RenderingTree::Special(__self_0), RenderingTree::Special(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RenderingTree {
        #[inline]
        fn clone(&self) -> RenderingTree {
            match self {
                RenderingTree::Empty => RenderingTree::Empty,
                RenderingTree::Node(__self_0) => {
                    RenderingTree::Node(::core::clone::Clone::clone(__self_0))
                }
                RenderingTree::Children(__self_0) => {
                    RenderingTree::Children(::core::clone::Clone::clone(__self_0))
                }
                RenderingTree::Special(__self_0) => {
                    RenderingTree::Special(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for RenderingTree {
        #[inline]
        fn default() -> RenderingTree {
            Self::Empty
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for RenderingTree {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                RenderingTree::Node(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                RenderingTree::Children(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                RenderingTree::Special(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                _ => {}
            }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for RenderingTree {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<DrawCommand>;
            let _: ::core::cmp::AssertParamIsEq<Vec<RenderingTree>>;
            let _: ::core::cmp::AssertParamIsEq<SpecialRenderingNode>;
        }
    }
    impl bincode::Encode for RenderingTree {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Empty => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Node(field0) => {
                    bincode::Encode::encode(&1u32, encoder)?;
                    bincode::Encode::encode(field0, encoder)?;
                }
                Self::Children(field0) => {
                    bincode::Encode::encode(&2u32, encoder)?;
                    bincode::Encode::encode(field0, encoder)?;
                }
                Self::Special(field0) => {
                    bincode::Encode::encode(&3u32, encoder)?;
                    bincode::Encode::encode(field0, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for RenderingTree {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Empty),
                1u32 => Ok(Self::Node(bincode::Decode::decode(decoder)?)),
                2u32 => Ok(Self::Children(bincode::Decode::decode(decoder)?)),
                3u32 => Ok(Self::Special(bincode::Decode::decode(decoder)?)),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 3u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for RenderingTree {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Empty {} => {
                    buffer.write_string("Empty");
                }
                Self::Node { field0 } => {
                    buffer.write_string("Node");
                    let field_bytes = Serialize::serialize(field0);
                    buffer.put_slice(&field_bytes);
                }
                Self::Children { field0 } => {
                    buffer.write_string("Children");
                    let field_bytes = Serialize::serialize(field0);
                    buffer.put_slice(&field_bytes);
                }
                Self::Special { field0 } => {
                    buffer.write_string("Special");
                    let field_bytes = Serialize::serialize(field0);
                    buffer.put_slice(&field_bytes);
                }
            }
            buffer
        }
    }
    impl Deserialize for RenderingTree {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Empty" => Ok(Self::Empty),
                "Node" => {
                    let field0 = { Deserialize::deserialize(buf)? };
                    Ok(Self::Node(field0))
                }
                "Children" => {
                    let field0 = { Deserialize::deserialize(buf)? };
                    Ok(Self::Children(field0))
                }
                "Special" => {
                    let field0 = { Deserialize::deserialize(buf)? };
                    Ok(Self::Special(field0))
                }
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    /// NOTE
    /// Order of tree traversal is important.
    /// - draw = pre-order dfs (NLR)
    /// - events = Reverse post-order (RLN)
    ///
    /// reference: https://en.wikipedia.org/wiki/Tree_traversal
    impl RenderingTree {
        pub fn iter(&self) -> impl Iterator<Item = &RenderingTree> {
            let mut vec = ::alloc::vec::Vec::new();
            match self {
                RenderingTree::Children(children) => {
                    vec.extend(children.iter());
                }
                RenderingTree::Node(_) | RenderingTree::Special(_) => vec.push(self),
                RenderingTree::Empty => {}
            };
            vec.into_iter()
        }
        pub fn wrap(rendering_trees: impl IntoIterator<Item = RenderingTree>) -> RenderingTree {
            let mut iter = rendering_trees.into_iter();
            let first = 'outer: {
                for x in iter.by_ref() {
                    if x != RenderingTree::Empty {
                        break 'outer x;
                    }
                }
                return RenderingTree::Empty;
            };
            let second = 'outer: {
                for x in iter.by_ref() {
                    if x != RenderingTree::Empty {
                        break 'outer x;
                    }
                }
                return first;
            };
            let mut children = <[_]>::into_vec(::alloc::boxed::box_new([first, second]));
            children.extend(iter.filter(|x| *x != RenderingTree::Empty));
            RenderingTree::Children(children)
        }
    }
}
mod skia_types {
    mod blender {
        use crate::*;
        use std::sync::Arc;
        pub struct NativeBlender {
            pub skia_blender: skia_safe::Blender,
        }
        impl NativeBlender {
            pub fn get(blender: &Blender) -> Arc<NativeBlender> {
                static CACHE: LruCache<Blender, NativeBlender> = LruCache::new();
                CACHE.get_or_create(blender, |blender| blender.into())
            }
            pub fn skia(&self) -> &skia_safe::Blender {
                &self.skia_blender
            }
        }
    }
    mod color_filter {
        use crate::*;
        use namui_type::*;
        use std::sync::Arc;
        pub struct NativeColorFilter {
            pub skia_color_filter: skia_safe::ColorFilter,
        }
        impl NativeColorFilter {
            pub fn get(color_filter: ColorFilter) -> Arc<NativeColorFilter> {
                static CACHE: LruCache<ColorFilter, NativeColorFilter> = LruCache::new();
                CACHE.get_or_create(&color_filter, |color_filter| color_filter.into())
            }
            pub fn skia(&self) -> &skia_safe::ColorFilter {
                &self.skia_color_filter
            }
        }
    }
    mod font {
        use super::*;
        use crate::*;
        use std::sync::Arc;
        pub struct NativeFont {
            skia_font: skia_safe::Font,
            pub metrics: FontMetrics,
            glyph_ids_caches: LruCache<String, GlyphIds>,
            glyph_widths_caches: LruCache<(GlyphIds, Paint), Vec<Px>>,
            glyph_bounds_caches: LruCache<(GlyphIds, Paint), Vec<Rect<Px>>>,
        }
        impl NativeFont {
            pub fn get(font: &Font) -> Option<Arc<Self>> {
                static FONT_MAP: StaticHashMap<Font, NativeFont> = StaticHashMap::new();
                FONT_MAP.get_or_try_create(font.clone(), |font| {
                    let typeface = NativeTypeface::get(&font.name)?;
                    let skia_font = skia_safe::Font::from_typeface(
                        &typeface.skia_typeface,
                        Some(font.size.as_i32() as f32),
                    );
                    let metrics = {
                        let (_line_spacing, skia_font_metrics) = &skia_font.metrics();
                        FontMetrics {
                            ascent: skia_font_metrics.ascent.into(),
                            descent: skia_font_metrics.descent.into(),
                            leading: skia_font_metrics.leading.into(),
                        }
                    };
                    Some(NativeFont {
                        skia_font,
                        metrics,
                        glyph_ids_caches: Default::default(),
                        glyph_widths_caches: Default::default(),
                        glyph_bounds_caches: Default::default(),
                    })
                })
            }
            pub fn skia(&self) -> &skia_safe::Font {
                &self.skia_font
            }
        }
        impl NativeFont {
            pub fn glyph_ids(&self, text: impl AsRef<str>) -> GlyphIds {
                let text = text.as_ref().to_string();
                if text.is_empty() {
                    return ::alloc::vec::Vec::new();
                }
                self.glyph_ids_caches
                    .get_or_create(&text, |text| self.skia_font.str_to_glyphs_vec(text))
                    .to_vec()
            }
            pub fn glyph_widths(&self, glyph_ids: GlyphIds, paint: &Paint) -> Vec<Px> {
                if glyph_ids.is_empty() {
                    return ::alloc::vec::Vec::new();
                }
                self.glyph_widths_caches
                    .get_or_create(&(glyph_ids, paint.clone()), |(glyph_ids, paint)| {
                        let native_paint = NativePaint::get(paint);
                        let mut widths = ::alloc::vec::from_elem(0.0, glyph_ids.len());
                        self.skia_font.get_widths_bounds(
                            glyph_ids,
                            Some(&mut widths),
                            None,
                            Some(native_paint.skia()),
                        );
                        widths.into_iter().map(|n| n.into()).collect()
                    })
                    .to_vec()
            }
            pub fn glyph_bounds(&self, glyph_ids: GlyphIds, paint: &Paint) -> Vec<Rect<Px>> {
                if glyph_ids.is_empty() {
                    return ::alloc::vec::Vec::new();
                }
                self.glyph_bounds_caches
                    .get_or_create(&(glyph_ids, paint.clone()), |(glyph_ids, paint)| {
                        let native_paint = NativePaint::get(paint);
                        let mut bounds =
                            ::alloc::vec::from_elem(skia_safe::Rect::default(), glyph_ids.len());
                        self.skia_font.get_bounds(
                            glyph_ids,
                            &mut bounds,
                            Some(native_paint.skia()),
                        );
                        bounds
                            .into_iter()
                            .map(|rect| Rect::Ltrb {
                                left: rect.left.into(),
                                top: rect.top.into(),
                                right: rect.right.into(),
                                bottom: rect.bottom.into(),
                            })
                            .collect()
                    })
                    .to_vec()
            }
        }
    }
    mod paint {
        use crate::*;
        use std::sync::Arc;
        pub struct NativePaint {
            skia_paint: skia_safe::Paint,
        }
        impl NativePaint {
            pub fn get(paint: &Paint) -> Arc<Self> {
                static NATIVE_PAINT_CACHE: LruCache<Paint, NativePaint, 128> = LruCache::new();
                NATIVE_PAINT_CACHE.get_or_create(paint, Self::new)
            }
            fn new(paint: &Paint) -> Self {
                NativePaint {
                    skia_paint: new_skia_paint(paint),
                }
            }
            pub fn skia(&self) -> &skia_safe::Paint {
                &self.skia_paint
            }
        }
        fn new_skia_paint(paint: &Paint) -> skia_safe::Paint {
            let mut skia_paint = skia_safe::Paint::new(skia_safe::Color4f::from(paint.color), None);
            let &Paint {
                color: _,
                paint_style,
                anti_alias,
                stroke_width,
                stroke_cap,
                stroke_join,
                stroke_miter,
                color_filter,
                blend_mode,
                ref shader,
                mask_filter,
                ref image_filter,
                ..
            } = paint;
            if let Some(style) = paint_style {
                skia_paint.set_style(style.into());
            }
            if let Some(anti_alias) = anti_alias {
                skia_paint.set_anti_alias(anti_alias);
            }
            if stroke_width > 0.px() {
                skia_paint.set_stroke_width(stroke_width.as_f32());
            }
            if let Some(stroke_cap) = stroke_cap {
                skia_paint.set_stroke_cap(stroke_cap.into());
            }
            if let Some(stroke_join) = stroke_join {
                skia_paint.set_stroke_join(stroke_join.into());
            }
            if stroke_miter > 0.px() {
                skia_paint.set_stroke_miter(stroke_miter.as_f32());
            }
            if let Some(color_filter) = color_filter {
                let native_color_filter = NativeColorFilter::get(color_filter);
                skia_paint.set_color_filter(Some(native_color_filter.skia().clone()));
            }
            if let Some(blend_mode) = blend_mode {
                skia_paint.set_blend_mode(blend_mode.into());
            }
            if let Some(shader) = shader {
                let native_shader = NativeShader::get(shader);
                skia_paint.set_shader(Some(native_shader.skia().clone()));
            }
            if let Some(mask_filter) = mask_filter {
                skia_paint.set_mask_filter(match mask_filter {
                    MaskFilter::Blur { blur_style, sigma } => {
                        skia_safe::MaskFilter::blur(blur_style.into(), sigma, false)
                    }
                });
            }
            if let Some(image_filter) = image_filter {
                skia_paint.set_image_filter(Some(image_filter.as_ref().into()));
            }
            skia_paint
        }
    }
    mod path {
        use crate::*;
        use std::sync::Arc;
        pub struct NativePath {
            skia_path: skia_safe::Path,
            path: Path,
        }
        impl NativePath {
            pub fn get(path: &Path) -> Arc<Self> {
                static CACHE: LruCache<Path, NativePath> = LruCache::new();
                CACHE.get_or_create(path, NativePath::new)
            }
            pub fn new(path: &Path) -> Self {
                let mut skia_path = skia_safe::Path::new();
                apply_command_to_skia_path(&mut skia_path, path);
                NativePath {
                    skia_path,
                    path: path.clone(),
                }
            }
            fn painted_path(path: &Path, paint: &Paint) -> Arc<Self> {
                let path = path.clone().stroke(StrokeOptions {
                    cap: paint.stroke_cap,
                    join: paint.stroke_join,
                    width: Some(paint.stroke_width),
                    miter_limit: Some(paint.stroke_miter),
                    precision: None,
                });
                Self::get(&path)
            }
            pub fn contains(&self, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
                if self.skia().contains((xy.x.as_f32(), xy.y.as_f32())) {
                    return true;
                }
                let Some(paint) = paint else {
                    return false;
                };
                Self::painted_path(&self.path, paint)
                    .skia()
                    .contains((xy.x.as_f32(), xy.y.as_f32()))
            }
            pub fn bounding_box(&self, paint: Option<&Paint>) -> Option<Rect<Px>> {
                if let Some(paint) = paint {
                    Self::painted_path(&self.path, paint).bounding_box(None)
                } else {
                    let bounds = self.skia_path.bounds();
                    if bounds.left == 0.0
                        && bounds.top == 0.0
                        && bounds.right == 0.0
                        && bounds.bottom == 0.0
                    {
                        None
                    } else {
                        Some((*bounds).into())
                    }
                }
            }
            pub fn skia(&self) -> &skia_safe::Path {
                &self.skia_path
            }
        }
        fn apply_command_to_skia_path(skia_path: &mut skia_safe::Path, path: &Path) {
            for command in path.commands() {
                match command {
                    &PathCommand::AddRect { rect } => {
                        skia_path.add_rect(skia_safe::Rect::from(rect), None);
                    }
                    &PathCommand::AddRrect { rect, rx, ry } => {
                        skia_path.add_rrect(
                            skia_safe::RRect::new_rect_xy(
                                skia_safe::Rect::from(rect),
                                rx.into(),
                                ry.into(),
                            ),
                            None,
                        );
                    }
                    PathCommand::Stroke { stroke_options } => {
                        let mut paint = skia_safe::Paint::default();
                        paint.set_style(skia_safe::PaintStyle::Stroke);
                        paint.set_stroke_cap(
                            stroke_options
                                .cap
                                .map(|c| c.into())
                                .unwrap_or(skia_safe::PaintCap::Butt),
                        );
                        paint.set_stroke_join(
                            stroke_options
                                .join
                                .map(|j| j.into())
                                .unwrap_or(skia_safe::PaintJoin::Miter),
                        );
                        paint.set_stroke_width(
                            stroke_options.width.map(|w| w.into()).unwrap_or(1.0),
                        );
                        paint.set_stroke_miter(
                            stroke_options.miter_limit.map(|m| m.into()).unwrap_or(4.0),
                        );
                        let precision = *stroke_options.precision.unwrap_or(1.0.into());
                        if !skia_safe::path_utils::fill_path_with_paint(
                            &skia_path.clone(),
                            &paint,
                            skia_path,
                            None,
                            skia_safe::Matrix::scale((precision, precision)),
                        ) {}
                    }
                    PathCommand::MoveTo { xy } => {
                        skia_path.move_to(*xy);
                    }
                    PathCommand::LineTo { xy } => {
                        skia_path.line_to(*xy);
                    }
                    PathCommand::CubicTo {
                        first_xy,
                        second_xy,
                        end_xy,
                    } => {
                        skia_path.cubic_to(*first_xy, *second_xy, *end_xy);
                    }
                    &PathCommand::ArcTo {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        skia_path.arc_to(
                            skia_safe::Rect::from(oval),
                            start_angle.as_degrees(),
                            delta_angle.as_degrees(),
                            false,
                        );
                    }
                    &PathCommand::Scale { xy } => {
                        skia_path
                            .transform(&skia_safe::Matrix::scale(xy.map(|x| x.as_f32()).into()));
                    }
                    &PathCommand::Translate { xy } => {
                        skia_path.offset(xy);
                    }
                    &PathCommand::Transform { matrix } => {
                        skia_path.transform(&matrix.into());
                    }
                    &PathCommand::AddOval { rect } => {
                        skia_path.add_oval(skia_safe::Rect::from(rect), None);
                    }
                    &PathCommand::AddArc {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        skia_path.add_arc(
                            skia_safe::Rect::from(oval),
                            start_angle.as_degrees(),
                            delta_angle.as_degrees(),
                        );
                    }
                    PathCommand::AddPoly { xys, close } => {
                        let points = xys.iter().map(|xy| (*xy).into()).collect::<Vec<_>>();
                        skia_path.add_poly(points.as_slice(), *close);
                    }
                    PathCommand::Close => {
                        skia_path.close();
                    }
                }
            }
        }
    }
    mod shader {
        use crate::*;
        use std::sync::Arc;
        pub struct NativeShader {
            pub skia_shader: skia_safe::Shader,
        }
        unsafe impl Send for NativeShader {}
        unsafe impl Sync for NativeShader {}
        impl NativeShader {
            pub fn get(shader: &Shader) -> Arc<Self> {
                static NATIVE_SHADER_MAP: LruCache<Shader, NativeShader, 64> = LruCache::new();
                NATIVE_SHADER_MAP.get_or_create(shader, |shader| match shader {
                    Shader::Image { src, tile_mode } => NativeShader {
                        skia_shader: src
                            .skia_image()
                            .to_shader(
                                Some((tile_mode.x.into(), tile_mode.y.into())),
                                skia_safe::SamplingOptions::new(
                                    FilterMode::Linear.into(),
                                    MipmapMode::Linear.into(),
                                ),
                                None,
                            )
                            .expect("Failed to create shader from image"),
                    },
                    Shader::Blend {
                        blend_mode,
                        src,
                        dest,
                    } => {
                        let native_src = NativeShader::get(src);
                        let native_dest = NativeShader::get(dest);
                        let blended = skia_safe::shaders::blend(
                            skia_safe::BlendMode::from(*blend_mode),
                            &native_src.skia_shader,
                            &native_dest.skia_shader,
                        );
                        NativeShader {
                            skia_shader: blended,
                        }
                    }
                    &Shader::LinearGradient {
                        start_xy,
                        end_xy,
                        ref colors,
                        tile_mode,
                    } => {
                        let colors: Vec<_> = colors
                            .iter()
                            .map(|color| skia_safe::Color::from(*color))
                            .collect();
                        let shader = skia_safe::gradient_shader::linear(
                            (
                                skia_safe::Point::new(start_xy.x.into(), start_xy.y.into()),
                                skia_safe::Point::new(end_xy.x.into(), end_xy.y.into()),
                            ),
                            skia_safe::gradient_shader::GradientShaderColors::Colors(
                                colors.as_slice(),
                            ),
                            None,
                            tile_mode.into(),
                            None,
                            None,
                        )
                        .unwrap();
                        NativeShader {
                            skia_shader: shader,
                        }
                    }
                })
            }
            pub fn skia(&self) -> &skia_safe::Shader {
                &self.skia_shader
            }
        }
    }
    mod text_blob {
        use crate::*;
        use namui_type::*;
        use std::sync::Arc;
        pub struct NativeTextBlob {
            pub skia_text_blob: skia_safe::TextBlob,
        }
        impl NativeTextBlob {
            #[allow(dead_code)]
            pub fn from_text(string: &str, font: &NativeFont) -> Self {
                NativeTextBlob {
                    skia_text_blob: skia_safe::TextBlob::new(string, font.skia()).unwrap(),
                }
            }
            pub fn from_glyph_ids(glyph_ids: GlyphIds, font: &Font) -> Option<Arc<Self>> {
                struct CacheKey {
                    glyph_ids: GlyphIds,
                    font: Font,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for CacheKey {
                    #[inline]
                    fn clone(&self) -> CacheKey {
                        CacheKey {
                            glyph_ids: ::core::clone::Clone::clone(&self.glyph_ids),
                            font: ::core::clone::Clone::clone(&self.font),
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for CacheKey {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for CacheKey {
                    #[inline]
                    fn eq(&self, other: &CacheKey) -> bool {
                        self.glyph_ids == other.glyph_ids && self.font == other.font
                    }
                }
                #[automatically_derived]
                impl ::core::cmp::Eq for CacheKey {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        let _: ::core::cmp::AssertParamIsEq<GlyphIds>;
                        let _: ::core::cmp::AssertParamIsEq<Font>;
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for CacheKey {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        ::core::hash::Hash::hash(&self.glyph_ids, state);
                        ::core::hash::Hash::hash(&self.font, state)
                    }
                }
                static CACHE: LruCache<CacheKey, NativeTextBlob> = LruCache::new();
                let cache_key = CacheKey {
                    glyph_ids: glyph_ids.clone(),
                    font: font.clone(),
                };
                CACHE.get_or_try_create(&cache_key, |key| {
                    let native_font = NativeFont::get(&key.font)?;
                    let skia_text_blob =
                        skia_safe::TextBlob::from_text(glyph_ids.as_slice(), native_font.skia());
                    skia_text_blob.map(|skia_text_blob| NativeTextBlob { skia_text_blob })
                })
            }
            pub fn skia(&self) -> &skia_safe::TextBlob {
                &self.skia_text_blob
            }
        }
    }
    mod typeface {
        use crate::*;
        use dashmap::DashMap;
        use std::sync::{Arc, OnceLock};
        pub struct NativeTypeface {
            pub skia_typeface: skia_safe::Typeface,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NativeTypeface {
            #[inline]
            fn clone(&self) -> NativeTypeface {
                NativeTypeface {
                    skia_typeface: ::core::clone::Clone::clone(&self.skia_typeface),
                }
            }
        }
        static TYPEFACE_MAP: OnceLock<DashMap<String, Arc<NativeTypeface>>> = OnceLock::new();
        fn typeface_map() -> &'static DashMap<String, Arc<NativeTypeface>> {
            TYPEFACE_MAP.get_or_init(DashMap::new)
        }
        impl NativeTypeface {
            pub fn get(name: impl AsRef<str>) -> Option<Arc<Self>> {
                typeface_map()
                    .get(&name.as_ref().to_string())
                    .map(|v| v.value().clone())
            }
            pub fn load(name: impl AsRef<str>, bytes: &[u8]) -> anyhow::Result<()> {
                let skia_typeface = skia_safe::FontMgr::default()
                    .new_from_data(bytes, None)
                    .ok_or_else(|| {
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(format_args!(
                                "Failed to create a typeface from data."
                            ));
                            error
                        })
                    })?;
                typeface_map().insert(
                    name.as_ref().to_string(),
                    Arc::new(NativeTypeface { skia_typeface }),
                );
                Ok(())
            }
        }
        #[unsafe(no_mangle)]
        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        pub extern "C" fn _register_font(
            name_ptr: *const u8,
            name_len: usize,
            buffer_ptr: *const u8,
            buffer_len: usize,
        ) {
            let name_bytes = unsafe { std::slice::from_raw_parts(name_ptr, name_len) };
            let name = String::from_utf8_lossy(name_bytes).to_string();
            let buffer_bytes = unsafe { std::slice::from_raw_parts(buffer_ptr, buffer_len) };
            let buffer = Vec::from(buffer_bytes);
            if let Err(e) = NativeTypeface::load(&name, &buffer) {
                {
                    ::core::panicking::panic_fmt(format_args!(
                        "Failed to load font {0}: {1}",
                        name, e
                    ));
                };
            }
        }
    }
    pub use blender::*;
    pub use color_filter::*;
    pub use font::*;
    pub use paint::*;
    pub use path::*;
    pub use shader::*;
    pub use text_blob::*;
    pub use typeface::*;
}
mod types {
    mod blender {
        use crate::*;
        pub enum Blender {
            BlendMode(BlendMode),
            Sksl(String),
            Arithmetic {
                k1: OrderedFloat,
                k2: OrderedFloat,
                k3: OrderedFloat,
                k4: OrderedFloat,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Blender {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Blender::BlendMode(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "BlendMode", &__self_0)
                    }
                    Blender::Sksl(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Sksl", &__self_0)
                    }
                    Blender::Arithmetic {
                        k1: __self_0,
                        k2: __self_1,
                        k3: __self_2,
                        k4: __self_3,
                    } => ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Arithmetic",
                        "k1",
                        __self_0,
                        "k2",
                        __self_1,
                        "k3",
                        __self_2,
                        "k4",
                        &__self_3,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Blender {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Blender {
            #[inline]
            fn eq(&self, other: &Blender) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Blender::BlendMode(__self_0), Blender::BlendMode(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Blender::Sksl(__self_0), Blender::Sksl(__arg1_0)) => __self_0 == __arg1_0,
                        (
                            Blender::Arithmetic {
                                k1: __self_0,
                                k2: __self_1,
                                k3: __self_2,
                                k4: __self_3,
                            },
                            Blender::Arithmetic {
                                k1: __arg1_0,
                                k2: __arg1_1,
                                k3: __arg1_2,
                                k4: __arg1_3,
                            },
                        ) => {
                            __self_0 == __arg1_0
                                && __self_1 == __arg1_1
                                && __self_2 == __arg1_2
                                && __self_3 == __arg1_3
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Blender {
            #[inline]
            fn clone(&self) -> Blender {
                match self {
                    Blender::BlendMode(__self_0) => {
                        Blender::BlendMode(::core::clone::Clone::clone(__self_0))
                    }
                    Blender::Sksl(__self_0) => Blender::Sksl(::core::clone::Clone::clone(__self_0)),
                    Blender::Arithmetic {
                        k1: __self_0,
                        k2: __self_1,
                        k3: __self_2,
                        k4: __self_3,
                    } => Blender::Arithmetic {
                        k1: ::core::clone::Clone::clone(__self_0),
                        k2: ::core::clone::Clone::clone(__self_1),
                        k3: ::core::clone::Clone::clone(__self_2),
                        k4: ::core::clone::Clone::clone(__self_3),
                    },
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Blender {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    Blender::BlendMode(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    Blender::Sksl(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    Blender::Arithmetic {
                        k1: __self_0,
                        k2: __self_1,
                        k3: __self_2,
                        k4: __self_3,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state);
                        ::core::hash::Hash::hash(__self_3, state)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Blender {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<BlendMode>;
                let _: ::core::cmp::AssertParamIsEq<String>;
                let _: ::core::cmp::AssertParamIsEq<OrderedFloat>;
            }
        }
        impl bincode::Encode for Blender {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::BlendMode(field0) => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Sksl(field0) => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(field0, encoder)?;
                    }
                    Self::Arithmetic { k1, k2, k3, k4 } => {
                        bincode::Encode::encode(&2u32, encoder)?;
                        bincode::Encode::encode(k1, encoder)?;
                        bincode::Encode::encode(k2, encoder)?;
                        bincode::Encode::encode(k3, encoder)?;
                        bincode::Encode::encode(k4, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for Blender {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::BlendMode(bincode::Decode::decode(decoder)?)),
                    1u32 => Ok(Self::Sksl(bincode::Decode::decode(decoder)?)),
                    2u32 => Ok(Self::Arithmetic {
                        k1: bincode::Decode::decode(decoder)?,
                        k2: bincode::Decode::decode(decoder)?,
                        k3: bincode::Decode::decode(decoder)?,
                        k4: bincode::Decode::decode(decoder)?,
                    }),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for Blender {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::BlendMode { field0 } => {
                        buffer.write_string("BlendMode");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Sksl { field0 } => {
                        buffer.write_string("Sksl");
                        let field_bytes = Serialize::serialize(field0);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Arithmetic { k1, k2, k3, k4 } => {
                        buffer.write_string("Arithmetic");
                        buffer.write_string("k1");
                        let field_bytes = Serialize::serialize(k1);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("k2");
                        let field_bytes = Serialize::serialize(k2);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("k3");
                        let field_bytes = Serialize::serialize(k3);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("k4");
                        let field_bytes = Serialize::serialize(k4);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for Blender {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "BlendMode" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::BlendMode(field0))
                    }
                    "Sksl" => {
                        let field0 = { Deserialize::deserialize(buf)? };
                        Ok(Self::Sksl(field0))
                    }
                    "Arithmetic" => {
                        let field_name = buf.read_name("k1")?;
                        let k1 = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("k2")?;
                        let k2 = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("k3")?;
                        let k3 = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("k4")?;
                        let k4 = Deserialize::deserialize(buf)?;
                        Ok(Self::Arithmetic { k1, k2, k3, k4 })
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl Blender {
            /// Create a blender that implements the following:
            /// `k1 * src * dst + k2 * src + k3 * dst + k4`
            pub fn arithmetic(k1: f32, k2: f32, k3: f32, k4: f32) -> Self {
                Blender::Arithmetic {
                    k1: k1.into(),
                    k2: k2.into(),
                    k3: k3.into(),
                    k4: k4.into(),
                }
            }
        }
        impl From<BlendMode> for Blender {
            fn from(value: BlendMode) -> Self {
                Blender::BlendMode(value)
            }
        }
        impl From<&Blender> for NativeBlender {
            fn from(blender: &Blender) -> Self {
                let skia_blender = match blender {
                    Blender::BlendMode(blend_mode) => {
                        skia_safe::BlendMode::from(*blend_mode).into()
                    }
                    Blender::Sksl(sksl) => skia_safe::RuntimeEffect::make_for_blender(sksl, None)
                        .unwrap()
                        .make_blender(skia_safe::Data::new_empty(), None)
                        .unwrap(),
                    Blender::Arithmetic { k1, k2, k3, k4 } => skia_safe::Blender::arithmetic(
                        k1.as_f32(),
                        k2.as_f32(),
                        k3.as_f32(),
                        k4.as_f32(),
                        false,
                    )
                    .unwrap(),
                };
                NativeBlender { skia_blender }
            }
        }
        impl From<&Blender> for skia_safe::Blender {
            fn from(blender: &Blender) -> Self {
                let native_blender = NativeBlender::get(blender);
                native_blender.skia().clone()
            }
        }
    }
    mod codes {
        use std::str::FromStr;
        #[repr(u8)]
        pub enum Code {
            Escape = 0,
            Digit1,
            Digit2,
            Digit3,
            Digit4,
            Digit5,
            Digit6,
            Digit7,
            Digit8,
            Digit9,
            Digit0,
            Minus,
            Equal,
            Backspace,
            Tab,
            KeyQ,
            KeyW,
            KeyE,
            KeyR,
            KeyT,
            KeyY,
            KeyU,
            KeyI,
            KeyO,
            KeyP,
            BracketLeft,
            BracketRight,
            Enter,
            ControlLeft,
            KeyA,
            KeyS,
            KeyD,
            KeyF,
            KeyG,
            KeyH,
            KeyJ,
            KeyK,
            KeyL,
            Semicolon,
            Quote,
            Backquote,
            ShiftLeft,
            Backslash,
            KeyZ,
            KeyX,
            KeyC,
            KeyV,
            KeyB,
            KeyN,
            KeyM,
            Comma,
            Period,
            Slash,
            ShiftRight,
            AltLeft,
            Space,
            CapsLock,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            Pause,
            ScrollLock,
            IntlBackslash,
            F11,
            F12,
            ControlRight,
            PrintScreen,
            AltRight,
            NumLock,
            Home,
            ArrowUp,
            PageUp,
            ArrowLeft,
            ArrowRight,
            End,
            ArrowDown,
            PageDown,
            Insert,
            Delete,
            ContextMenu,
            IntlRo,
            IntlYen,
            SuperLeft,
            SuperRight,
            Convert,
            KanaMode,
            Lang1,
            Lang2,
            Lang3,
            Lang4,
            Lang5,
            NonConvert,
            Help,
            Numpad0,
            Numpad1,
            Numpad2,
            Numpad3,
            Numpad4,
            Numpad5,
            Numpad6,
            Numpad7,
            Numpad8,
            Numpad9,
            NumpadAdd,
            NumpadBackspace,
            NumpadClear,
            NumpadClearEntry,
            NumpadComma,
            NumpadDecimal,
            NumpadDivide,
            NumpadEnter,
            NumpadEqual,
            NumpadHash,
            NumpadMemoryAdd,
            NumpadMemoryClear,
            NumpadMemoryRecall,
            NumpadMemoryStore,
            NumpadMemorySubtract,
            NumpadMultiply,
            NumpadParenLeft,
            NumpadParenRight,
            NumpadStar,
            NumpadSubtract,
            Fn,
            FnLock,
            BrowserBack,
            BrowserFavorites,
            BrowserForward,
            BrowserHome,
            BrowserRefresh,
            BrowserSearch,
            BrowserStop,
            Eject,
            LaunchApp1,
            LaunchApp2,
            LaunchMail,
            MediaPlayPause,
            MediaSelect,
            MediaStop,
            MediaTrackNext,
            MediaTrackPrevious,
            Power,
            Sleep,
            AudioVolumeDown,
            AudioVolumeMute,
            AudioVolumeUp,
            WakeUp,
            Meta,
            Hyper,
            Turbo,
            Abort,
            Resume,
            Suspend,
            Again,
            Copy,
            Cut,
            Find,
            Open,
            Paste,
            Props,
            Select,
            Undo,
            Hiragana,
            Katakana,
            F13,
            F14,
            F15,
            F16,
            F17,
            F18,
            F19,
            F20,
            F21,
            F22,
            F23,
            F24,
            F25,
            F26,
            F27,
            F28,
            F29,
            F30,
            F31,
            F32,
            F33,
            F34,
            F35,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Code {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Code::Escape => "Escape",
                        Code::Digit1 => "Digit1",
                        Code::Digit2 => "Digit2",
                        Code::Digit3 => "Digit3",
                        Code::Digit4 => "Digit4",
                        Code::Digit5 => "Digit5",
                        Code::Digit6 => "Digit6",
                        Code::Digit7 => "Digit7",
                        Code::Digit8 => "Digit8",
                        Code::Digit9 => "Digit9",
                        Code::Digit0 => "Digit0",
                        Code::Minus => "Minus",
                        Code::Equal => "Equal",
                        Code::Backspace => "Backspace",
                        Code::Tab => "Tab",
                        Code::KeyQ => "KeyQ",
                        Code::KeyW => "KeyW",
                        Code::KeyE => "KeyE",
                        Code::KeyR => "KeyR",
                        Code::KeyT => "KeyT",
                        Code::KeyY => "KeyY",
                        Code::KeyU => "KeyU",
                        Code::KeyI => "KeyI",
                        Code::KeyO => "KeyO",
                        Code::KeyP => "KeyP",
                        Code::BracketLeft => "BracketLeft",
                        Code::BracketRight => "BracketRight",
                        Code::Enter => "Enter",
                        Code::ControlLeft => "ControlLeft",
                        Code::KeyA => "KeyA",
                        Code::KeyS => "KeyS",
                        Code::KeyD => "KeyD",
                        Code::KeyF => "KeyF",
                        Code::KeyG => "KeyG",
                        Code::KeyH => "KeyH",
                        Code::KeyJ => "KeyJ",
                        Code::KeyK => "KeyK",
                        Code::KeyL => "KeyL",
                        Code::Semicolon => "Semicolon",
                        Code::Quote => "Quote",
                        Code::Backquote => "Backquote",
                        Code::ShiftLeft => "ShiftLeft",
                        Code::Backslash => "Backslash",
                        Code::KeyZ => "KeyZ",
                        Code::KeyX => "KeyX",
                        Code::KeyC => "KeyC",
                        Code::KeyV => "KeyV",
                        Code::KeyB => "KeyB",
                        Code::KeyN => "KeyN",
                        Code::KeyM => "KeyM",
                        Code::Comma => "Comma",
                        Code::Period => "Period",
                        Code::Slash => "Slash",
                        Code::ShiftRight => "ShiftRight",
                        Code::AltLeft => "AltLeft",
                        Code::Space => "Space",
                        Code::CapsLock => "CapsLock",
                        Code::F1 => "F1",
                        Code::F2 => "F2",
                        Code::F3 => "F3",
                        Code::F4 => "F4",
                        Code::F5 => "F5",
                        Code::F6 => "F6",
                        Code::F7 => "F7",
                        Code::F8 => "F8",
                        Code::F9 => "F9",
                        Code::F10 => "F10",
                        Code::Pause => "Pause",
                        Code::ScrollLock => "ScrollLock",
                        Code::IntlBackslash => "IntlBackslash",
                        Code::F11 => "F11",
                        Code::F12 => "F12",
                        Code::ControlRight => "ControlRight",
                        Code::PrintScreen => "PrintScreen",
                        Code::AltRight => "AltRight",
                        Code::NumLock => "NumLock",
                        Code::Home => "Home",
                        Code::ArrowUp => "ArrowUp",
                        Code::PageUp => "PageUp",
                        Code::ArrowLeft => "ArrowLeft",
                        Code::ArrowRight => "ArrowRight",
                        Code::End => "End",
                        Code::ArrowDown => "ArrowDown",
                        Code::PageDown => "PageDown",
                        Code::Insert => "Insert",
                        Code::Delete => "Delete",
                        Code::ContextMenu => "ContextMenu",
                        Code::IntlRo => "IntlRo",
                        Code::IntlYen => "IntlYen",
                        Code::SuperLeft => "SuperLeft",
                        Code::SuperRight => "SuperRight",
                        Code::Convert => "Convert",
                        Code::KanaMode => "KanaMode",
                        Code::Lang1 => "Lang1",
                        Code::Lang2 => "Lang2",
                        Code::Lang3 => "Lang3",
                        Code::Lang4 => "Lang4",
                        Code::Lang5 => "Lang5",
                        Code::NonConvert => "NonConvert",
                        Code::Help => "Help",
                        Code::Numpad0 => "Numpad0",
                        Code::Numpad1 => "Numpad1",
                        Code::Numpad2 => "Numpad2",
                        Code::Numpad3 => "Numpad3",
                        Code::Numpad4 => "Numpad4",
                        Code::Numpad5 => "Numpad5",
                        Code::Numpad6 => "Numpad6",
                        Code::Numpad7 => "Numpad7",
                        Code::Numpad8 => "Numpad8",
                        Code::Numpad9 => "Numpad9",
                        Code::NumpadAdd => "NumpadAdd",
                        Code::NumpadBackspace => "NumpadBackspace",
                        Code::NumpadClear => "NumpadClear",
                        Code::NumpadClearEntry => "NumpadClearEntry",
                        Code::NumpadComma => "NumpadComma",
                        Code::NumpadDecimal => "NumpadDecimal",
                        Code::NumpadDivide => "NumpadDivide",
                        Code::NumpadEnter => "NumpadEnter",
                        Code::NumpadEqual => "NumpadEqual",
                        Code::NumpadHash => "NumpadHash",
                        Code::NumpadMemoryAdd => "NumpadMemoryAdd",
                        Code::NumpadMemoryClear => "NumpadMemoryClear",
                        Code::NumpadMemoryRecall => "NumpadMemoryRecall",
                        Code::NumpadMemoryStore => "NumpadMemoryStore",
                        Code::NumpadMemorySubtract => "NumpadMemorySubtract",
                        Code::NumpadMultiply => "NumpadMultiply",
                        Code::NumpadParenLeft => "NumpadParenLeft",
                        Code::NumpadParenRight => "NumpadParenRight",
                        Code::NumpadStar => "NumpadStar",
                        Code::NumpadSubtract => "NumpadSubtract",
                        Code::Fn => "Fn",
                        Code::FnLock => "FnLock",
                        Code::BrowserBack => "BrowserBack",
                        Code::BrowserFavorites => "BrowserFavorites",
                        Code::BrowserForward => "BrowserForward",
                        Code::BrowserHome => "BrowserHome",
                        Code::BrowserRefresh => "BrowserRefresh",
                        Code::BrowserSearch => "BrowserSearch",
                        Code::BrowserStop => "BrowserStop",
                        Code::Eject => "Eject",
                        Code::LaunchApp1 => "LaunchApp1",
                        Code::LaunchApp2 => "LaunchApp2",
                        Code::LaunchMail => "LaunchMail",
                        Code::MediaPlayPause => "MediaPlayPause",
                        Code::MediaSelect => "MediaSelect",
                        Code::MediaStop => "MediaStop",
                        Code::MediaTrackNext => "MediaTrackNext",
                        Code::MediaTrackPrevious => "MediaTrackPrevious",
                        Code::Power => "Power",
                        Code::Sleep => "Sleep",
                        Code::AudioVolumeDown => "AudioVolumeDown",
                        Code::AudioVolumeMute => "AudioVolumeMute",
                        Code::AudioVolumeUp => "AudioVolumeUp",
                        Code::WakeUp => "WakeUp",
                        Code::Meta => "Meta",
                        Code::Hyper => "Hyper",
                        Code::Turbo => "Turbo",
                        Code::Abort => "Abort",
                        Code::Resume => "Resume",
                        Code::Suspend => "Suspend",
                        Code::Again => "Again",
                        Code::Copy => "Copy",
                        Code::Cut => "Cut",
                        Code::Find => "Find",
                        Code::Open => "Open",
                        Code::Paste => "Paste",
                        Code::Props => "Props",
                        Code::Select => "Select",
                        Code::Undo => "Undo",
                        Code::Hiragana => "Hiragana",
                        Code::Katakana => "Katakana",
                        Code::F13 => "F13",
                        Code::F14 => "F14",
                        Code::F15 => "F15",
                        Code::F16 => "F16",
                        Code::F17 => "F17",
                        Code::F18 => "F18",
                        Code::F19 => "F19",
                        Code::F20 => "F20",
                        Code::F21 => "F21",
                        Code::F22 => "F22",
                        Code::F23 => "F23",
                        Code::F24 => "F24",
                        Code::F25 => "F25",
                        Code::F26 => "F26",
                        Code::F27 => "F27",
                        Code::F28 => "F28",
                        Code::F29 => "F29",
                        Code::F30 => "F30",
                        Code::F31 => "F31",
                        Code::F32 => "F32",
                        Code::F33 => "F33",
                        Code::F34 => "F34",
                        Code::F35 => "F35",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Code {
            #[inline]
            fn clone(&self) -> Code {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Code {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Code {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Code {
            #[inline]
            fn eq(&self, other: &Code) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Code {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Code {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        impl FromStr for Code {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "Escape" => Ok(Code::Escape),
                    "Digit1" => Ok(Code::Digit1),
                    "Digit2" => Ok(Code::Digit2),
                    "Digit3" => Ok(Code::Digit3),
                    "Digit4" => Ok(Code::Digit4),
                    "Digit5" => Ok(Code::Digit5),
                    "Digit6" => Ok(Code::Digit6),
                    "Digit7" => Ok(Code::Digit7),
                    "Digit8" => Ok(Code::Digit8),
                    "Digit9" => Ok(Code::Digit9),
                    "Digit0" => Ok(Code::Digit0),
                    "Minus" => Ok(Code::Minus),
                    "Equal" => Ok(Code::Equal),
                    "Backspace" => Ok(Code::Backspace),
                    "Tab" => Ok(Code::Tab),
                    "KeyQ" => Ok(Code::KeyQ),
                    "KeyW" => Ok(Code::KeyW),
                    "KeyE" => Ok(Code::KeyE),
                    "KeyR" => Ok(Code::KeyR),
                    "KeyT" => Ok(Code::KeyT),
                    "KeyY" => Ok(Code::KeyY),
                    "KeyU" => Ok(Code::KeyU),
                    "KeyI" => Ok(Code::KeyI),
                    "KeyO" => Ok(Code::KeyO),
                    "KeyP" => Ok(Code::KeyP),
                    "BracketLeft" => Ok(Code::BracketLeft),
                    "BracketRight" => Ok(Code::BracketRight),
                    "Enter" => Ok(Code::Enter),
                    "ControlLeft" => Ok(Code::ControlLeft),
                    "KeyA" => Ok(Code::KeyA),
                    "KeyS" => Ok(Code::KeyS),
                    "KeyD" => Ok(Code::KeyD),
                    "KeyF" => Ok(Code::KeyF),
                    "KeyG" => Ok(Code::KeyG),
                    "KeyH" => Ok(Code::KeyH),
                    "KeyJ" => Ok(Code::KeyJ),
                    "KeyK" => Ok(Code::KeyK),
                    "KeyL" => Ok(Code::KeyL),
                    "Semicolon" => Ok(Code::Semicolon),
                    "Quote" => Ok(Code::Quote),
                    "Backquote" => Ok(Code::Backquote),
                    "ShiftLeft" => Ok(Code::ShiftLeft),
                    "Backslash" => Ok(Code::Backslash),
                    "KeyZ" => Ok(Code::KeyZ),
                    "KeyX" => Ok(Code::KeyX),
                    "KeyC" => Ok(Code::KeyC),
                    "KeyV" => Ok(Code::KeyV),
                    "KeyB" => Ok(Code::KeyB),
                    "KeyN" => Ok(Code::KeyN),
                    "KeyM" => Ok(Code::KeyM),
                    "Comma" => Ok(Code::Comma),
                    "Period" => Ok(Code::Period),
                    "Slash" => Ok(Code::Slash),
                    "ShiftRight" => Ok(Code::ShiftRight),
                    "AltLeft" => Ok(Code::AltLeft),
                    "Space" => Ok(Code::Space),
                    "CapsLock" => Ok(Code::CapsLock),
                    "F1" => Ok(Code::F1),
                    "F2" => Ok(Code::F2),
                    "F3" => Ok(Code::F3),
                    "F4" => Ok(Code::F4),
                    "F5" => Ok(Code::F5),
                    "F6" => Ok(Code::F6),
                    "F7" => Ok(Code::F7),
                    "F8" => Ok(Code::F8),
                    "F9" => Ok(Code::F9),
                    "F10" => Ok(Code::F10),
                    "Pause" => Ok(Code::Pause),
                    "ScrollLock" => Ok(Code::ScrollLock),
                    "IntlBackslash" => Ok(Code::IntlBackslash),
                    "F11" => Ok(Code::F11),
                    "F12" => Ok(Code::F12),
                    "ControlRight" => Ok(Code::ControlRight),
                    "PrintScreen" => Ok(Code::PrintScreen),
                    "AltRight" => Ok(Code::AltRight),
                    "NumLock" => Ok(Code::NumLock),
                    "Home" => Ok(Code::Home),
                    "ArrowUp" => Ok(Code::ArrowUp),
                    "PageUp" => Ok(Code::PageUp),
                    "ArrowLeft" => Ok(Code::ArrowLeft),
                    "ArrowRight" => Ok(Code::ArrowRight),
                    "End" => Ok(Code::End),
                    "ArrowDown" => Ok(Code::ArrowDown),
                    "PageDown" => Ok(Code::PageDown),
                    "Insert" => Ok(Code::Insert),
                    "Delete" => Ok(Code::Delete),
                    "ContextMenu" => Ok(Code::ContextMenu),
                    _ => Err(::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Unknown code: {0}", s))
                    })),
                }
            }
        }
        impl std::fmt::Display for Code {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let str = match self {
                    Code::Escape => "Escape",
                    Code::Digit1 => "Digit1",
                    Code::Digit2 => "Digit2",
                    Code::Digit3 => "Digit3",
                    Code::Digit4 => "Digit4",
                    Code::Digit5 => "Digit5",
                    Code::Digit6 => "Digit6",
                    Code::Digit7 => "Digit7",
                    Code::Digit8 => "Digit8",
                    Code::Digit9 => "Digit9",
                    Code::Digit0 => "Digit0",
                    Code::Minus => "Minus",
                    Code::Equal => "Equal",
                    Code::Backspace => "Backspace",
                    Code::Tab => "Tab",
                    Code::KeyQ => "KeyQ",
                    Code::KeyW => "KeyW",
                    Code::KeyE => "KeyE",
                    Code::KeyR => "KeyR",
                    Code::KeyT => "KeyT",
                    Code::KeyY => "KeyY",
                    Code::KeyU => "KeyU",
                    Code::KeyI => "KeyI",
                    Code::KeyO => "KeyO",
                    Code::KeyP => "KeyP",
                    Code::BracketLeft => "BracketLeft",
                    Code::BracketRight => "BracketRight",
                    Code::Enter => "Enter",
                    Code::ControlLeft => "ControlLeft",
                    Code::KeyA => "KeyA",
                    Code::KeyS => "KeyS",
                    Code::KeyD => "KeyD",
                    Code::KeyF => "KeyF",
                    Code::KeyG => "KeyG",
                    Code::KeyH => "KeyH",
                    Code::KeyJ => "KeyJ",
                    Code::KeyK => "KeyK",
                    Code::KeyL => "KeyL",
                    Code::Semicolon => "Semicolon",
                    Code::Quote => "Quote",
                    Code::Backquote => "Backquote",
                    Code::ShiftLeft => "ShiftLeft",
                    Code::Backslash => "Backslash",
                    Code::KeyZ => "KeyZ",
                    Code::KeyX => "KeyX",
                    Code::KeyC => "KeyC",
                    Code::KeyV => "KeyV",
                    Code::KeyB => "KeyB",
                    Code::KeyN => "KeyN",
                    Code::KeyM => "KeyM",
                    Code::Comma => "Comma",
                    Code::Period => "Period",
                    Code::Slash => "Slash",
                    Code::ShiftRight => "ShiftRight",
                    Code::AltLeft => "AltLeft",
                    Code::Space => "Space",
                    Code::CapsLock => "CapsLock",
                    Code::F1 => "F1",
                    Code::F2 => "F2",
                    Code::F3 => "F3",
                    Code::F4 => "F4",
                    Code::F5 => "F5",
                    Code::F6 => "F6",
                    Code::F7 => "F7",
                    Code::F8 => "F8",
                    Code::F9 => "F9",
                    Code::F10 => "F10",
                    Code::Pause => "Pause",
                    Code::ScrollLock => "ScrollLock",
                    Code::IntlBackslash => "IntlBackslash",
                    Code::F11 => "F11",
                    Code::F12 => "F12",
                    Code::ControlRight => "ControlRight",
                    Code::PrintScreen => "PrintScreen",
                    Code::AltRight => "AltRight",
                    Code::NumLock => "NumLock",
                    Code::Home => "Home",
                    Code::ArrowUp => "ArrowUp",
                    Code::PageUp => "PageUp",
                    Code::ArrowLeft => "ArrowLeft",
                    Code::ArrowRight => "ArrowRight",
                    Code::End => "End",
                    Code::ArrowDown => "ArrowDown",
                    Code::PageDown => "PageDown",
                    Code::Insert => "Insert",
                    Code::Delete => "Delete",
                    Code::ContextMenu => "ContextMenu",
                    Code::IntlRo => "IntlRo",
                    Code::IntlYen => "IntlYen",
                    Code::SuperLeft => "SuperLeft",
                    Code::SuperRight => "SuperRight",
                    Code::Convert => "Convert",
                    Code::KanaMode => "KanaMode",
                    Code::Lang1 => "Lang1",
                    Code::Lang2 => "Lang2",
                    Code::Lang3 => "Lang3",
                    Code::Lang4 => "Lang4",
                    Code::Lang5 => "Lang5",
                    Code::NonConvert => "NonConvert",
                    Code::Help => "Help",
                    Code::Numpad0 => "Numpad0",
                    Code::Numpad1 => "Numpad1",
                    Code::Numpad2 => "Numpad2",
                    Code::Numpad3 => "Numpad3",
                    Code::Numpad4 => "Numpad4",
                    Code::Numpad5 => "Numpad5",
                    Code::Numpad6 => "Numpad6",
                    Code::Numpad7 => "Numpad7",
                    Code::Numpad8 => "Numpad8",
                    Code::Numpad9 => "Numpad9",
                    Code::NumpadAdd => "NumpadAdd",
                    Code::NumpadBackspace => "NumpadBackspace",
                    Code::NumpadClear => "NumpadClear",
                    Code::NumpadClearEntry => "NumpadClearEntry",
                    Code::NumpadComma => "NumpadComma",
                    Code::NumpadDecimal => "NumpadDecimal",
                    Code::NumpadDivide => "NumpadDivide",
                    Code::NumpadEnter => "NumpadEnter",
                    Code::NumpadEqual => "NumpadEqual",
                    Code::NumpadHash => "NumpadHash",
                    Code::NumpadMemoryAdd => "NumpadMemoryAdd",
                    Code::NumpadMemoryClear => "NumpadMemoryClear",
                    Code::NumpadMemoryRecall => "NumpadMemoryRecall",
                    Code::NumpadMemoryStore => "NumpadMemoryStore",
                    Code::NumpadMemorySubtract => "NumpadMemorySubtract",
                    Code::NumpadMultiply => "NumpadMultiply",
                    Code::NumpadParenLeft => "NumpadParenLeft",
                    Code::NumpadParenRight => "NumpadParenRight",
                    Code::NumpadStar => "NumpadStar",
                    Code::NumpadSubtract => "NumpadSubtract",
                    Code::Fn => "Fn",
                    Code::FnLock => "FnLock",
                    Code::BrowserBack => "BrowserBack",
                    Code::BrowserFavorites => "BrowserFavorites",
                    Code::BrowserForward => "BrowserForward",
                    Code::BrowserHome => "BrowserHome",
                    Code::BrowserRefresh => "BrowserRefresh",
                    Code::BrowserSearch => "BrowserSearch",
                    Code::BrowserStop => "BrowserStop",
                    Code::Eject => "Eject",
                    Code::LaunchApp1 => "LaunchApp1",
                    Code::LaunchApp2 => "LaunchApp2",
                    Code::LaunchMail => "LaunchMail",
                    Code::MediaPlayPause => "MediaPlayPause",
                    Code::MediaSelect => "MediaSelect",
                    Code::MediaStop => "MediaStop",
                    Code::MediaTrackNext => "MediaTrackNext",
                    Code::MediaTrackPrevious => "MediaTrackPrevious",
                    Code::Power => "Power",
                    Code::Sleep => "Sleep",
                    Code::AudioVolumeDown => "AudioVolumeDown",
                    Code::AudioVolumeMute => "AudioVolumeMute",
                    Code::AudioVolumeUp => "AudioVolumeUp",
                    Code::WakeUp => "WakeUp",
                    Code::Meta => "Meta",
                    Code::Hyper => "Hyper",
                    Code::Turbo => "Turbo",
                    Code::Abort => "Abort",
                    Code::Resume => "Resume",
                    Code::Suspend => "Suspend",
                    Code::Again => "Again",
                    Code::Copy => "Copy",
                    Code::Cut => "Cut",
                    Code::Find => "Find",
                    Code::Open => "Open",
                    Code::Paste => "Paste",
                    Code::Props => "Props",
                    Code::Select => "Select",
                    Code::Undo => "Undo",
                    Code::Hiragana => "Hiragana",
                    Code::Katakana => "Katakana",
                    Code::F13 => "F13",
                    Code::F14 => "F14",
                    Code::F15 => "F15",
                    Code::F16 => "F16",
                    Code::F17 => "F17",
                    Code::F18 => "F18",
                    Code::F19 => "F19",
                    Code::F20 => "F20",
                    Code::F21 => "F21",
                    Code::F22 => "F22",
                    Code::F23 => "F23",
                    Code::F24 => "F24",
                    Code::F25 => "F25",
                    Code::F26 => "F26",
                    Code::F27 => "F27",
                    Code::F28 => "F28",
                    Code::F29 => "F29",
                    Code::F30 => "F30",
                    Code::F31 => "F31",
                    Code::F32 => "F32",
                    Code::F33 => "F33",
                    Code::F34 => "F34",
                    Code::F35 => "F35",
                };
                f.write_fmt(format_args!("{0}", str))
            }
        }
        impl TryFrom<u8> for Code {
            type Error = u8;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                const MAP: [Code; 194] = [
                    Code::Escape,
                    Code::Digit1,
                    Code::Digit2,
                    Code::Digit3,
                    Code::Digit4,
                    Code::Digit5,
                    Code::Digit6,
                    Code::Digit7,
                    Code::Digit8,
                    Code::Digit9,
                    Code::Digit0,
                    Code::Minus,
                    Code::Equal,
                    Code::Backspace,
                    Code::Tab,
                    Code::KeyQ,
                    Code::KeyW,
                    Code::KeyE,
                    Code::KeyR,
                    Code::KeyT,
                    Code::KeyY,
                    Code::KeyU,
                    Code::KeyI,
                    Code::KeyO,
                    Code::KeyP,
                    Code::BracketLeft,
                    Code::BracketRight,
                    Code::Enter,
                    Code::ControlLeft,
                    Code::KeyA,
                    Code::KeyS,
                    Code::KeyD,
                    Code::KeyF,
                    Code::KeyG,
                    Code::KeyH,
                    Code::KeyJ,
                    Code::KeyK,
                    Code::KeyL,
                    Code::Semicolon,
                    Code::Quote,
                    Code::Backquote,
                    Code::ShiftLeft,
                    Code::Backslash,
                    Code::KeyZ,
                    Code::KeyX,
                    Code::KeyC,
                    Code::KeyV,
                    Code::KeyB,
                    Code::KeyN,
                    Code::KeyM,
                    Code::Comma,
                    Code::Period,
                    Code::Slash,
                    Code::ShiftRight,
                    Code::AltLeft,
                    Code::Space,
                    Code::CapsLock,
                    Code::F1,
                    Code::F2,
                    Code::F3,
                    Code::F4,
                    Code::F5,
                    Code::F6,
                    Code::F7,
                    Code::F8,
                    Code::F9,
                    Code::F10,
                    Code::Pause,
                    Code::ScrollLock,
                    Code::IntlBackslash,
                    Code::F11,
                    Code::F12,
                    Code::ControlRight,
                    Code::PrintScreen,
                    Code::AltRight,
                    Code::NumLock,
                    Code::Home,
                    Code::ArrowUp,
                    Code::PageUp,
                    Code::ArrowLeft,
                    Code::ArrowRight,
                    Code::End,
                    Code::ArrowDown,
                    Code::PageDown,
                    Code::Insert,
                    Code::Delete,
                    Code::ContextMenu,
                    Code::IntlRo,
                    Code::IntlYen,
                    Code::SuperLeft,
                    Code::SuperRight,
                    Code::Convert,
                    Code::KanaMode,
                    Code::Lang1,
                    Code::Lang2,
                    Code::Lang3,
                    Code::Lang4,
                    Code::Lang5,
                    Code::NonConvert,
                    Code::Help,
                    Code::Numpad0,
                    Code::Numpad1,
                    Code::Numpad2,
                    Code::Numpad3,
                    Code::Numpad4,
                    Code::Numpad5,
                    Code::Numpad6,
                    Code::Numpad7,
                    Code::Numpad8,
                    Code::Numpad9,
                    Code::NumpadAdd,
                    Code::NumpadBackspace,
                    Code::NumpadClear,
                    Code::NumpadClearEntry,
                    Code::NumpadComma,
                    Code::NumpadDecimal,
                    Code::NumpadDivide,
                    Code::NumpadEnter,
                    Code::NumpadEqual,
                    Code::NumpadHash,
                    Code::NumpadMemoryAdd,
                    Code::NumpadMemoryClear,
                    Code::NumpadMemoryRecall,
                    Code::NumpadMemoryStore,
                    Code::NumpadMemorySubtract,
                    Code::NumpadMultiply,
                    Code::NumpadParenLeft,
                    Code::NumpadParenRight,
                    Code::NumpadStar,
                    Code::NumpadSubtract,
                    Code::Fn,
                    Code::FnLock,
                    Code::BrowserBack,
                    Code::BrowserFavorites,
                    Code::BrowserForward,
                    Code::BrowserHome,
                    Code::BrowserRefresh,
                    Code::BrowserSearch,
                    Code::BrowserStop,
                    Code::Eject,
                    Code::LaunchApp1,
                    Code::LaunchApp2,
                    Code::LaunchMail,
                    Code::MediaPlayPause,
                    Code::MediaSelect,
                    Code::MediaStop,
                    Code::MediaTrackNext,
                    Code::MediaTrackPrevious,
                    Code::Power,
                    Code::Sleep,
                    Code::AudioVolumeDown,
                    Code::AudioVolumeMute,
                    Code::AudioVolumeUp,
                    Code::WakeUp,
                    Code::Meta,
                    Code::Hyper,
                    Code::Turbo,
                    Code::Abort,
                    Code::Resume,
                    Code::Suspend,
                    Code::Again,
                    Code::Copy,
                    Code::Cut,
                    Code::Find,
                    Code::Open,
                    Code::Paste,
                    Code::Props,
                    Code::Select,
                    Code::Undo,
                    Code::Hiragana,
                    Code::Katakana,
                    Code::F13,
                    Code::F14,
                    Code::F15,
                    Code::F16,
                    Code::F17,
                    Code::F18,
                    Code::F19,
                    Code::F20,
                    Code::F21,
                    Code::F22,
                    Code::F23,
                    Code::F24,
                    Code::F25,
                    Code::F26,
                    Code::F27,
                    Code::F28,
                    Code::F29,
                    Code::F30,
                    Code::F31,
                    Code::F32,
                    Code::F33,
                    Code::F34,
                    Code::F35,
                ];
                if value < MAP.len() as u8 {
                    Ok(MAP[value as usize])
                } else {
                    Err(value)
                }
            }
        }
    }
    mod color_filter {
        use crate::*;
        pub enum ColorFilter {
            Blend {
                color: Color,
                blend_mode: BlendMode,
            },
            ScaleMatrix {
                r: OrderedFloat,
                g: OrderedFloat,
                b: OrderedFloat,
                a: OrderedFloat,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ColorFilter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ColorFilter::Blend {
                        color: __self_0,
                        blend_mode: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Blend",
                        "color",
                        __self_0,
                        "blend_mode",
                        &__self_1,
                    ),
                    ColorFilter::ScaleMatrix {
                        r: __self_0,
                        g: __self_1,
                        b: __self_2,
                        a: __self_3,
                    } => ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "ScaleMatrix",
                        "r",
                        __self_0,
                        "g",
                        __self_1,
                        "b",
                        __self_2,
                        "a",
                        &__self_3,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ColorFilter {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ColorFilter {
            #[inline]
            fn eq(&self, other: &ColorFilter) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ColorFilter::Blend {
                                color: __self_0,
                                blend_mode: __self_1,
                            },
                            ColorFilter::Blend {
                                color: __arg1_0,
                                blend_mode: __arg1_1,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        (
                            ColorFilter::ScaleMatrix {
                                r: __self_0,
                                g: __self_1,
                                b: __self_2,
                                a: __self_3,
                            },
                            ColorFilter::ScaleMatrix {
                                r: __arg1_0,
                                g: __arg1_1,
                                b: __arg1_2,
                                a: __arg1_3,
                            },
                        ) => {
                            __self_0 == __arg1_0
                                && __self_1 == __arg1_1
                                && __self_2 == __arg1_2
                                && __self_3 == __arg1_3
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ColorFilter {
            #[inline]
            fn clone(&self) -> ColorFilter {
                let _: ::core::clone::AssertParamIsClone<Color>;
                let _: ::core::clone::AssertParamIsClone<BlendMode>;
                let _: ::core::clone::AssertParamIsClone<OrderedFloat>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ColorFilter {}
        #[automatically_derived]
        impl ::core::hash::Hash for ColorFilter {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    ColorFilter::Blend {
                        color: __self_0,
                        blend_mode: __self_1,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state)
                    }
                    ColorFilter::ScaleMatrix {
                        r: __self_0,
                        g: __self_1,
                        b: __self_2,
                        a: __self_3,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state);
                        ::core::hash::Hash::hash(__self_3, state)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ColorFilter {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Color>;
                let _: ::core::cmp::AssertParamIsEq<BlendMode>;
                let _: ::core::cmp::AssertParamIsEq<OrderedFloat>;
            }
        }
        impl bincode::Encode for ColorFilter {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Blend { color, blend_mode } => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(color, encoder)?;
                        bincode::Encode::encode(blend_mode, encoder)?;
                    }
                    Self::ScaleMatrix { r, g, b, a } => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(r, encoder)?;
                        bincode::Encode::encode(g, encoder)?;
                        bincode::Encode::encode(b, encoder)?;
                        bincode::Encode::encode(a, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for ColorFilter {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Blend {
                        color: bincode::Decode::decode(decoder)?,
                        blend_mode: bincode::Decode::decode(decoder)?,
                    }),
                    1u32 => Ok(Self::ScaleMatrix {
                        r: bincode::Decode::decode(decoder)?,
                        g: bincode::Decode::decode(decoder)?,
                        b: bincode::Decode::decode(decoder)?,
                        a: bincode::Decode::decode(decoder)?,
                    }),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 1u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for ColorFilter {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Blend { color, blend_mode } => {
                        buffer.write_string("Blend");
                        buffer.write_string("color");
                        let field_bytes = Serialize::serialize(color);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("blend_mode");
                        let field_bytes = Serialize::serialize(blend_mode);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::ScaleMatrix { r, g, b, a } => {
                        buffer.write_string("ScaleMatrix");
                        buffer.write_string("r");
                        let field_bytes = Serialize::serialize(r);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("g");
                        let field_bytes = Serialize::serialize(g);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("b");
                        let field_bytes = Serialize::serialize(b);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("a");
                        let field_bytes = Serialize::serialize(a);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for ColorFilter {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Blend" => {
                        let field_name = buf.read_name("color")?;
                        let color = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("blend_mode")?;
                        let blend_mode = Deserialize::deserialize(buf)?;
                        Ok(Self::Blend { color, blend_mode })
                    }
                    "ScaleMatrix" => {
                        let field_name = buf.read_name("r")?;
                        let r = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("g")?;
                        let g = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("b")?;
                        let b = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("a")?;
                        let a = Deserialize::deserialize(buf)?;
                        Ok(Self::ScaleMatrix { r, g, b, a })
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl ColorFilter {
            pub fn scale_matrix(r: f32, g: f32, b: f32, a: f32) -> Self {
                ColorFilter::ScaleMatrix {
                    r: r.into(),
                    g: g.into(),
                    b: b.into(),
                    a: a.into(),
                }
            }
        }
        impl From<&ColorFilter> for NativeColorFilter {
            fn from(value: &ColorFilter) -> Self {
                match *value {
                    ColorFilter::Blend { color, blend_mode } => NativeColorFilter {
                        skia_color_filter: skia_safe::color_filters::blend(
                            color,
                            blend_mode.into(),
                        )
                        .unwrap(),
                    },
                    ColorFilter::ScaleMatrix { r, g, b, a } => {
                        let mut color_matrix = skia_safe::ColorMatrix::default();
                        color_matrix.set_scale(*r, *b, *g, Some(*a));
                        let skia_color_filter =
                            skia_safe::color_filters::matrix(&color_matrix, None);
                        NativeColorFilter { skia_color_filter }
                    }
                }
            }
        }
    }
    mod font {
        use crate::*;
        pub struct Font {
            pub size: IntPx,
            pub name: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Font {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Font",
                    "size",
                    &self.size,
                    "name",
                    &&self.name,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Font {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Font {
            #[inline]
            fn eq(&self, other: &Font) -> bool {
                self.size == other.size && self.name == other.name
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Font {
            #[inline]
            fn clone(&self) -> Font {
                Font {
                    size: ::core::clone::Clone::clone(&self.size),
                    name: ::core::clone::Clone::clone(&self.name),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Font {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<IntPx>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Font {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.size, state);
                ::core::hash::Hash::hash(&self.name, state)
            }
        }
        impl bincode::Encode for Font {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.size, encoder)?;
                bincode::Encode::encode(&self.name, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for Font {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    size: bincode::Decode::decode(decoder)?,
                    name: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for Font {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("size");
                let field_bytes = Serialize::serialize(&self.size);
                buffer.put_slice(&field_bytes);
                buffer.write_string("name");
                let field_bytes = Serialize::serialize(&self.name);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for Font {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("size")?;
                let size = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("name")?;
                let name = Deserialize::deserialize(buf)?;
                Ok(Self { size, name })
            }
        }
        impl Font {
            pub fn groups(&self, text: &str, paint: &Paint) -> Vec<GlyphGroup> {
                let Some(native_font) = NativeFont::get(self) else {
                    return ::alloc::vec::Vec::new();
                };
                let glyph_ids = native_font.glyph_ids(text);
                let glyph_widths = native_font.glyph_widths(glyph_ids.clone(), paint);
                let glyphs = glyph_ids
                    .into_iter()
                    .zip(glyph_widths)
                    .map(|(id, width)| Glyph { id, width })
                    .collect::<Vec<_>>();
                let width = glyphs.iter().map(|glyph| glyph.width).sum();
                <[_]>::into_vec(::alloc::boxed::box_new([GlyphGroup {
                    font: self.clone(),
                    glyphs,
                    width,
                }]))
            }
            pub fn width(&self, text: &str, paint: &Paint) -> Px {
                self.groups(text, paint)
                    .into_iter()
                    .map(|group| group.width)
                    .sum()
            }
            pub fn widths(&self, text: &str, paint: &Paint) -> Vec<Px> {
                self.groups(text, paint)
                    .into_iter()
                    .flat_map(|group| group.glyphs.into_iter().map(|glyph| glyph.width))
                    .collect()
            }
            pub fn font_metrics(&self) -> FontMetrics {
                match NativeFont::get(self) {
                    Some(font) => font.metrics,
                    None => FontMetrics::default(),
                }
            }
            pub fn bounds(&self, text: &str, paint: &Paint) -> Vec<Rect<Px>> {
                let Some(native_font) = NativeFont::get(self) else {
                    return ::alloc::vec::Vec::new();
                };
                let glyph_ids = native_font.glyph_ids(text);
                native_font.glyph_bounds(glyph_ids.clone(), paint)
            }
            pub fn bound(&self, text: &str, paint: &Paint) -> Rect<Px> {
                self.bounds(text, paint)
                    .into_iter()
                    .fold(Rect::default(), |a, b| {
                        a.get_minimum_rectangle_containing(b)
                    })
            }
        }
    }
    mod image {
        use super::*;
        use crate::*;
        use bytes::*;
        use std::{
            collections::BTreeMap,
            fmt::Debug,
            hash::Hash,
            sync::{Arc, OnceLock},
        };
        pub struct Image {
            id: usize,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Image {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "Image", "id", &&self.id)
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Image {
            #[inline]
            fn clone(&self) -> Image {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Image {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Image {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Image {
            #[inline]
            fn eq(&self, other: &Image) -> bool {
                self.id == other.id
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Image {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Image {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.id, state)
            }
        }
        impl bincode::Encode for Image {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.id, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for Image {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    id: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for Image {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("id");
                let field_bytes = Serialize::serialize(&self.id);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for Image {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("id")?;
                let id = Deserialize::deserialize(buf)?;
                Ok(Self { id })
            }
        }
        impl Image {
            pub const STANDARD_CURSOR_SPRITE_SET: Image = Image { id: 100000 };
            pub const fn new(id: usize) -> Self {
                Self { id }
            }
            #[allow(dead_code)]
            pub fn get_default_shader(&self) -> Shader {
                Shader::Image {
                    src: *self,
                    tile_mode: Xy::single(TileMode::Clamp),
                }
            }
            pub fn info(&self) -> ImageInfo {
                IMAGE_INFOS.with(|image_infos| {
                    image_infos
                        .get_or_init(|| {
                            let image_count = unsafe { _get_image_count() };
                            let mut image_infos = BTreeMap::new();
                            let image_info_size = 14;
                            let mut buffer =
                                ::alloc::vec::from_elem(0u8, image_count * image_info_size);
                            unsafe { _get_image_infos(buffer.as_mut_ptr()) };
                            let mut buffer_reader: &[u8] = buffer.as_ref();
                            for _ in 0..image_count {
                                let id = buffer_reader.get_u32_le() as usize;
                                let alpha_type = AlphaType::from(buffer_reader.get_u8());
                                let color_type = ColorType::from(buffer_reader.get_u8());
                                let width = px(buffer_reader.get_u32_le() as f32);
                                let height = px(buffer_reader.get_u32_le() as f32);
                                image_infos.insert(
                                    id,
                                    ImageInfo {
                                        alpha_type,
                                        color_type,
                                        width,
                                        height,
                                    },
                                );
                            }
                            image_infos
                        })
                        .get(&self.id)
                        .cloned()
                        .unwrap_or_else(|| {
                            ::core::panicking::panic_fmt(format_args!(
                                "Image {0} not found",
                                self.id
                            ));
                        })
                })
            }
            pub(crate) fn skia_image(&self) -> Arc<skia_safe::Image> {
                IMAGES.get().unwrap().get(&self.id).unwrap().clone()
            }
        }
        const IMAGE_INFOS: ::std::thread::LocalKey<OnceLock<BTreeMap<usize, ImageInfo>>> = {
            const __INIT: OnceLock<BTreeMap<usize, ImageInfo>> = { OnceLock::new() };
            unsafe {
                ::std::thread::LocalKey::new(
                    const {
                        if ::std::mem::needs_drop::<OnceLock<BTreeMap<usize, ImageInfo>>>() {
                            |_| {
                                #[thread_local]
                                static VAL: ::std::thread::local_impl::EagerStorage<
                                    OnceLock<BTreeMap<usize, ImageInfo>>,
                                > = ::std::thread::local_impl::EagerStorage::new(__INIT);
                                VAL.get()
                            }
                        } else {
                            |_| {
                                #[thread_local]
                                static VAL: OnceLock<BTreeMap<usize, ImageInfo>> = __INIT;
                                &VAL
                            }
                        }
                    },
                )
            }
        };
        unsafe extern "C" {
            fn _get_image_count() -> usize;
            /**
             * image info layout
             * - id: u32
             * - alpha_type: u8
             * - color_type: u8
             * - width: u32
             * - height: u32
             */
            fn _get_image_infos(buffer: *mut u8);
        }
        pub struct ImageInfo {
            pub alpha_type: AlphaType,
            pub color_type: ColorType,
            pub height: Px,
            pub width: Px,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ImageInfo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ImageInfo",
                    "alpha_type",
                    &self.alpha_type,
                    "color_type",
                    &self.color_type,
                    "height",
                    &self.height,
                    "width",
                    &&self.width,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ImageInfo {
            #[inline]
            fn clone(&self) -> ImageInfo {
                let _: ::core::clone::AssertParamIsClone<AlphaType>;
                let _: ::core::clone::AssertParamIsClone<ColorType>;
                let _: ::core::clone::AssertParamIsClone<Px>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ImageInfo {}
        #[automatically_derived]
        impl ::core::hash::Hash for ImageInfo {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.alpha_type, state);
                ::core::hash::Hash::hash(&self.color_type, state);
                ::core::hash::Hash::hash(&self.height, state);
                ::core::hash::Hash::hash(&self.width, state)
            }
        }
        impl bincode::Encode for ImageInfo {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.alpha_type, encoder)?;
                bincode::Encode::encode(&self.color_type, encoder)?;
                bincode::Encode::encode(&self.height, encoder)?;
                bincode::Encode::encode(&self.width, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for ImageInfo {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    alpha_type: bincode::Decode::decode(decoder)?,
                    color_type: bincode::Decode::decode(decoder)?,
                    height: bincode::Decode::decode(decoder)?,
                    width: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for ImageInfo {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("alpha_type");
                let field_bytes = Serialize::serialize(&self.alpha_type);
                buffer.put_slice(&field_bytes);
                buffer.write_string("color_type");
                let field_bytes = Serialize::serialize(&self.color_type);
                buffer.put_slice(&field_bytes);
                buffer.write_string("height");
                let field_bytes = Serialize::serialize(&self.height);
                buffer.put_slice(&field_bytes);
                buffer.write_string("width");
                let field_bytes = Serialize::serialize(&self.width);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for ImageInfo {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("alpha_type")?;
                let alpha_type = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("color_type")?;
                let color_type = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("height")?;
                let height = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("width")?;
                let width = Deserialize::deserialize(buf)?;
                Ok(Self {
                    alpha_type,
                    color_type,
                    height,
                    width,
                })
            }
        }
        impl ImageInfo {
            pub fn wh(&self) -> Wh<Px> {
                Wh {
                    width: self.width,
                    height: self.height,
                }
            }
        }
        impl From<ImageInfo> for skia_safe::ImageInfo {
            fn from(val: ImageInfo) -> Self {
                skia_safe::ImageInfo::new(
                    skia_safe::ISize {
                        width: val.width.as_f32() as i32,
                        height: val.height.as_f32() as i32,
                    },
                    val.color_type.into(),
                    val.alpha_type.into(),
                    None,
                )
            }
        }
        static IMAGES: OnceLock<dashmap::DashMap<usize, Arc<skia_safe::image::Image>>> =
            OnceLock::new();
        static IMAGE_BUFFER_PTR: OnceLock<dashmap::DashMap<usize, usize>> = OnceLock::new();
        #[unsafe(no_mangle)]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn _register_image(
            image_id: usize,
            buffer_ptr: *const u8,
            buffer_len: usize,
        ) {
            IMAGE_BUFFER_PTR
                .get_or_init(dashmap::DashMap::new)
                .insert(image_id, buffer_ptr as usize);
            let data = unsafe {
                skia_safe::Data::new_bytes(std::slice::from_raw_parts(buffer_ptr, buffer_len))
            };
            let image = skia_safe::image::Image::from_encoded(data).unwrap();
            IMAGES
                .get_or_init(dashmap::DashMap::new)
                .insert(image_id, Arc::new(image));
        }
        #[unsafe(no_mangle)]
        #[allow(clippy::missing_safety_doc)]
        /**
         * image info layout
         * - id: u32
         * - alpha_type: u8
         * - color_type: u8
         * - width: u32
         * - height: u32
         */
        pub unsafe extern "C" fn _image_infos(ptr: *mut u8) {
            let images = IMAGES.get().unwrap();
            let count = images.len();
            let image_info_size = 14;
            let mut bytes = unsafe { std::slice::from_raw_parts_mut(ptr, count * image_info_size) };
            for image in images.iter() {
                let info = image.image_info();
                let id = image.key();
                bytes.put_u32_le(*id as u32);
                let alpha_type: AlphaType = info.alpha_type().into();
                let alpha_type: u8 = alpha_type.into();
                bytes.put_u8(alpha_type);
                let color_type: ColorType = info.color_type().into();
                let color_type: u8 = color_type.into();
                bytes.put_u8(color_type);
                let width = info.width();
                let height = info.height();
                bytes.put_u32_le(width as u32);
                bytes.put_u32_le(height as u32);
            }
        }
    }
    mod image_filter {
        use crate::*;
        use std::fmt::Debug;
        pub enum ImageFilter {
            #[default]
            Empty,
            Blur {
                sigma_xy: Xy<OrderedFloat>,
                tile_mode: Option<TileMode>,
                input: Option<Box<ImageFilter>>,
                /// crop_rect is not supported in wasm
                crop_rect: Option<Rect<Px>>,
            },
            Image {
                src: Image,
            },
            Blend {
                blender: Blender,
                background: Box<ImageFilter>,
                foreground: Box<ImageFilter>,
            },
            Offset {
                offset: Xy<Px>,
                input: Box<ImageFilter>,
            },
            ColorFilter {
                color_filter: ColorFilter,
                input: Box<ImageFilter>,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ImageFilter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ImageFilter::Empty => ::core::fmt::Formatter::write_str(f, "Empty"),
                    ImageFilter::Blur {
                        sigma_xy: __self_0,
                        tile_mode: __self_1,
                        input: __self_2,
                        crop_rect: __self_3,
                    } => ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Blur",
                        "sigma_xy",
                        __self_0,
                        "tile_mode",
                        __self_1,
                        "input",
                        __self_2,
                        "crop_rect",
                        &__self_3,
                    ),
                    ImageFilter::Image { src: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "Image", "src", &__self_0,
                        )
                    }
                    ImageFilter::Blend {
                        blender: __self_0,
                        background: __self_1,
                        foreground: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Blend",
                        "blender",
                        __self_0,
                        "background",
                        __self_1,
                        "foreground",
                        &__self_2,
                    ),
                    ImageFilter::Offset {
                        offset: __self_0,
                        input: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "Offset", "offset", __self_0, "input", &__self_1,
                    ),
                    ImageFilter::ColorFilter {
                        color_filter: __self_0,
                        input: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ColorFilter",
                        "color_filter",
                        __self_0,
                        "input",
                        &__self_1,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ImageFilter {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ImageFilter {
            #[inline]
            fn eq(&self, other: &ImageFilter) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            ImageFilter::Blur {
                                sigma_xy: __self_0,
                                tile_mode: __self_1,
                                input: __self_2,
                                crop_rect: __self_3,
                            },
                            ImageFilter::Blur {
                                sigma_xy: __arg1_0,
                                tile_mode: __arg1_1,
                                input: __arg1_2,
                                crop_rect: __arg1_3,
                            },
                        ) => {
                            __self_0 == __arg1_0
                                && __self_1 == __arg1_1
                                && __self_2 == __arg1_2
                                && __self_3 == __arg1_3
                        }
                        (
                            ImageFilter::Image { src: __self_0 },
                            ImageFilter::Image { src: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            ImageFilter::Blend {
                                blender: __self_0,
                                background: __self_1,
                                foreground: __self_2,
                            },
                            ImageFilter::Blend {
                                blender: __arg1_0,
                                background: __arg1_1,
                                foreground: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            ImageFilter::Offset {
                                offset: __self_0,
                                input: __self_1,
                            },
                            ImageFilter::Offset {
                                offset: __arg1_0,
                                input: __arg1_1,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        (
                            ImageFilter::ColorFilter {
                                color_filter: __self_0,
                                input: __self_1,
                            },
                            ImageFilter::ColorFilter {
                                color_filter: __arg1_0,
                                input: __arg1_1,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ImageFilter {
            #[inline]
            fn clone(&self) -> ImageFilter {
                match self {
                    ImageFilter::Empty => ImageFilter::Empty,
                    ImageFilter::Blur {
                        sigma_xy: __self_0,
                        tile_mode: __self_1,
                        input: __self_2,
                        crop_rect: __self_3,
                    } => ImageFilter::Blur {
                        sigma_xy: ::core::clone::Clone::clone(__self_0),
                        tile_mode: ::core::clone::Clone::clone(__self_1),
                        input: ::core::clone::Clone::clone(__self_2),
                        crop_rect: ::core::clone::Clone::clone(__self_3),
                    },
                    ImageFilter::Image { src: __self_0 } => ImageFilter::Image {
                        src: ::core::clone::Clone::clone(__self_0),
                    },
                    ImageFilter::Blend {
                        blender: __self_0,
                        background: __self_1,
                        foreground: __self_2,
                    } => ImageFilter::Blend {
                        blender: ::core::clone::Clone::clone(__self_0),
                        background: ::core::clone::Clone::clone(__self_1),
                        foreground: ::core::clone::Clone::clone(__self_2),
                    },
                    ImageFilter::Offset {
                        offset: __self_0,
                        input: __self_1,
                    } => ImageFilter::Offset {
                        offset: ::core::clone::Clone::clone(__self_0),
                        input: ::core::clone::Clone::clone(__self_1),
                    },
                    ImageFilter::ColorFilter {
                        color_filter: __self_0,
                        input: __self_1,
                    } => ImageFilter::ColorFilter {
                        color_filter: ::core::clone::Clone::clone(__self_0),
                        input: ::core::clone::Clone::clone(__self_1),
                    },
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ImageFilter {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    ImageFilter::Blur {
                        sigma_xy: __self_0,
                        tile_mode: __self_1,
                        input: __self_2,
                        crop_rect: __self_3,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state);
                        ::core::hash::Hash::hash(__self_3, state)
                    }
                    ImageFilter::Image { src: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    ImageFilter::Blend {
                        blender: __self_0,
                        background: __self_1,
                        foreground: __self_2,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state)
                    }
                    ImageFilter::Offset {
                        offset: __self_0,
                        input: __self_1,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state)
                    }
                    ImageFilter::ColorFilter {
                        color_filter: __self_0,
                        input: __self_1,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state)
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ImageFilter {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Xy<OrderedFloat>>;
                let _: ::core::cmp::AssertParamIsEq<Option<TileMode>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Box<ImageFilter>>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Rect<Px>>>;
                let _: ::core::cmp::AssertParamIsEq<Image>;
                let _: ::core::cmp::AssertParamIsEq<Blender>;
                let _: ::core::cmp::AssertParamIsEq<Box<ImageFilter>>;
                let _: ::core::cmp::AssertParamIsEq<Box<ImageFilter>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Box<ImageFilter>>;
                let _: ::core::cmp::AssertParamIsEq<ColorFilter>;
                let _: ::core::cmp::AssertParamIsEq<Box<ImageFilter>>;
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ImageFilter {
            #[inline]
            fn default() -> ImageFilter {
                Self::Empty
            }
        }
        impl bincode::Encode for ImageFilter {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Empty => {
                        bincode::Encode::encode(&0u32, encoder)?;
                    }
                    Self::Blur {
                        sigma_xy,
                        tile_mode,
                        input,
                        crop_rect,
                    } => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(sigma_xy, encoder)?;
                        bincode::Encode::encode(tile_mode, encoder)?;
                        bincode::Encode::encode(input, encoder)?;
                        bincode::Encode::encode(crop_rect, encoder)?;
                    }
                    Self::Image { src } => {
                        bincode::Encode::encode(&2u32, encoder)?;
                        bincode::Encode::encode(src, encoder)?;
                    }
                    Self::Blend {
                        blender,
                        background,
                        foreground,
                    } => {
                        bincode::Encode::encode(&3u32, encoder)?;
                        bincode::Encode::encode(blender, encoder)?;
                        bincode::Encode::encode(background, encoder)?;
                        bincode::Encode::encode(foreground, encoder)?;
                    }
                    Self::Offset { offset, input } => {
                        bincode::Encode::encode(&4u32, encoder)?;
                        bincode::Encode::encode(offset, encoder)?;
                        bincode::Encode::encode(input, encoder)?;
                    }
                    Self::ColorFilter {
                        color_filter,
                        input,
                    } => {
                        bincode::Encode::encode(&5u32, encoder)?;
                        bincode::Encode::encode(color_filter, encoder)?;
                        bincode::Encode::encode(input, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for ImageFilter {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Empty),
                    1u32 => Ok(Self::Blur {
                        sigma_xy: bincode::Decode::decode(decoder)?,
                        tile_mode: bincode::Decode::decode(decoder)?,
                        input: bincode::Decode::decode(decoder)?,
                        crop_rect: bincode::Decode::decode(decoder)?,
                    }),
                    2u32 => Ok(Self::Image {
                        src: bincode::Decode::decode(decoder)?,
                    }),
                    3u32 => Ok(Self::Blend {
                        blender: bincode::Decode::decode(decoder)?,
                        background: bincode::Decode::decode(decoder)?,
                        foreground: bincode::Decode::decode(decoder)?,
                    }),
                    4u32 => Ok(Self::Offset {
                        offset: bincode::Decode::decode(decoder)?,
                        input: bincode::Decode::decode(decoder)?,
                    }),
                    5u32 => Ok(Self::ColorFilter {
                        color_filter: bincode::Decode::decode(decoder)?,
                        input: bincode::Decode::decode(decoder)?,
                    }),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 5u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for ImageFilter {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Empty {} => {
                        buffer.write_string("Empty");
                    }
                    Self::Blur {
                        sigma_xy,
                        tile_mode,
                        input,
                        crop_rect,
                    } => {
                        buffer.write_string("Blur");
                        buffer.write_string("sigma_xy");
                        let field_bytes = Serialize::serialize(sigma_xy);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("tile_mode");
                        let field_bytes = Serialize::serialize(tile_mode);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("input");
                        let field_bytes = Serialize::serialize(input);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("crop_rect");
                        let field_bytes = Serialize::serialize(crop_rect);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Image { src } => {
                        buffer.write_string("Image");
                        buffer.write_string("src");
                        let field_bytes = Serialize::serialize(src);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Blend {
                        blender,
                        background,
                        foreground,
                    } => {
                        buffer.write_string("Blend");
                        buffer.write_string("blender");
                        let field_bytes = Serialize::serialize(blender);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("background");
                        let field_bytes = Serialize::serialize(background);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("foreground");
                        let field_bytes = Serialize::serialize(foreground);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Offset { offset, input } => {
                        buffer.write_string("Offset");
                        buffer.write_string("offset");
                        let field_bytes = Serialize::serialize(offset);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("input");
                        let field_bytes = Serialize::serialize(input);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::ColorFilter {
                        color_filter,
                        input,
                    } => {
                        buffer.write_string("ColorFilter");
                        buffer.write_string("color_filter");
                        let field_bytes = Serialize::serialize(color_filter);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("input");
                        let field_bytes = Serialize::serialize(input);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for ImageFilter {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Empty" => Ok(Self::Empty),
                    "Blur" => {
                        let field_name = buf.read_name("sigma_xy")?;
                        let sigma_xy = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("tile_mode")?;
                        let tile_mode = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("input")?;
                        let input = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("crop_rect")?;
                        let crop_rect = Deserialize::deserialize(buf)?;
                        Ok(Self::Blur {
                            sigma_xy,
                            tile_mode,
                            input,
                            crop_rect,
                        })
                    }
                    "Image" => {
                        let field_name = buf.read_name("src")?;
                        let src = Deserialize::deserialize(buf)?;
                        Ok(Self::Image { src })
                    }
                    "Blend" => {
                        let field_name = buf.read_name("blender")?;
                        let blender = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("background")?;
                        let background = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("foreground")?;
                        let foreground = Deserialize::deserialize(buf)?;
                        Ok(Self::Blend {
                            blender,
                            background,
                            foreground,
                        })
                    }
                    "Offset" => {
                        let field_name = buf.read_name("offset")?;
                        let offset = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("input")?;
                        let input = Deserialize::deserialize(buf)?;
                        Ok(Self::Offset { offset, input })
                    }
                    "ColorFilter" => {
                        let field_name = buf.read_name("color_filter")?;
                        let color_filter = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("input")?;
                        let input = Deserialize::deserialize(buf)?;
                        Ok(Self::ColorFilter {
                            color_filter,
                            input,
                        })
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl ImageFilter {
            pub fn offset(self, offset: Xy<Px>) -> Self {
                ImageFilter::Offset {
                    offset,
                    input: Box::new(self),
                }
            }
            pub fn blend(
                blender: impl Into<Blender>,
                background: ImageFilter,
                foreground: ImageFilter,
            ) -> Self {
                ImageFilter::Blend {
                    blender: blender.into(),
                    background: Box::new(background),
                    foreground: Box::new(foreground),
                }
            }
            pub fn color_filter(self, color_filter: ColorFilter) -> Self {
                ImageFilter::ColorFilter {
                    color_filter,
                    input: Box::new(self),
                }
            }
        }
        impl From<&ImageFilter> for skia_safe::ImageFilter {
            fn from(image_filter: &ImageFilter) -> Self {
                match image_filter {
                    &ImageFilter::Blur {
                        sigma_xy,
                        tile_mode,
                        ref input,
                        crop_rect,
                    } => skia_safe::image_filters::blur(
                        (sigma_xy.x.as_f32(), sigma_xy.y.as_f32()),
                        tile_mode.map(|tile_mode| tile_mode.into()),
                        input.as_ref().map(|input| input.as_ref().into()),
                        crop_rect.map(|x| skia_safe::Rect::from(x).into()),
                    )
                    .unwrap(),
                    ImageFilter::Image { src } => skia_safe::image_filters::image(
                        skia_safe::Image::clone(&src.skia_image()),
                        None,
                        None,
                        None,
                    )
                    .unwrap(),
                    ImageFilter::Blend {
                        blender,
                        background,
                        foreground,
                    } => skia_safe::image_filters::blend(
                        skia_safe::Blender::from(blender),
                        skia_safe::ImageFilter::from(background.as_ref()),
                        skia_safe::ImageFilter::from(foreground.as_ref()),
                        None,
                    )
                    .unwrap(),
                    ImageFilter::Offset { offset, input } => skia_safe::image_filters::offset(
                        (offset.x.as_f32(), offset.y.as_f32()),
                        skia_safe::ImageFilter::from(input.as_ref()),
                        None,
                    )
                    .unwrap(),
                    ImageFilter::ColorFilter {
                        color_filter,
                        input,
                    } => skia_safe::image_filters::color_filter(
                        NativeColorFilter::from(color_filter).skia(),
                        skia_safe::ImageFilter::from(input.as_ref()),
                        None,
                    )
                    .unwrap(),
                    ImageFilter::Empty => skia_safe::image_filters::empty(),
                }
            }
        }
    }
    mod mask_filter {
        use crate::*;
        use std::hash::Hash;
        pub enum MaskFilter {
            Blur { blur_style: BlurStyle, sigma: f32 },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MaskFilter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    MaskFilter::Blur {
                        blur_style: __self_0,
                        sigma: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Blur",
                        "blur_style",
                        __self_0,
                        "sigma",
                        &__self_1,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MaskFilter {
            #[inline]
            fn clone(&self) -> MaskFilter {
                let _: ::core::clone::AssertParamIsClone<BlurStyle>;
                let _: ::core::clone::AssertParamIsClone<f32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for MaskFilter {}
        impl bincode::Encode for MaskFilter {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Blur { blur_style, sigma } => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(blur_style, encoder)?;
                        bincode::Encode::encode(sigma, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for MaskFilter {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Blur {
                        blur_style: bincode::Decode::decode(decoder)?,
                        sigma: bincode::Decode::decode(decoder)?,
                    }),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 0u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for MaskFilter {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Blur { blur_style, sigma } => {
                        buffer.write_string("Blur");
                        buffer.write_string("blur_style");
                        let field_bytes = Serialize::serialize(blur_style);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("sigma");
                        let field_bytes = Serialize::serialize(sigma);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for MaskFilter {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Blur" => {
                        let field_name = buf.read_name("blur_style")?;
                        let blur_style = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("sigma")?;
                        let sigma = Deserialize::deserialize(buf)?;
                        Ok(Self::Blur { blur_style, sigma })
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl Hash for MaskFilter {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                match self {
                    MaskFilter::Blur { blur_style, sigma } => {
                        blur_style.hash(state);
                        sigma.to_bits().hash(state);
                    }
                }
            }
        }
        impl PartialEq for MaskFilter {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        MaskFilter::Blur {
                            blur_style: blur_style1,
                            sigma: sigma1,
                        },
                        MaskFilter::Blur {
                            blur_style: blur_style2,
                            sigma: sigma2,
                        },
                    ) => blur_style1 == blur_style2 && sigma1.to_bits() == sigma2.to_bits(),
                }
            }
        }
        impl Eq for MaskFilter {}
        pub enum BlurStyle {
            /// Fuzzy inside and outside
            Normal,
            /// Solid inside, fuzzy outside
            Solid,
            /// Nothing inside, fuzzy outside
            Outer,
            /// Fuzzy inside, nothing outside
            Inner,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BlurStyle {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        BlurStyle::Normal => "Normal",
                        BlurStyle::Solid => "Solid",
                        BlurStyle::Outer => "Outer",
                        BlurStyle::Inner => "Inner",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BlurStyle {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BlurStyle {
            #[inline]
            fn eq(&self, other: &BlurStyle) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BlurStyle {
            #[inline]
            fn clone(&self) -> BlurStyle {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for BlurStyle {}
        #[automatically_derived]
        impl ::core::hash::Hash for BlurStyle {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for BlurStyle {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        impl bincode::Encode for BlurStyle {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Normal => {
                        bincode::Encode::encode(&0u32, encoder)?;
                    }
                    Self::Solid => {
                        bincode::Encode::encode(&1u32, encoder)?;
                    }
                    Self::Outer => {
                        bincode::Encode::encode(&2u32, encoder)?;
                    }
                    Self::Inner => {
                        bincode::Encode::encode(&3u32, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for BlurStyle {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Normal),
                    1u32 => Ok(Self::Solid),
                    2u32 => Ok(Self::Outer),
                    3u32 => Ok(Self::Inner),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 3u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for BlurStyle {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Normal {} => {
                        buffer.write_string("Normal");
                    }
                    Self::Solid {} => {
                        buffer.write_string("Solid");
                    }
                    Self::Outer {} => {
                        buffer.write_string("Outer");
                    }
                    Self::Inner {} => {
                        buffer.write_string("Inner");
                    }
                }
                buffer
            }
        }
        impl Deserialize for BlurStyle {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Normal" => Ok(Self::Normal),
                    "Solid" => Ok(Self::Solid),
                    "Outer" => Ok(Self::Outer),
                    "Inner" => Ok(Self::Inner),
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl From<BlurStyle> for skia_safe::BlurStyle {
            fn from(blur_style: BlurStyle) -> Self {
                match blur_style {
                    BlurStyle::Normal => skia_safe::BlurStyle::Normal,
                    BlurStyle::Solid => skia_safe::BlurStyle::Solid,
                    BlurStyle::Outer => skia_safe::BlurStyle::Outer,
                    BlurStyle::Inner => skia_safe::BlurStyle::Inner,
                }
            }
        }
        /// https://android.googlesource.com/platform/frameworks/base/+/41fceb4/libs/hwui/utils/Blur.cpp
        /// This constant approximates the scaling done in the software path's
        /// "high quality" mode, in SkBlurMask::Blur() (1 / sqrt(3)).
        pub mod blur_sigma {
            const BLUR_SIGMA_SCALE: f32 = 0.57735;
            pub fn from_radius(radius: f32) -> f32 {
                if radius <= 0.0 {
                    return 0.0;
                }
                radius * BLUR_SIGMA_SCALE + 0.5
            }
            pub fn to_radius(sigma: f32) -> f32 {
                if sigma <= 0.5 {
                    return 0.0;
                }
                (sigma - 0.5) / BLUR_SIGMA_SCALE
            }
        }
    }
    mod paint {
        use crate::*;
        pub struct Paint {
            pub color: Color,
            pub paint_style: Option<PaintStyle>,
            pub anti_alias: Option<bool>,
            pub stroke_width: Px,
            pub stroke_cap: Option<StrokeCap>,
            pub stroke_join: Option<StrokeJoin>,
            pub stroke_miter: Px,
            pub color_filter: Option<ColorFilter>,
            pub blend_mode: Option<BlendMode>,
            pub shader: Option<Box<Shader>>,
            pub mask_filter: Option<MaskFilter>,
            pub image_filter: Option<Box<ImageFilter>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Paint {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "color",
                    "paint_style",
                    "anti_alias",
                    "stroke_width",
                    "stroke_cap",
                    "stroke_join",
                    "stroke_miter",
                    "color_filter",
                    "blend_mode",
                    "shader",
                    "mask_filter",
                    "image_filter",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.color,
                    &self.paint_style,
                    &self.anti_alias,
                    &self.stroke_width,
                    &self.stroke_cap,
                    &self.stroke_join,
                    &self.stroke_miter,
                    &self.color_filter,
                    &self.blend_mode,
                    &self.shader,
                    &self.mask_filter,
                    &&self.image_filter,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Paint", names, values)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Paint {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Paint {
            #[inline]
            fn eq(&self, other: &Paint) -> bool {
                self.color == other.color
                    && self.paint_style == other.paint_style
                    && self.anti_alias == other.anti_alias
                    && self.stroke_width == other.stroke_width
                    && self.stroke_cap == other.stroke_cap
                    && self.stroke_join == other.stroke_join
                    && self.stroke_miter == other.stroke_miter
                    && self.color_filter == other.color_filter
                    && self.blend_mode == other.blend_mode
                    && self.shader == other.shader
                    && self.mask_filter == other.mask_filter
                    && self.image_filter == other.image_filter
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Paint {
            #[inline]
            fn clone(&self) -> Paint {
                Paint {
                    color: ::core::clone::Clone::clone(&self.color),
                    paint_style: ::core::clone::Clone::clone(&self.paint_style),
                    anti_alias: ::core::clone::Clone::clone(&self.anti_alias),
                    stroke_width: ::core::clone::Clone::clone(&self.stroke_width),
                    stroke_cap: ::core::clone::Clone::clone(&self.stroke_cap),
                    stroke_join: ::core::clone::Clone::clone(&self.stroke_join),
                    stroke_miter: ::core::clone::Clone::clone(&self.stroke_miter),
                    color_filter: ::core::clone::Clone::clone(&self.color_filter),
                    blend_mode: ::core::clone::Clone::clone(&self.blend_mode),
                    shader: ::core::clone::Clone::clone(&self.shader),
                    mask_filter: ::core::clone::Clone::clone(&self.mask_filter),
                    image_filter: ::core::clone::Clone::clone(&self.image_filter),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Paint {
            #[inline]
            fn default() -> Paint {
                Paint {
                    color: ::core::default::Default::default(),
                    paint_style: ::core::default::Default::default(),
                    anti_alias: ::core::default::Default::default(),
                    stroke_width: ::core::default::Default::default(),
                    stroke_cap: ::core::default::Default::default(),
                    stroke_join: ::core::default::Default::default(),
                    stroke_miter: ::core::default::Default::default(),
                    color_filter: ::core::default::Default::default(),
                    blend_mode: ::core::default::Default::default(),
                    shader: ::core::default::Default::default(),
                    mask_filter: ::core::default::Default::default(),
                    image_filter: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Paint {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.color, state);
                ::core::hash::Hash::hash(&self.paint_style, state);
                ::core::hash::Hash::hash(&self.anti_alias, state);
                ::core::hash::Hash::hash(&self.stroke_width, state);
                ::core::hash::Hash::hash(&self.stroke_cap, state);
                ::core::hash::Hash::hash(&self.stroke_join, state);
                ::core::hash::Hash::hash(&self.stroke_miter, state);
                ::core::hash::Hash::hash(&self.color_filter, state);
                ::core::hash::Hash::hash(&self.blend_mode, state);
                ::core::hash::Hash::hash(&self.shader, state);
                ::core::hash::Hash::hash(&self.mask_filter, state);
                ::core::hash::Hash::hash(&self.image_filter, state)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Paint {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Color>;
                let _: ::core::cmp::AssertParamIsEq<Option<PaintStyle>>;
                let _: ::core::cmp::AssertParamIsEq<Option<bool>>;
                let _: ::core::cmp::AssertParamIsEq<Px>;
                let _: ::core::cmp::AssertParamIsEq<Option<StrokeCap>>;
                let _: ::core::cmp::AssertParamIsEq<Option<StrokeJoin>>;
                let _: ::core::cmp::AssertParamIsEq<Option<ColorFilter>>;
                let _: ::core::cmp::AssertParamIsEq<Option<BlendMode>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Box<Shader>>>;
                let _: ::core::cmp::AssertParamIsEq<Option<MaskFilter>>;
                let _: ::core::cmp::AssertParamIsEq<Option<Box<ImageFilter>>>;
            }
        }
        impl bincode::Encode for Paint {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.color, encoder)?;
                bincode::Encode::encode(&self.paint_style, encoder)?;
                bincode::Encode::encode(&self.anti_alias, encoder)?;
                bincode::Encode::encode(&self.stroke_width, encoder)?;
                bincode::Encode::encode(&self.stroke_cap, encoder)?;
                bincode::Encode::encode(&self.stroke_join, encoder)?;
                bincode::Encode::encode(&self.stroke_miter, encoder)?;
                bincode::Encode::encode(&self.color_filter, encoder)?;
                bincode::Encode::encode(&self.blend_mode, encoder)?;
                bincode::Encode::encode(&self.shader, encoder)?;
                bincode::Encode::encode(&self.mask_filter, encoder)?;
                bincode::Encode::encode(&self.image_filter, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for Paint {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    color: bincode::Decode::decode(decoder)?,
                    paint_style: bincode::Decode::decode(decoder)?,
                    anti_alias: bincode::Decode::decode(decoder)?,
                    stroke_width: bincode::Decode::decode(decoder)?,
                    stroke_cap: bincode::Decode::decode(decoder)?,
                    stroke_join: bincode::Decode::decode(decoder)?,
                    stroke_miter: bincode::Decode::decode(decoder)?,
                    color_filter: bincode::Decode::decode(decoder)?,
                    blend_mode: bincode::Decode::decode(decoder)?,
                    shader: bincode::Decode::decode(decoder)?,
                    mask_filter: bincode::Decode::decode(decoder)?,
                    image_filter: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for Paint {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("color");
                let field_bytes = Serialize::serialize(&self.color);
                buffer.put_slice(&field_bytes);
                buffer.write_string("paint_style");
                let field_bytes = Serialize::serialize(&self.paint_style);
                buffer.put_slice(&field_bytes);
                buffer.write_string("anti_alias");
                let field_bytes = Serialize::serialize(&self.anti_alias);
                buffer.put_slice(&field_bytes);
                buffer.write_string("stroke_width");
                let field_bytes = Serialize::serialize(&self.stroke_width);
                buffer.put_slice(&field_bytes);
                buffer.write_string("stroke_cap");
                let field_bytes = Serialize::serialize(&self.stroke_cap);
                buffer.put_slice(&field_bytes);
                buffer.write_string("stroke_join");
                let field_bytes = Serialize::serialize(&self.stroke_join);
                buffer.put_slice(&field_bytes);
                buffer.write_string("stroke_miter");
                let field_bytes = Serialize::serialize(&self.stroke_miter);
                buffer.put_slice(&field_bytes);
                buffer.write_string("color_filter");
                let field_bytes = Serialize::serialize(&self.color_filter);
                buffer.put_slice(&field_bytes);
                buffer.write_string("blend_mode");
                let field_bytes = Serialize::serialize(&self.blend_mode);
                buffer.put_slice(&field_bytes);
                buffer.write_string("shader");
                let field_bytes = Serialize::serialize(&self.shader);
                buffer.put_slice(&field_bytes);
                buffer.write_string("mask_filter");
                let field_bytes = Serialize::serialize(&self.mask_filter);
                buffer.put_slice(&field_bytes);
                buffer.write_string("image_filter");
                let field_bytes = Serialize::serialize(&self.image_filter);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for Paint {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("color")?;
                let color = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("paint_style")?;
                let paint_style = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("anti_alias")?;
                let anti_alias = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("stroke_width")?;
                let stroke_width = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("stroke_cap")?;
                let stroke_cap = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("stroke_join")?;
                let stroke_join = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("stroke_miter")?;
                let stroke_miter = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("color_filter")?;
                let color_filter = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("blend_mode")?;
                let blend_mode = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("shader")?;
                let shader = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("mask_filter")?;
                let mask_filter = Deserialize::deserialize(buf)?;
                let field_name = buf.read_name("image_filter")?;
                let image_filter = Deserialize::deserialize(buf)?;
                Ok(Self {
                    color,
                    paint_style,
                    anti_alias,
                    stroke_width,
                    stroke_cap,
                    stroke_join,
                    stroke_miter,
                    color_filter,
                    blend_mode,
                    shader,
                    mask_filter,
                    image_filter,
                })
            }
        }
        impl Paint {
            pub fn new(color: Color) -> Self {
                Self {
                    color,
                    ..Default::default()
                }
            }
            pub fn set_color(mut self, color: Color) -> Self {
                self.color = color;
                self
            }
            pub fn set_style(mut self, style: PaintStyle) -> Self {
                self.paint_style = Some(style);
                self
            }
            pub fn set_anti_alias(mut self, value: bool) -> Self {
                self.anti_alias = Some(value);
                self
            }
            pub fn set_stroke_width(mut self, width: Px) -> Self {
                self.stroke_width = width;
                self
            }
            pub fn set_stroke_cap(mut self, cap: StrokeCap) -> Self {
                self.stroke_cap = Some(cap);
                self
            }
            pub fn set_stroke_join(mut self, join: StrokeJoin) -> Self {
                self.stroke_join = Some(join);
                self
            }
            pub fn set_color_filter(mut self, color_filter: ColorFilter) -> Self {
                self.color_filter = Some(color_filter);
                self
            }
            pub fn set_blend_mode(mut self, blend_mode: BlendMode) -> Self {
                self.blend_mode = Some(blend_mode);
                self
            }
            pub fn set_shader(mut self, shader: Shader) -> Self {
                self.shader = Some(Box::new(shader));
                self
            }
            pub fn set_mask_filter(mut self, mask_filter: MaskFilter) -> Self {
                self.mask_filter = Some(mask_filter);
                self
            }
            pub fn set_image_filter(mut self, image_filter: ImageFilter) -> Self {
                self.image_filter = Some(Box::new(image_filter));
                self
            }
        }
    }
    mod path {
        use crate::*;
        pub struct Path {
            commands: Vec<PathCommand>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Path {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Path",
                    "commands",
                    &&self.commands,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Path {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Path {
            #[inline]
            fn eq(&self, other: &Path) -> bool {
                self.commands == other.commands
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Path {
            #[inline]
            fn clone(&self) -> Path {
                Path {
                    commands: ::core::clone::Clone::clone(&self.commands),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Path {
            #[inline]
            fn default() -> Path {
                Path {
                    commands: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Path {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Vec<PathCommand>>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Path {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.commands, state)
            }
        }
        impl bincode::Encode for Path {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                bincode::Encode::encode(&self.commands, encoder)?;
                Ok(())
            }
        }
        impl bincode::Decode<()> for Path {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                Ok(Self {
                    commands: bincode::Decode::decode(decoder)?,
                })
            }
        }
        impl Serialize for Path {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                buffer.write_string("commands");
                let field_bytes = Serialize::serialize(&self.commands);
                buffer.put_slice(&field_bytes);
                buffer
            }
        }
        impl Deserialize for Path {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let field_name = buf.read_name("commands")?;
                let commands = Deserialize::deserialize(buf)?;
                Ok(Self { commands })
            }
        }
        impl Path {
            pub fn new() -> Self {
                Self {
                    commands: Vec::with_capacity(2),
                }
            }
            pub fn commands(&self) -> &Vec<PathCommand> {
                &self.commands
            }
            pub fn add_rect(mut self, rect: Rect<Px>) -> Self {
                self.commands.push(PathCommand::AddRect { rect });
                self
            }
            pub fn add_rrect(mut self, rect: Rect<Px>, rx: Px, ry: Px) -> Self {
                self.commands.push(PathCommand::AddRrect { rect, rx, ry });
                self
            }
            pub fn stroke(mut self, options: StrokeOptions) -> Self {
                self.commands.push(PathCommand::Stroke {
                    stroke_options: options,
                });
                self
            }
            pub fn move_to(mut self, x: Px, y: Px) -> Self {
                self.commands.push(PathCommand::MoveTo { xy: Xy { x, y } });
                self
            }
            pub fn line_to(mut self, x: Px, y: Px) -> Self {
                self.commands.push(PathCommand::LineTo { xy: Xy { x, y } });
                self
            }
            pub fn cubic_to(mut self, first_xy: Xy<Px>, second_xy: Xy<Px>, end_xy: Xy<Px>) -> Self {
                self.commands.push(PathCommand::CubicTo {
                    first_xy,
                    second_xy,
                    end_xy,
                });
                self
            }
            pub fn arc_to(
                mut self,
                oval: Rect<Px>,
                start_angle: Angle,
                delta_angle: Angle,
            ) -> Self {
                self.commands.push(PathCommand::ArcTo {
                    oval,
                    start_angle,
                    delta_angle,
                });
                self
            }
            pub fn scale(mut self, sx: f32, sy: f32) -> Self {
                self.commands.push(PathCommand::Scale {
                    xy: Xy {
                        x: sx.into(),
                        y: sy.into(),
                    },
                });
                self
            }
            pub fn translate(mut self, x: Px, y: Px) -> Self {
                self.commands
                    .push(PathCommand::Translate { xy: Xy { x, y } });
                self
            }
            pub fn transform(mut self, matrix: TransformMatrix) -> Self {
                self.commands.push(PathCommand::Transform { matrix });
                self
            }
            pub fn add_oval(mut self, rect: Rect<Px>) -> Self {
                self.commands.push(PathCommand::AddOval { rect });
                self
            }
            pub fn add_arc(
                mut self,
                oval: Rect<Px>,
                start_angle: Angle,
                delta_angle: Angle,
            ) -> Self {
                self.commands.push(PathCommand::AddArc {
                    oval,
                    start_angle,
                    delta_angle,
                });
                self
            }
            pub fn add_poly(mut self, xy_array: &[Xy<Px>], close: bool) -> Self {
                self.commands.push(PathCommand::AddPoly {
                    xys: xy_array.to_vec(),
                    close,
                });
                self
            }
            pub fn close(mut self) -> Self {
                self.commands.push(PathCommand::Close);
                self
            }
        }
        pub enum PathCommand {
            AddRect {
                rect: Rect<Px>,
            },
            AddRrect {
                rect: Rect<Px>,
                rx: Px,
                ry: Px,
            },
            Stroke {
                stroke_options: StrokeOptions,
            },
            MoveTo {
                xy: Xy<Px>,
            },
            LineTo {
                xy: Xy<Px>,
            },
            CubicTo {
                first_xy: Xy<Px>,
                second_xy: Xy<Px>,
                end_xy: Xy<Px>,
            },
            ArcTo {
                oval: Rect<Px>,
                start_angle: Angle,
                delta_angle: Angle,
            },
            Scale {
                xy: Xy<OrderedFloat>,
            },
            Translate {
                xy: Xy<Px>,
            },
            Transform {
                matrix: TransformMatrix,
            },
            AddOval {
                rect: Rect<Px>,
            },
            AddArc {
                oval: Rect<Px>,
                start_angle: Angle,
                delta_angle: Angle,
            },
            AddPoly {
                xys: Vec<Xy<Px>>,
                close: bool,
            },
            Close,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PathCommand {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    PathCommand::AddRect { rect: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "AddRect", "rect", &__self_0,
                        )
                    }
                    PathCommand::AddRrect {
                        rect: __self_0,
                        rx: __self_1,
                        ry: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f, "AddRrect", "rect", __self_0, "rx", __self_1, "ry", &__self_2,
                    ),
                    PathCommand::Stroke {
                        stroke_options: __self_0,
                    } => ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Stroke",
                        "stroke_options",
                        &__self_0,
                    ),
                    PathCommand::MoveTo { xy: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "MoveTo", "xy", &__self_0,
                        )
                    }
                    PathCommand::LineTo { xy: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "LineTo", "xy", &__self_0,
                        )
                    }
                    PathCommand::CubicTo {
                        first_xy: __self_0,
                        second_xy: __self_1,
                        end_xy: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "CubicTo",
                        "first_xy",
                        __self_0,
                        "second_xy",
                        __self_1,
                        "end_xy",
                        &__self_2,
                    ),
                    PathCommand::ArcTo {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ArcTo",
                        "oval",
                        __self_0,
                        "start_angle",
                        __self_1,
                        "delta_angle",
                        &__self_2,
                    ),
                    PathCommand::Scale { xy: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "Scale", "xy", &__self_0,
                        )
                    }
                    PathCommand::Translate { xy: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Translate",
                            "xy",
                            &__self_0,
                        )
                    }
                    PathCommand::Transform { matrix: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "Transform",
                            "matrix",
                            &__self_0,
                        )
                    }
                    PathCommand::AddOval { rect: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "AddOval", "rect", &__self_0,
                        )
                    }
                    PathCommand::AddArc {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "AddArc",
                        "oval",
                        __self_0,
                        "start_angle",
                        __self_1,
                        "delta_angle",
                        &__self_2,
                    ),
                    PathCommand::AddPoly {
                        xys: __self_0,
                        close: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "AddPoly", "xys", __self_0, "close", &__self_1,
                    ),
                    PathCommand::Close => ::core::fmt::Formatter::write_str(f, "Close"),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PathCommand {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PathCommand {
            #[inline]
            fn eq(&self, other: &PathCommand) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            PathCommand::AddRect { rect: __self_0 },
                            PathCommand::AddRect { rect: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::AddRrect {
                                rect: __self_0,
                                rx: __self_1,
                                ry: __self_2,
                            },
                            PathCommand::AddRrect {
                                rect: __arg1_0,
                                rx: __arg1_1,
                                ry: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            PathCommand::Stroke {
                                stroke_options: __self_0,
                            },
                            PathCommand::Stroke {
                                stroke_options: __arg1_0,
                            },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::MoveTo { xy: __self_0 },
                            PathCommand::MoveTo { xy: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::LineTo { xy: __self_0 },
                            PathCommand::LineTo { xy: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::CubicTo {
                                first_xy: __self_0,
                                second_xy: __self_1,
                                end_xy: __self_2,
                            },
                            PathCommand::CubicTo {
                                first_xy: __arg1_0,
                                second_xy: __arg1_1,
                                end_xy: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            PathCommand::ArcTo {
                                oval: __self_0,
                                start_angle: __self_1,
                                delta_angle: __self_2,
                            },
                            PathCommand::ArcTo {
                                oval: __arg1_0,
                                start_angle: __arg1_1,
                                delta_angle: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            PathCommand::Scale { xy: __self_0 },
                            PathCommand::Scale { xy: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::Translate { xy: __self_0 },
                            PathCommand::Translate { xy: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::Transform { matrix: __self_0 },
                            PathCommand::Transform { matrix: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::AddOval { rect: __self_0 },
                            PathCommand::AddOval { rect: __arg1_0 },
                        ) => __self_0 == __arg1_0,
                        (
                            PathCommand::AddArc {
                                oval: __self_0,
                                start_angle: __self_1,
                                delta_angle: __self_2,
                            },
                            PathCommand::AddArc {
                                oval: __arg1_0,
                                start_angle: __arg1_1,
                                delta_angle: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            PathCommand::AddPoly {
                                xys: __self_0,
                                close: __self_1,
                            },
                            PathCommand::AddPoly {
                                xys: __arg1_0,
                                close: __arg1_1,
                            },
                        ) => __self_1 == __arg1_1 && __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PathCommand {
            #[inline]
            fn clone(&self) -> PathCommand {
                match self {
                    PathCommand::AddRect { rect: __self_0 } => PathCommand::AddRect {
                        rect: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::AddRrect {
                        rect: __self_0,
                        rx: __self_1,
                        ry: __self_2,
                    } => PathCommand::AddRrect {
                        rect: ::core::clone::Clone::clone(__self_0),
                        rx: ::core::clone::Clone::clone(__self_1),
                        ry: ::core::clone::Clone::clone(__self_2),
                    },
                    PathCommand::Stroke {
                        stroke_options: __self_0,
                    } => PathCommand::Stroke {
                        stroke_options: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::MoveTo { xy: __self_0 } => PathCommand::MoveTo {
                        xy: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::LineTo { xy: __self_0 } => PathCommand::LineTo {
                        xy: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::CubicTo {
                        first_xy: __self_0,
                        second_xy: __self_1,
                        end_xy: __self_2,
                    } => PathCommand::CubicTo {
                        first_xy: ::core::clone::Clone::clone(__self_0),
                        second_xy: ::core::clone::Clone::clone(__self_1),
                        end_xy: ::core::clone::Clone::clone(__self_2),
                    },
                    PathCommand::ArcTo {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => PathCommand::ArcTo {
                        oval: ::core::clone::Clone::clone(__self_0),
                        start_angle: ::core::clone::Clone::clone(__self_1),
                        delta_angle: ::core::clone::Clone::clone(__self_2),
                    },
                    PathCommand::Scale { xy: __self_0 } => PathCommand::Scale {
                        xy: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::Translate { xy: __self_0 } => PathCommand::Translate {
                        xy: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::Transform { matrix: __self_0 } => PathCommand::Transform {
                        matrix: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::AddOval { rect: __self_0 } => PathCommand::AddOval {
                        rect: ::core::clone::Clone::clone(__self_0),
                    },
                    PathCommand::AddArc {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => PathCommand::AddArc {
                        oval: ::core::clone::Clone::clone(__self_0),
                        start_angle: ::core::clone::Clone::clone(__self_1),
                        delta_angle: ::core::clone::Clone::clone(__self_2),
                    },
                    PathCommand::AddPoly {
                        xys: __self_0,
                        close: __self_1,
                    } => PathCommand::AddPoly {
                        xys: ::core::clone::Clone::clone(__self_0),
                        close: ::core::clone::Clone::clone(__self_1),
                    },
                    PathCommand::Close => PathCommand::Close,
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for PathCommand {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Px>;
                let _: ::core::cmp::AssertParamIsEq<StrokeOptions>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Angle>;
                let _: ::core::cmp::AssertParamIsEq<Xy<OrderedFloat>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<TransformMatrix>;
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Rect<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Xy<Px>>>;
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for PathCommand {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    PathCommand::AddRect { rect: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::AddRrect {
                        rect: __self_0,
                        rx: __self_1,
                        ry: __self_2,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state)
                    }
                    PathCommand::Stroke {
                        stroke_options: __self_0,
                    } => ::core::hash::Hash::hash(__self_0, state),
                    PathCommand::MoveTo { xy: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::LineTo { xy: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::CubicTo {
                        first_xy: __self_0,
                        second_xy: __self_1,
                        end_xy: __self_2,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state)
                    }
                    PathCommand::ArcTo {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state)
                    }
                    PathCommand::Scale { xy: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::Translate { xy: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::Transform { matrix: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::AddOval { rect: __self_0 } => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    PathCommand::AddArc {
                        oval: __self_0,
                        start_angle: __self_1,
                        delta_angle: __self_2,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state);
                        ::core::hash::Hash::hash(__self_2, state)
                    }
                    PathCommand::AddPoly {
                        xys: __self_0,
                        close: __self_1,
                    } => {
                        ::core::hash::Hash::hash(__self_0, state);
                        ::core::hash::Hash::hash(__self_1, state)
                    }
                    _ => {}
                }
            }
        }
        impl bincode::Encode for PathCommand {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::AddRect { rect } => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(rect, encoder)?;
                    }
                    Self::AddRrect { rect, rx, ry } => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(rect, encoder)?;
                        bincode::Encode::encode(rx, encoder)?;
                        bincode::Encode::encode(ry, encoder)?;
                    }
                    Self::Stroke { stroke_options } => {
                        bincode::Encode::encode(&2u32, encoder)?;
                        bincode::Encode::encode(stroke_options, encoder)?;
                    }
                    Self::MoveTo { xy } => {
                        bincode::Encode::encode(&3u32, encoder)?;
                        bincode::Encode::encode(xy, encoder)?;
                    }
                    Self::LineTo { xy } => {
                        bincode::Encode::encode(&4u32, encoder)?;
                        bincode::Encode::encode(xy, encoder)?;
                    }
                    Self::CubicTo {
                        first_xy,
                        second_xy,
                        end_xy,
                    } => {
                        bincode::Encode::encode(&5u32, encoder)?;
                        bincode::Encode::encode(first_xy, encoder)?;
                        bincode::Encode::encode(second_xy, encoder)?;
                        bincode::Encode::encode(end_xy, encoder)?;
                    }
                    Self::ArcTo {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        bincode::Encode::encode(&6u32, encoder)?;
                        bincode::Encode::encode(oval, encoder)?;
                        bincode::Encode::encode(start_angle, encoder)?;
                        bincode::Encode::encode(delta_angle, encoder)?;
                    }
                    Self::Scale { xy } => {
                        bincode::Encode::encode(&7u32, encoder)?;
                        bincode::Encode::encode(xy, encoder)?;
                    }
                    Self::Translate { xy } => {
                        bincode::Encode::encode(&8u32, encoder)?;
                        bincode::Encode::encode(xy, encoder)?;
                    }
                    Self::Transform { matrix } => {
                        bincode::Encode::encode(&9u32, encoder)?;
                        bincode::Encode::encode(matrix, encoder)?;
                    }
                    Self::AddOval { rect } => {
                        bincode::Encode::encode(&10u32, encoder)?;
                        bincode::Encode::encode(rect, encoder)?;
                    }
                    Self::AddArc {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        bincode::Encode::encode(&11u32, encoder)?;
                        bincode::Encode::encode(oval, encoder)?;
                        bincode::Encode::encode(start_angle, encoder)?;
                        bincode::Encode::encode(delta_angle, encoder)?;
                    }
                    Self::AddPoly { xys, close } => {
                        bincode::Encode::encode(&12u32, encoder)?;
                        bincode::Encode::encode(xys, encoder)?;
                        bincode::Encode::encode(close, encoder)?;
                    }
                    Self::Close => {
                        bincode::Encode::encode(&13u32, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for PathCommand {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::AddRect {
                        rect: bincode::Decode::decode(decoder)?,
                    }),
                    1u32 => Ok(Self::AddRrect {
                        rect: bincode::Decode::decode(decoder)?,
                        rx: bincode::Decode::decode(decoder)?,
                        ry: bincode::Decode::decode(decoder)?,
                    }),
                    2u32 => Ok(Self::Stroke {
                        stroke_options: bincode::Decode::decode(decoder)?,
                    }),
                    3u32 => Ok(Self::MoveTo {
                        xy: bincode::Decode::decode(decoder)?,
                    }),
                    4u32 => Ok(Self::LineTo {
                        xy: bincode::Decode::decode(decoder)?,
                    }),
                    5u32 => Ok(Self::CubicTo {
                        first_xy: bincode::Decode::decode(decoder)?,
                        second_xy: bincode::Decode::decode(decoder)?,
                        end_xy: bincode::Decode::decode(decoder)?,
                    }),
                    6u32 => Ok(Self::ArcTo {
                        oval: bincode::Decode::decode(decoder)?,
                        start_angle: bincode::Decode::decode(decoder)?,
                        delta_angle: bincode::Decode::decode(decoder)?,
                    }),
                    7u32 => Ok(Self::Scale {
                        xy: bincode::Decode::decode(decoder)?,
                    }),
                    8u32 => Ok(Self::Translate {
                        xy: bincode::Decode::decode(decoder)?,
                    }),
                    9u32 => Ok(Self::Transform {
                        matrix: bincode::Decode::decode(decoder)?,
                    }),
                    10u32 => Ok(Self::AddOval {
                        rect: bincode::Decode::decode(decoder)?,
                    }),
                    11u32 => Ok(Self::AddArc {
                        oval: bincode::Decode::decode(decoder)?,
                        start_angle: bincode::Decode::decode(decoder)?,
                        delta_angle: bincode::Decode::decode(decoder)?,
                    }),
                    12u32 => Ok(Self::AddPoly {
                        xys: bincode::Decode::decode(decoder)?,
                        close: bincode::Decode::decode(decoder)?,
                    }),
                    13u32 => Ok(Self::Close),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 13u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for PathCommand {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::AddRect { rect } => {
                        buffer.write_string("AddRect");
                        buffer.write_string("rect");
                        let field_bytes = Serialize::serialize(rect);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::AddRrect { rect, rx, ry } => {
                        buffer.write_string("AddRrect");
                        buffer.write_string("rect");
                        let field_bytes = Serialize::serialize(rect);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("rx");
                        let field_bytes = Serialize::serialize(rx);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("ry");
                        let field_bytes = Serialize::serialize(ry);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Stroke { stroke_options } => {
                        buffer.write_string("Stroke");
                        buffer.write_string("stroke_options");
                        let field_bytes = Serialize::serialize(stroke_options);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::MoveTo { xy } => {
                        buffer.write_string("MoveTo");
                        buffer.write_string("xy");
                        let field_bytes = Serialize::serialize(xy);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::LineTo { xy } => {
                        buffer.write_string("LineTo");
                        buffer.write_string("xy");
                        let field_bytes = Serialize::serialize(xy);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::CubicTo {
                        first_xy,
                        second_xy,
                        end_xy,
                    } => {
                        buffer.write_string("CubicTo");
                        buffer.write_string("first_xy");
                        let field_bytes = Serialize::serialize(first_xy);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("second_xy");
                        let field_bytes = Serialize::serialize(second_xy);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("end_xy");
                        let field_bytes = Serialize::serialize(end_xy);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::ArcTo {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        buffer.write_string("ArcTo");
                        buffer.write_string("oval");
                        let field_bytes = Serialize::serialize(oval);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("start_angle");
                        let field_bytes = Serialize::serialize(start_angle);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("delta_angle");
                        let field_bytes = Serialize::serialize(delta_angle);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Scale { xy } => {
                        buffer.write_string("Scale");
                        buffer.write_string("xy");
                        let field_bytes = Serialize::serialize(xy);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Translate { xy } => {
                        buffer.write_string("Translate");
                        buffer.write_string("xy");
                        let field_bytes = Serialize::serialize(xy);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Transform { matrix } => {
                        buffer.write_string("Transform");
                        buffer.write_string("matrix");
                        let field_bytes = Serialize::serialize(matrix);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::AddOval { rect } => {
                        buffer.write_string("AddOval");
                        buffer.write_string("rect");
                        let field_bytes = Serialize::serialize(rect);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::AddArc {
                        oval,
                        start_angle,
                        delta_angle,
                    } => {
                        buffer.write_string("AddArc");
                        buffer.write_string("oval");
                        let field_bytes = Serialize::serialize(oval);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("start_angle");
                        let field_bytes = Serialize::serialize(start_angle);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("delta_angle");
                        let field_bytes = Serialize::serialize(delta_angle);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::AddPoly { xys, close } => {
                        buffer.write_string("AddPoly");
                        buffer.write_string("xys");
                        let field_bytes = Serialize::serialize(xys);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("close");
                        let field_bytes = Serialize::serialize(close);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Close {} => {
                        buffer.write_string("Close");
                    }
                }
                buffer
            }
        }
        impl Deserialize for PathCommand {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "AddRect" => {
                        let field_name = buf.read_name("rect")?;
                        let rect = Deserialize::deserialize(buf)?;
                        Ok(Self::AddRect { rect })
                    }
                    "AddRrect" => {
                        let field_name = buf.read_name("rect")?;
                        let rect = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("rx")?;
                        let rx = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("ry")?;
                        let ry = Deserialize::deserialize(buf)?;
                        Ok(Self::AddRrect { rect, rx, ry })
                    }
                    "Stroke" => {
                        let field_name = buf.read_name("stroke_options")?;
                        let stroke_options = Deserialize::deserialize(buf)?;
                        Ok(Self::Stroke { stroke_options })
                    }
                    "MoveTo" => {
                        let field_name = buf.read_name("xy")?;
                        let xy = Deserialize::deserialize(buf)?;
                        Ok(Self::MoveTo { xy })
                    }
                    "LineTo" => {
                        let field_name = buf.read_name("xy")?;
                        let xy = Deserialize::deserialize(buf)?;
                        Ok(Self::LineTo { xy })
                    }
                    "CubicTo" => {
                        let field_name = buf.read_name("first_xy")?;
                        let first_xy = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("second_xy")?;
                        let second_xy = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("end_xy")?;
                        let end_xy = Deserialize::deserialize(buf)?;
                        Ok(Self::CubicTo {
                            first_xy,
                            second_xy,
                            end_xy,
                        })
                    }
                    "ArcTo" => {
                        let field_name = buf.read_name("oval")?;
                        let oval = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("start_angle")?;
                        let start_angle = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("delta_angle")?;
                        let delta_angle = Deserialize::deserialize(buf)?;
                        Ok(Self::ArcTo {
                            oval,
                            start_angle,
                            delta_angle,
                        })
                    }
                    "Scale" => {
                        let field_name = buf.read_name("xy")?;
                        let xy = Deserialize::deserialize(buf)?;
                        Ok(Self::Scale { xy })
                    }
                    "Translate" => {
                        let field_name = buf.read_name("xy")?;
                        let xy = Deserialize::deserialize(buf)?;
                        Ok(Self::Translate { xy })
                    }
                    "Transform" => {
                        let field_name = buf.read_name("matrix")?;
                        let matrix = Deserialize::deserialize(buf)?;
                        Ok(Self::Transform { matrix })
                    }
                    "AddOval" => {
                        let field_name = buf.read_name("rect")?;
                        let rect = Deserialize::deserialize(buf)?;
                        Ok(Self::AddOval { rect })
                    }
                    "AddArc" => {
                        let field_name = buf.read_name("oval")?;
                        let oval = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("start_angle")?;
                        let start_angle = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("delta_angle")?;
                        let delta_angle = Deserialize::deserialize(buf)?;
                        Ok(Self::AddArc {
                            oval,
                            start_angle,
                            delta_angle,
                        })
                    }
                    "AddPoly" => {
                        let field_name = buf.read_name("xys")?;
                        let xys = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("close")?;
                        let close = Deserialize::deserialize(buf)?;
                        Ok(Self::AddPoly { xys, close })
                    }
                    "Close" => Ok(Self::Close),
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
    }
    mod shader {
        use crate::*;
        use std::hash::Hash;
        pub enum Shader {
            Image {
                src: Image,
                tile_mode: Xy<TileMode>,
            },
            Blend {
                blend_mode: BlendMode,
                src: Box<Shader>,
                dest: Box<Shader>,
            },
            LinearGradient {
                start_xy: Xy<Px>,
                end_xy: Xy<Px>,
                colors: Vec<Color>,
                tile_mode: TileMode,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Shader {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Shader::Image {
                        src: __self_0,
                        tile_mode: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Image",
                        "src",
                        __self_0,
                        "tile_mode",
                        &__self_1,
                    ),
                    Shader::Blend {
                        blend_mode: __self_0,
                        src: __self_1,
                        dest: __self_2,
                    } => ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Blend",
                        "blend_mode",
                        __self_0,
                        "src",
                        __self_1,
                        "dest",
                        &__self_2,
                    ),
                    Shader::LinearGradient {
                        start_xy: __self_0,
                        end_xy: __self_1,
                        colors: __self_2,
                        tile_mode: __self_3,
                    } => ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "LinearGradient",
                        "start_xy",
                        __self_0,
                        "end_xy",
                        __self_1,
                        "colors",
                        __self_2,
                        "tile_mode",
                        &__self_3,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Shader {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Shader {
            #[inline]
            fn eq(&self, other: &Shader) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            Shader::Image {
                                src: __self_0,
                                tile_mode: __self_1,
                            },
                            Shader::Image {
                                src: __arg1_0,
                                tile_mode: __arg1_1,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                        (
                            Shader::Blend {
                                blend_mode: __self_0,
                                src: __self_1,
                                dest: __self_2,
                            },
                            Shader::Blend {
                                blend_mode: __arg1_0,
                                src: __arg1_1,
                                dest: __arg1_2,
                            },
                        ) => __self_0 == __arg1_0 && __self_1 == __arg1_1 && __self_2 == __arg1_2,
                        (
                            Shader::LinearGradient {
                                start_xy: __self_0,
                                end_xy: __self_1,
                                colors: __self_2,
                                tile_mode: __self_3,
                            },
                            Shader::LinearGradient {
                                start_xy: __arg1_0,
                                end_xy: __arg1_1,
                                colors: __arg1_2,
                                tile_mode: __arg1_3,
                            },
                        ) => {
                            __self_0 == __arg1_0
                                && __self_1 == __arg1_1
                                && __self_2 == __arg1_2
                                && __self_3 == __arg1_3
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Shader {
            #[inline]
            fn clone(&self) -> Shader {
                match self {
                    Shader::Image {
                        src: __self_0,
                        tile_mode: __self_1,
                    } => Shader::Image {
                        src: ::core::clone::Clone::clone(__self_0),
                        tile_mode: ::core::clone::Clone::clone(__self_1),
                    },
                    Shader::Blend {
                        blend_mode: __self_0,
                        src: __self_1,
                        dest: __self_2,
                    } => Shader::Blend {
                        blend_mode: ::core::clone::Clone::clone(__self_0),
                        src: ::core::clone::Clone::clone(__self_1),
                        dest: ::core::clone::Clone::clone(__self_2),
                    },
                    Shader::LinearGradient {
                        start_xy: __self_0,
                        end_xy: __self_1,
                        colors: __self_2,
                        tile_mode: __self_3,
                    } => Shader::LinearGradient {
                        start_xy: ::core::clone::Clone::clone(__self_0),
                        end_xy: ::core::clone::Clone::clone(__self_1),
                        colors: ::core::clone::Clone::clone(__self_2),
                        tile_mode: ::core::clone::Clone::clone(__self_3),
                    },
                }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Shader {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Image>;
                let _: ::core::cmp::AssertParamIsEq<Xy<TileMode>>;
                let _: ::core::cmp::AssertParamIsEq<BlendMode>;
                let _: ::core::cmp::AssertParamIsEq<Box<Shader>>;
                let _: ::core::cmp::AssertParamIsEq<Box<Shader>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Xy<Px>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<Color>>;
                let _: ::core::cmp::AssertParamIsEq<TileMode>;
            }
        }
        impl bincode::Encode for Shader {
            fn encode<__E: bincode::enc::Encoder>(
                &self,
                encoder: &mut __E,
            ) -> core::result::Result<(), bincode::error::EncodeError> {
                match self {
                    Self::Image { src, tile_mode } => {
                        bincode::Encode::encode(&0u32, encoder)?;
                        bincode::Encode::encode(src, encoder)?;
                        bincode::Encode::encode(tile_mode, encoder)?;
                    }
                    Self::Blend {
                        blend_mode,
                        src,
                        dest,
                    } => {
                        bincode::Encode::encode(&1u32, encoder)?;
                        bincode::Encode::encode(blend_mode, encoder)?;
                        bincode::Encode::encode(src, encoder)?;
                        bincode::Encode::encode(dest, encoder)?;
                    }
                    Self::LinearGradient {
                        start_xy,
                        end_xy,
                        colors,
                        tile_mode,
                    } => {
                        bincode::Encode::encode(&2u32, encoder)?;
                        bincode::Encode::encode(start_xy, encoder)?;
                        bincode::Encode::encode(end_xy, encoder)?;
                        bincode::Encode::encode(colors, encoder)?;
                        bincode::Encode::encode(tile_mode, encoder)?;
                    }
                }
                Ok(())
            }
        }
        impl bincode::Decode<()> for Shader {
            fn decode<__D: bincode::de::Decoder<Context = ()>>(
                decoder: &mut __D,
            ) -> core::result::Result<Self, bincode::error::DecodeError> {
                let discriminant: u32 = bincode::Decode::decode(decoder)?;
                match discriminant {
                    0u32 => Ok(Self::Image {
                        src: bincode::Decode::decode(decoder)?,
                        tile_mode: bincode::Decode::decode(decoder)?,
                    }),
                    1u32 => Ok(Self::Blend {
                        blend_mode: bincode::Decode::decode(decoder)?,
                        src: bincode::Decode::decode(decoder)?,
                        dest: bincode::Decode::decode(decoder)?,
                    }),
                    2u32 => Ok(Self::LinearGradient {
                        start_xy: bincode::Decode::decode(decoder)?,
                        end_xy: bincode::Decode::decode(decoder)?,
                        colors: bincode::Decode::decode(decoder)?,
                        tile_mode: bincode::Decode::decode(decoder)?,
                    }),
                    _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                        type_name: core::any::type_name::<Self>(),
                        allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                        found: discriminant,
                    }),
                }
            }
        }
        impl Serialize for Shader {
            fn serialize(&self) -> Vec<u8> {
                use BufMutExt;
                use bytes::BufMut;
                let mut buffer = ::alloc::vec::Vec::new();
                buffer.write_string(std::any::type_name::<Self>());
                match self {
                    Self::Image { src, tile_mode } => {
                        buffer.write_string("Image");
                        buffer.write_string("src");
                        let field_bytes = Serialize::serialize(src);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("tile_mode");
                        let field_bytes = Serialize::serialize(tile_mode);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::Blend {
                        blend_mode,
                        src,
                        dest,
                    } => {
                        buffer.write_string("Blend");
                        buffer.write_string("blend_mode");
                        let field_bytes = Serialize::serialize(blend_mode);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("src");
                        let field_bytes = Serialize::serialize(src);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("dest");
                        let field_bytes = Serialize::serialize(dest);
                        buffer.put_slice(&field_bytes);
                    }
                    Self::LinearGradient {
                        start_xy,
                        end_xy,
                        colors,
                        tile_mode,
                    } => {
                        buffer.write_string("LinearGradient");
                        buffer.write_string("start_xy");
                        let field_bytes = Serialize::serialize(start_xy);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("end_xy");
                        let field_bytes = Serialize::serialize(end_xy);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("colors");
                        let field_bytes = Serialize::serialize(colors);
                        buffer.put_slice(&field_bytes);
                        buffer.write_string("tile_mode");
                        let field_bytes = Serialize::serialize(tile_mode);
                        buffer.put_slice(&field_bytes);
                    }
                }
                buffer
            }
        }
        impl Deserialize for Shader {
            fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
                use BufExt;
                use bytes::Buf;
                buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {        let variant_name = buf.read_string();
                match variant_name.as_ref() {
                    "Image" => {
                        let field_name = buf.read_name("src")?;
                        let src = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("tile_mode")?;
                        let tile_mode = Deserialize::deserialize(buf)?;
                        Ok(Self::Image { src, tile_mode })
                    }
                    "Blend" => {
                        let field_name = buf.read_name("blend_mode")?;
                        let blend_mode = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("src")?;
                        let src = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("dest")?;
                        let dest = Deserialize::deserialize(buf)?;
                        Ok(Self::Blend {
                            blend_mode,
                            src,
                            dest,
                        })
                    }
                    "LinearGradient" => {
                        let field_name = buf.read_name("start_xy")?;
                        let start_xy = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("end_xy")?;
                        let end_xy = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("colors")?;
                        let colors = Deserialize::deserialize(buf)?;
                        let field_name = buf.read_name("tile_mode")?;
                        let tile_mode = Deserialize::deserialize(buf)?;
                        Ok(Self::LinearGradient {
                            start_xy,
                            end_xy,
                            colors,
                            tile_mode,
                        })
                    }
                    _ => Err(DeserializeError::InvalidEnumVariant {
                        expected: std::any::type_name::<Self>().to_string(),
                        actual: variant_name,
                    }),
                }
            }
        }
        impl Shader {
            pub fn blend(&self, blend_mode: BlendMode, shader: &Shader) -> Shader {
                Shader::Blend {
                    blend_mode,
                    src: Box::new(self.clone()),
                    dest: Box::new(shader.clone()),
                }
            }
        }
        impl Hash for Shader {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                match self {
                    Shader::Image { src, tile_mode } => {
                        src.hash(state);
                        tile_mode.hash(state);
                    }
                    Shader::Blend {
                        blend_mode,
                        src,
                        dest,
                    } => {
                        blend_mode.hash(state);
                        src.hash(state);
                        dest.hash(state);
                    }
                    Shader::LinearGradient {
                        start_xy,
                        end_xy,
                        colors,
                        tile_mode,
                    } => {
                        start_xy.hash(state);
                        end_xy.hash(state);
                        colors.hash(state);
                        tile_mode.hash(state);
                    }
                }
            }
        }
    }
    use crate::*;
    pub use blender::*;
    pub use codes::*;
    pub use color_filter::*;
    pub use font::*;
    pub use image::*;
    pub use image_filter::*;
    pub use mask_filter::*;
    pub use paint::*;
    pub use path::*;
    pub use shader::*;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    pub struct FontMetrics {
        /// suggested space above the baseline. < 0
        pub ascent: Px,
        /// suggested space below the baseline. > 0
        pub descent: Px,
        /// suggested spacing between descent of previous line and ascent of next line.
        pub leading: Px,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FontMetrics {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "FontMetrics",
                "ascent",
                &self.ascent,
                "descent",
                &self.descent,
                "leading",
                &&self.leading,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FontMetrics {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FontMetrics {
        #[inline]
        fn eq(&self, other: &FontMetrics) -> bool {
            self.ascent == other.ascent
                && self.descent == other.descent
                && self.leading == other.leading
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FontMetrics {
        #[inline]
        fn clone(&self) -> FontMetrics {
            let _: ::core::clone::AssertParamIsClone<Px>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FontMetrics {}
    #[automatically_derived]
    impl ::core::default::Default for FontMetrics {
        #[inline]
        fn default() -> FontMetrics {
            FontMetrics {
                ascent: ::core::default::Default::default(),
                descent: ::core::default::Default::default(),
                leading: ::core::default::Default::default(),
            }
        }
    }
    impl FontMetrics {
        pub fn height(&self) -> Px {
            -self.ascent + self.descent
        }
    }
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Color {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f, "Color", "r", &self.r, "g", &self.g, "b", &self.b, "a", &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Color {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, other: &Color) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        fn clone(&self) -> Color {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Color {}
    #[automatically_derived]
    impl ::core::default::Default for Color {
        #[inline]
        fn default() -> Color {
            Color {
                r: ::core::default::Default::default(),
                g: ::core::default::Default::default(),
                b: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Color {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.r, state);
            ::core::hash::Hash::hash(&self.g, state);
            ::core::hash::Hash::hash(&self.b, state);
            ::core::hash::Hash::hash(&self.a, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Color {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    impl bincode::Encode for Color {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            bincode::Encode::encode(&self.r, encoder)?;
            bincode::Encode::encode(&self.g, encoder)?;
            bincode::Encode::encode(&self.b, encoder)?;
            bincode::Encode::encode(&self.a, encoder)?;
            Ok(())
        }
    }
    impl bincode::Decode<()> for Color {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            Ok(Self {
                r: bincode::Decode::decode(decoder)?,
                g: bincode::Decode::decode(decoder)?,
                b: bincode::Decode::decode(decoder)?,
                a: bincode::Decode::decode(decoder)?,
            })
        }
    }
    impl Serialize for Color {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            buffer.write_string("r");
            let field_bytes = Serialize::serialize(&self.r);
            buffer.put_slice(&field_bytes);
            buffer.write_string("g");
            let field_bytes = Serialize::serialize(&self.g);
            buffer.put_slice(&field_bytes);
            buffer.write_string("b");
            let field_bytes = Serialize::serialize(&self.b);
            buffer.put_slice(&field_bytes);
            buffer.write_string("a");
            let field_bytes = Serialize::serialize(&self.a);
            buffer.put_slice(&field_bytes);
            buffer
        }
    }
    impl Deserialize for Color {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let field_name = buf.read_name("r")?;
            let r = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("g")?;
            let g = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("b")?;
            let b = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("a")?;
            let a = Deserialize::deserialize(buf)?;
            Ok(Self { r, g, b, a })
        }
    }
    impl Color {
        pub const WHITE: Color = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        pub const BLACK: Color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        pub const TRANSPARENT: Color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        pub const RED: Color = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        pub const GREEN: Color = Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
        pub const BLUE: Color = Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        };
        pub fn from_f01(r: f32, g: f32, b: f32, a: f32) -> Color {
            Color {
                r: (r * 255.0) as u8,
                g: (g * 255.0) as u8,
                b: (b * 255.0) as u8,
                a: (a * 255.0) as u8,
            }
        }
        pub const fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
            Color { r, g, b, a }
        }
        pub fn grayscale_f01(value: f32) -> Color {
            Color::from_f01(value, value, value, 1.0)
        }
        pub fn grayscale_alpha_f01(value: f32, alpha: f32) -> Color {
            Color::from_f01(value, value, value, alpha)
        }
        pub const fn grayscale_u8(value: u8) -> Color {
            Color::from_u8(value, value, value, 255)
        }
        pub fn from_string_for_random_color(value: &str, is_random_alpha: bool) -> Self {
            let mut hasher = DefaultHasher::default();
            value.hash(&mut hasher);
            let hash = hasher.finish();
            Self::from_u8(
                ((hash >> 24) & 0xff) as u8,
                ((hash >> 16) & 0xff) as u8,
                ((hash >> 8) & 0xff) as u8,
                if is_random_alpha {
                    (hash & 0xff) as u8
                } else {
                    255
                },
            )
        }
        pub fn brighter(&self, value: f32) -> Self {
            let Hsl01 {
                hue,
                saturation,
                lightness,
                alpha,
            } = self.as_hsl01();
            Self::from_hsl01(Hsl01 {
                hue,
                saturation: (saturation - value).clamp(0.0, 1.0),
                lightness: (lightness + value).clamp(0.0, 1.0),
                alpha,
            })
        }
        fn as_hsl01(&self) -> Hsl01 {
            let r = self.r as f32 / 255.0;
            let g = self.g as f32 / 255.0;
            let b = self.b as f32 / 255.0;
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            let delta = max - min;
            let hue = if delta == 0.0 {
                0.0
            } else {
                60.0 * match max {
                    value if value == r => (g - b) / delta,
                    value if value == g => (b - r) / delta + 2.0,
                    value if value == b => (r - g) / delta + 4.0,
                    _ => ::core::panicking::panic("internal error: entered unreachable code"),
                }
            };
            let lightness = (max + min) / 2.0;
            let saturation = if delta == 0.0 {
                0.0
            } else {
                delta / (1.0 - (2.0 * lightness - 1.0).abs())
            };
            Hsl01 {
                hue,
                saturation,
                lightness,
                alpha: self.a as f32 / 255.0,
            }
        }
        fn from_hsl01(hsl: Hsl01) -> Self {
            let Hsl01 {
                hue,
                saturation,
                lightness,
                alpha,
            } = hsl;
            let hue = hue % 360.0;
            let hue_stage = hue / 60.0;
            let primary_chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
            let secondary_chroma = primary_chroma * (1.0 - (hue_stage % 2.0).abs());
            let (base_r, base_g, base_b) = match hue_stage {
                x if x < 1.0 => (primary_chroma, secondary_chroma, 0.0),
                x if x < 2.0 => (secondary_chroma, primary_chroma, 0.0),
                x if x < 3.0 => (0.0, primary_chroma, secondary_chroma),
                x if x < 4.0 => (0.0, secondary_chroma, primary_chroma),
                x if x < 5.0 => (secondary_chroma, 0.0, primary_chroma),
                x if x < 6.0 => (primary_chroma, 0.0, secondary_chroma),
                _ => (0.0, 0.0, 0.0),
            };
            let lightness_factor = lightness - primary_chroma / 2.0;
            Color::from_f01(
                base_r + lightness_factor,
                base_g + lightness_factor,
                base_b + lightness_factor,
                alpha,
            )
        }
        pub const fn with_alpha(mut self, alpha: u8) -> Self {
            self.a = alpha;
            self
        }
    }
    impl From<Color> for skia_safe::Color4f {
        fn from(color: Color) -> Self {
            skia_safe::Color4f::from_bytes_rgba(u32::from_le_bytes([
                color.r, color.g, color.b, color.a,
            ]))
        }
    }
    impl From<Color> for skia_safe::Color {
        fn from(color: Color) -> Self {
            skia_safe::Color::from_argb(color.a, color.r, color.g, color.b)
        }
    }
    struct Hsl01 {
        hue: f32,
        saturation: f32,
        lightness: f32,
        alpha: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Hsl01 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Hsl01",
                "hue",
                &self.hue,
                "saturation",
                &self.saturation,
                "lightness",
                &self.lightness,
                "alpha",
                &&self.alpha,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Hsl01 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Hsl01 {
        #[inline]
        fn eq(&self, other: &Hsl01) -> bool {
            self.hue == other.hue
                && self.saturation == other.saturation
                && self.lightness == other.lightness
                && self.alpha == other.alpha
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Hsl01 {
        #[inline]
        fn clone(&self) -> Hsl01 {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Hsl01 {}
    pub enum PaintStyle {
        Fill,
        Stroke,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PaintStyle {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PaintStyle::Fill => "Fill",
                    PaintStyle::Stroke => "Stroke",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PaintStyle {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PaintStyle {
        #[inline]
        fn eq(&self, other: &PaintStyle) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PaintStyle {
        #[inline]
        fn clone(&self) -> PaintStyle {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PaintStyle {}
    #[automatically_derived]
    impl ::core::hash::Hash for PaintStyle {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for PaintStyle {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for PaintStyle {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Fill => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Stroke => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for PaintStyle {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Fill),
                1u32 => Ok(Self::Stroke),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 1u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for PaintStyle {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Fill {} => {
                    buffer.write_string("Fill");
                }
                Self::Stroke {} => {
                    buffer.write_string("Stroke");
                }
            }
            buffer
        }
    }
    impl Deserialize for PaintStyle {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Fill" => Ok(Self::Fill),
                "Stroke" => Ok(Self::Stroke),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<PaintStyle> for skia_safe::PaintStyle {
        fn from(paint_style: PaintStyle) -> Self {
            match paint_style {
                PaintStyle::Fill => skia_safe::PaintStyle::Fill,
                PaintStyle::Stroke => skia_safe::PaintStyle::Stroke,
            }
        }
    }
    pub enum StrokeCap {
        Butt,
        Round,
        Square,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StrokeCap {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    StrokeCap::Butt => "Butt",
                    StrokeCap::Round => "Round",
                    StrokeCap::Square => "Square",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StrokeCap {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StrokeCap {
        #[inline]
        fn eq(&self, other: &StrokeCap) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StrokeCap {
        #[inline]
        fn clone(&self) -> StrokeCap {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StrokeCap {}
    #[automatically_derived]
    impl ::core::cmp::Eq for StrokeCap {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StrokeCap {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bincode::Encode for StrokeCap {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Butt => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Round => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Square => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for StrokeCap {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Butt),
                1u32 => Ok(Self::Round),
                2u32 => Ok(Self::Square),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for StrokeCap {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Butt {} => {
                    buffer.write_string("Butt");
                }
                Self::Round {} => {
                    buffer.write_string("Round");
                }
                Self::Square {} => {
                    buffer.write_string("Square");
                }
            }
            buffer
        }
    }
    impl Deserialize for StrokeCap {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Butt" => Ok(Self::Butt),
                "Round" => Ok(Self::Round),
                "Square" => Ok(Self::Square),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<StrokeCap> for skia_safe::PaintCap {
        fn from(stroke_cap: StrokeCap) -> Self {
            match stroke_cap {
                StrokeCap::Butt => skia_safe::PaintCap::Butt,
                StrokeCap::Round => skia_safe::PaintCap::Round,
                StrokeCap::Square => skia_safe::PaintCap::Square,
            }
        }
    }
    pub enum StrokeJoin {
        Bevel,
        Miter,
        Round,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StrokeJoin {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    StrokeJoin::Bevel => "Bevel",
                    StrokeJoin::Miter => "Miter",
                    StrokeJoin::Round => "Round",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StrokeJoin {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StrokeJoin {
        #[inline]
        fn eq(&self, other: &StrokeJoin) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StrokeJoin {
        #[inline]
        fn clone(&self) -> StrokeJoin {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StrokeJoin {}
    #[automatically_derived]
    impl ::core::cmp::Eq for StrokeJoin {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StrokeJoin {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bincode::Encode for StrokeJoin {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Bevel => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Miter => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Round => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for StrokeJoin {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Bevel),
                1u32 => Ok(Self::Miter),
                2u32 => Ok(Self::Round),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for StrokeJoin {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Bevel {} => {
                    buffer.write_string("Bevel");
                }
                Self::Miter {} => {
                    buffer.write_string("Miter");
                }
                Self::Round {} => {
                    buffer.write_string("Round");
                }
            }
            buffer
        }
    }
    impl Deserialize for StrokeJoin {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Bevel" => Ok(Self::Bevel),
                "Miter" => Ok(Self::Miter),
                "Round" => Ok(Self::Round),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<StrokeJoin> for skia_safe::PaintJoin {
        fn from(stroke_join: StrokeJoin) -> Self {
            match stroke_join {
                StrokeJoin::Bevel => skia_safe::PaintJoin::Bevel,
                StrokeJoin::Miter => skia_safe::PaintJoin::Miter,
                StrokeJoin::Round => skia_safe::PaintJoin::Round,
            }
        }
    }
    pub struct StrokeOptions {
        pub width: Option<Px>,
        pub miter_limit: Option<Px>,
        ///
        /// if > 1, increase precision, else if (0 < resScale < 1) reduce precision to
        /// favor speed and size
        ///
        pub precision: Option<OrderedFloat>,
        pub join: Option<StrokeJoin>,
        pub cap: Option<StrokeCap>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StrokeOptions {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "StrokeOptions",
                "width",
                &self.width,
                "miter_limit",
                &self.miter_limit,
                "precision",
                &self.precision,
                "join",
                &self.join,
                "cap",
                &&self.cap,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StrokeOptions {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StrokeOptions {
        #[inline]
        fn eq(&self, other: &StrokeOptions) -> bool {
            self.width == other.width
                && self.miter_limit == other.miter_limit
                && self.precision == other.precision
                && self.join == other.join
                && self.cap == other.cap
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StrokeOptions {
        #[inline]
        fn clone(&self) -> StrokeOptions {
            let _: ::core::clone::AssertParamIsClone<Option<Px>>;
            let _: ::core::clone::AssertParamIsClone<Option<Px>>;
            let _: ::core::clone::AssertParamIsClone<Option<OrderedFloat>>;
            let _: ::core::clone::AssertParamIsClone<Option<StrokeJoin>>;
            let _: ::core::clone::AssertParamIsClone<Option<StrokeCap>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StrokeOptions {}
    #[automatically_derived]
    impl ::core::cmp::Eq for StrokeOptions {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Option<Px>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Px>>;
            let _: ::core::cmp::AssertParamIsEq<Option<OrderedFloat>>;
            let _: ::core::cmp::AssertParamIsEq<Option<StrokeJoin>>;
            let _: ::core::cmp::AssertParamIsEq<Option<StrokeCap>>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for StrokeOptions {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.width, state);
            ::core::hash::Hash::hash(&self.miter_limit, state);
            ::core::hash::Hash::hash(&self.precision, state);
            ::core::hash::Hash::hash(&self.join, state);
            ::core::hash::Hash::hash(&self.cap, state)
        }
    }
    impl bincode::Encode for StrokeOptions {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            bincode::Encode::encode(&self.width, encoder)?;
            bincode::Encode::encode(&self.miter_limit, encoder)?;
            bincode::Encode::encode(&self.precision, encoder)?;
            bincode::Encode::encode(&self.join, encoder)?;
            bincode::Encode::encode(&self.cap, encoder)?;
            Ok(())
        }
    }
    impl bincode::Decode<()> for StrokeOptions {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            Ok(Self {
                width: bincode::Decode::decode(decoder)?,
                miter_limit: bincode::Decode::decode(decoder)?,
                precision: bincode::Decode::decode(decoder)?,
                join: bincode::Decode::decode(decoder)?,
                cap: bincode::Decode::decode(decoder)?,
            })
        }
    }
    impl Serialize for StrokeOptions {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            buffer.write_string("width");
            let field_bytes = Serialize::serialize(&self.width);
            buffer.put_slice(&field_bytes);
            buffer.write_string("miter_limit");
            let field_bytes = Serialize::serialize(&self.miter_limit);
            buffer.put_slice(&field_bytes);
            buffer.write_string("precision");
            let field_bytes = Serialize::serialize(&self.precision);
            buffer.put_slice(&field_bytes);
            buffer.write_string("join");
            let field_bytes = Serialize::serialize(&self.join);
            buffer.put_slice(&field_bytes);
            buffer.write_string("cap");
            let field_bytes = Serialize::serialize(&self.cap);
            buffer.put_slice(&field_bytes);
            buffer
        }
    }
    impl Deserialize for StrokeOptions {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let field_name = buf.read_name("width")?;
            let width = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("miter_limit")?;
            let miter_limit = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("precision")?;
            let precision = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("join")?;
            let join = Deserialize::deserialize(buf)?;
            let field_name = buf.read_name("cap")?;
            let cap = Deserialize::deserialize(buf)?;
            Ok(Self {
                width,
                miter_limit,
                precision,
                join,
                cap,
            })
        }
    }
    pub enum ClipOp {
        Intersect,
        Difference,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ClipOp {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ClipOp::Intersect => "Intersect",
                    ClipOp::Difference => "Difference",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ClipOp {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ClipOp {
        #[inline]
        fn eq(&self, other: &ClipOp) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ClipOp {
        #[inline]
        fn clone(&self) -> ClipOp {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ClipOp {}
    #[automatically_derived]
    impl ::core::hash::Hash for ClipOp {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ClipOp {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for ClipOp {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Intersect => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Difference => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for ClipOp {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Intersect),
                1u32 => Ok(Self::Difference),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 1u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for ClipOp {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Intersect {} => {
                    buffer.write_string("Intersect");
                }
                Self::Difference {} => {
                    buffer.write_string("Difference");
                }
            }
            buffer
        }
    }
    impl Deserialize for ClipOp {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Intersect" => Ok(Self::Intersect),
                "Difference" => Ok(Self::Difference),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<ClipOp> for skia_safe::ClipOp {
        fn from(clip_op: ClipOp) -> Self {
            match clip_op {
                ClipOp::Intersect => skia_safe::ClipOp::Intersect,
                ClipOp::Difference => skia_safe::ClipOp::Difference,
            }
        }
    }
    pub enum AlphaType {
        Opaque,
        Premul,
        Unpremul,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AlphaType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AlphaType::Opaque => "Opaque",
                    AlphaType::Premul => "Premul",
                    AlphaType::Unpremul => "Unpremul",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AlphaType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AlphaType {
        #[inline]
        fn eq(&self, other: &AlphaType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AlphaType {
        #[inline]
        fn clone(&self) -> AlphaType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AlphaType {}
    #[automatically_derived]
    impl ::core::hash::Hash for AlphaType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bincode::Encode for AlphaType {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Opaque => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Premul => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Unpremul => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for AlphaType {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Opaque),
                1u32 => Ok(Self::Premul),
                2u32 => Ok(Self::Unpremul),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for AlphaType {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Opaque {} => {
                    buffer.write_string("Opaque");
                }
                Self::Premul {} => {
                    buffer.write_string("Premul");
                }
                Self::Unpremul {} => {
                    buffer.write_string("Unpremul");
                }
            }
            buffer
        }
    }
    impl Deserialize for AlphaType {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Opaque" => Ok(Self::Opaque),
                "Premul" => Ok(Self::Premul),
                "Unpremul" => Ok(Self::Unpremul),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<u8> for AlphaType {
        fn from(val: u8) -> Self {
            match val {
                0 => AlphaType::Opaque,
                1 => AlphaType::Premul,
                2 => AlphaType::Unpremul,
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("invalid alpha type {0}", val),
                    ));
                }
            }
        }
    }
    impl From<AlphaType> for u8 {
        fn from(val: AlphaType) -> Self {
            match val {
                AlphaType::Opaque => 0,
                AlphaType::Premul => 1,
                AlphaType::Unpremul => 2,
            }
        }
    }
    impl From<skia_safe::AlphaType> for AlphaType {
        fn from(val: skia_safe::AlphaType) -> Self {
            match val {
                skia_safe::AlphaType::Opaque => AlphaType::Opaque,
                skia_safe::AlphaType::Premul => AlphaType::Premul,
                skia_safe::AlphaType::Unpremul => AlphaType::Unpremul,
                skia_safe::AlphaType::Unknown => {
                    ::core::panicking::panic_fmt(format_args!(
                        "not implemented: {0}",
                        format_args!("canvaskit doesn\'t support AlphaType::Unknown"),
                    ));
                }
            }
        }
    }
    impl From<AlphaType> for skia_safe::AlphaType {
        fn from(val: AlphaType) -> Self {
            match val {
                AlphaType::Opaque => skia_safe::AlphaType::Opaque,
                AlphaType::Premul => skia_safe::AlphaType::Premul,
                AlphaType::Unpremul => skia_safe::AlphaType::Unpremul,
            }
        }
    }
    pub enum ColorType {
        Alpha8,
        Rgb565,
        Rgba8888,
        Bgra8888,
        Rgba1010102,
        Rgb101010x,
        Gray8,
        RgbaF16,
        RgbaF32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ColorType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ColorType::Alpha8 => "Alpha8",
                    ColorType::Rgb565 => "Rgb565",
                    ColorType::Rgba8888 => "Rgba8888",
                    ColorType::Bgra8888 => "Bgra8888",
                    ColorType::Rgba1010102 => "Rgba1010102",
                    ColorType::Rgb101010x => "Rgb101010x",
                    ColorType::Gray8 => "Gray8",
                    ColorType::RgbaF16 => "RgbaF16",
                    ColorType::RgbaF32 => "RgbaF32",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ColorType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ColorType {
        #[inline]
        fn eq(&self, other: &ColorType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ColorType {
        #[inline]
        fn clone(&self) -> ColorType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ColorType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ColorType {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ColorType {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bincode::Encode for ColorType {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Alpha8 => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Rgb565 => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Rgba8888 => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
                Self::Bgra8888 => {
                    bincode::Encode::encode(&3u32, encoder)?;
                }
                Self::Rgba1010102 => {
                    bincode::Encode::encode(&4u32, encoder)?;
                }
                Self::Rgb101010x => {
                    bincode::Encode::encode(&5u32, encoder)?;
                }
                Self::Gray8 => {
                    bincode::Encode::encode(&6u32, encoder)?;
                }
                Self::RgbaF16 => {
                    bincode::Encode::encode(&7u32, encoder)?;
                }
                Self::RgbaF32 => {
                    bincode::Encode::encode(&8u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for ColorType {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Alpha8),
                1u32 => Ok(Self::Rgb565),
                2u32 => Ok(Self::Rgba8888),
                3u32 => Ok(Self::Bgra8888),
                4u32 => Ok(Self::Rgba1010102),
                5u32 => Ok(Self::Rgb101010x),
                6u32 => Ok(Self::Gray8),
                7u32 => Ok(Self::RgbaF16),
                8u32 => Ok(Self::RgbaF32),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 8u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for ColorType {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Alpha8 {} => {
                    buffer.write_string("Alpha8");
                }
                Self::Rgb565 {} => {
                    buffer.write_string("Rgb565");
                }
                Self::Rgba8888 {} => {
                    buffer.write_string("Rgba8888");
                }
                Self::Bgra8888 {} => {
                    buffer.write_string("Bgra8888");
                }
                Self::Rgba1010102 {} => {
                    buffer.write_string("Rgba1010102");
                }
                Self::Rgb101010x {} => {
                    buffer.write_string("Rgb101010x");
                }
                Self::Gray8 {} => {
                    buffer.write_string("Gray8");
                }
                Self::RgbaF16 {} => {
                    buffer.write_string("RgbaF16");
                }
                Self::RgbaF32 {} => {
                    buffer.write_string("RgbaF32");
                }
            }
            buffer
        }
    }
    impl Deserialize for ColorType {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Alpha8" => Ok(Self::Alpha8),
                "Rgb565" => Ok(Self::Rgb565),
                "Rgba8888" => Ok(Self::Rgba8888),
                "Bgra8888" => Ok(Self::Bgra8888),
                "Rgba1010102" => Ok(Self::Rgba1010102),
                "Rgb101010x" => Ok(Self::Rgb101010x),
                "Gray8" => Ok(Self::Gray8),
                "RgbaF16" => Ok(Self::RgbaF16),
                "RgbaF32" => Ok(Self::RgbaF32),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl ColorType {
        pub fn word(&self) -> usize {
            match self {
                ColorType::Alpha8 => 1,
                ColorType::Rgb565 => 2,
                ColorType::Rgba8888 => 4,
                ColorType::Bgra8888 => 4,
                ColorType::Rgba1010102 => 4,
                ColorType::Rgb101010x => 4,
                ColorType::Gray8 => 1,
                ColorType::RgbaF16 => 8,
                ColorType::RgbaF32 => 16,
            }
        }
    }
    impl From<u8> for ColorType {
        fn from(val: u8) -> Self {
            match val {
                0 => ColorType::Alpha8,
                1 => ColorType::Rgb565,
                2 => ColorType::Rgba8888,
                3 => ColorType::Bgra8888,
                4 => ColorType::Rgba1010102,
                5 => ColorType::Rgb101010x,
                6 => ColorType::Gray8,
                7 => ColorType::RgbaF16,
                8 => ColorType::RgbaF32,
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("invalid color type {0}", val),
                    ));
                }
            }
        }
    }
    impl From<ColorType> for u8 {
        fn from(val: ColorType) -> Self {
            match val {
                ColorType::Alpha8 => 0,
                ColorType::Rgb565 => 1,
                ColorType::Rgba8888 => 2,
                ColorType::Bgra8888 => 3,
                ColorType::Rgba1010102 => 4,
                ColorType::Rgb101010x => 5,
                ColorType::Gray8 => 6,
                ColorType::RgbaF16 => 7,
                ColorType::RgbaF32 => 8,
            }
        }
    }
    impl From<skia_safe::ColorType> for ColorType {
        fn from(val: skia_safe::ColorType) -> Self {
            match val {
                skia_safe::ColorType::Alpha8 => ColorType::Alpha8,
                skia_safe::ColorType::RGB565 => ColorType::Rgb565,
                skia_safe::ColorType::RGBA8888 => ColorType::Rgba8888,
                skia_safe::ColorType::BGRA8888 => ColorType::Bgra8888,
                skia_safe::ColorType::RGBA1010102 => ColorType::Rgba1010102,
                skia_safe::ColorType::RGB101010x => ColorType::Rgb101010x,
                skia_safe::ColorType::Gray8 => ColorType::Gray8,
                skia_safe::ColorType::RGBAF16 => ColorType::RgbaF16,
                skia_safe::ColorType::RGBAF32 => ColorType::RgbaF32,
                _ => ::core::panicking::panic("not implemented"),
            }
        }
    }
    impl From<ColorType> for skia_safe::ColorType {
        fn from(val: ColorType) -> Self {
            match val {
                ColorType::Alpha8 => skia_safe::ColorType::Alpha8,
                ColorType::Rgb565 => skia_safe::ColorType::RGB565,
                ColorType::Rgba8888 => skia_safe::ColorType::RGBA8888,
                ColorType::Bgra8888 => skia_safe::ColorType::BGRA8888,
                ColorType::Rgba1010102 => skia_safe::ColorType::RGBA1010102,
                ColorType::Rgb101010x => skia_safe::ColorType::RGB101010x,
                ColorType::Gray8 => skia_safe::ColorType::Gray8,
                ColorType::RgbaF16 => skia_safe::ColorType::RGBAF16,
                ColorType::RgbaF32 => skia_safe::ColorType::RGBAF32,
            }
        }
    }
    pub enum FilterMode {
        Linear,
        Nearest,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FilterMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    FilterMode::Linear => "Linear",
                    FilterMode::Nearest => "Nearest",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FilterMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FilterMode {
        #[inline]
        fn eq(&self, other: &FilterMode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for FilterMode {
        #[inline]
        fn clone(&self) -> FilterMode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FilterMode {}
    impl From<FilterMode> for skia_safe::FilterMode {
        fn from(filter_mode: FilterMode) -> Self {
            match filter_mode {
                FilterMode::Linear => skia_safe::FilterMode::Linear,
                FilterMode::Nearest => skia_safe::FilterMode::Nearest,
            }
        }
    }
    pub enum MipmapMode {
        None,
        Nearest,
        Linear,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MipmapMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MipmapMode::None => "None",
                    MipmapMode::Nearest => "Nearest",
                    MipmapMode::Linear => "Linear",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MipmapMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MipmapMode {
        #[inline]
        fn eq(&self, other: &MipmapMode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MipmapMode {
        #[inline]
        fn clone(&self) -> MipmapMode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MipmapMode {}
    impl From<MipmapMode> for skia_safe::MipmapMode {
        fn from(mipmap_mode: MipmapMode) -> Self {
            match mipmap_mode {
                MipmapMode::None => skia_safe::MipmapMode::None,
                MipmapMode::Nearest => skia_safe::MipmapMode::Nearest,
                MipmapMode::Linear => skia_safe::MipmapMode::Linear,
            }
        }
    }
    pub enum BlendMode {
        Clear,
        Src,
        Dst,
        SrcOver,
        DstOver,
        SrcIn,
        DstIn,
        SrcOut,
        DstOut,
        SrcATop,
        DstATop,
        Xor,
        Plus,
        Modulate,
        Screen,
        Overlay,
        Darken,
        Lighten,
        ColorDodge,
        ColorBurn,
        HardLight,
        SoftLight,
        Difference,
        Exclusion,
        Multiply,
        Hue,
        Saturation,
        Color,
        Luminosity,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BlendMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    BlendMode::Clear => "Clear",
                    BlendMode::Src => "Src",
                    BlendMode::Dst => "Dst",
                    BlendMode::SrcOver => "SrcOver",
                    BlendMode::DstOver => "DstOver",
                    BlendMode::SrcIn => "SrcIn",
                    BlendMode::DstIn => "DstIn",
                    BlendMode::SrcOut => "SrcOut",
                    BlendMode::DstOut => "DstOut",
                    BlendMode::SrcATop => "SrcATop",
                    BlendMode::DstATop => "DstATop",
                    BlendMode::Xor => "Xor",
                    BlendMode::Plus => "Plus",
                    BlendMode::Modulate => "Modulate",
                    BlendMode::Screen => "Screen",
                    BlendMode::Overlay => "Overlay",
                    BlendMode::Darken => "Darken",
                    BlendMode::Lighten => "Lighten",
                    BlendMode::ColorDodge => "ColorDodge",
                    BlendMode::ColorBurn => "ColorBurn",
                    BlendMode::HardLight => "HardLight",
                    BlendMode::SoftLight => "SoftLight",
                    BlendMode::Difference => "Difference",
                    BlendMode::Exclusion => "Exclusion",
                    BlendMode::Multiply => "Multiply",
                    BlendMode::Hue => "Hue",
                    BlendMode::Saturation => "Saturation",
                    BlendMode::Color => "Color",
                    BlendMode::Luminosity => "Luminosity",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BlendMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BlendMode {
        #[inline]
        fn eq(&self, other: &BlendMode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BlendMode {
        #[inline]
        fn clone(&self) -> BlendMode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for BlendMode {}
    #[automatically_derived]
    impl ::core::cmp::Eq for BlendMode {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BlendMode {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl bincode::Encode for BlendMode {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Clear => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Src => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Dst => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
                Self::SrcOver => {
                    bincode::Encode::encode(&3u32, encoder)?;
                }
                Self::DstOver => {
                    bincode::Encode::encode(&4u32, encoder)?;
                }
                Self::SrcIn => {
                    bincode::Encode::encode(&5u32, encoder)?;
                }
                Self::DstIn => {
                    bincode::Encode::encode(&6u32, encoder)?;
                }
                Self::SrcOut => {
                    bincode::Encode::encode(&7u32, encoder)?;
                }
                Self::DstOut => {
                    bincode::Encode::encode(&8u32, encoder)?;
                }
                Self::SrcATop => {
                    bincode::Encode::encode(&9u32, encoder)?;
                }
                Self::DstATop => {
                    bincode::Encode::encode(&10u32, encoder)?;
                }
                Self::Xor => {
                    bincode::Encode::encode(&11u32, encoder)?;
                }
                Self::Plus => {
                    bincode::Encode::encode(&12u32, encoder)?;
                }
                Self::Modulate => {
                    bincode::Encode::encode(&13u32, encoder)?;
                }
                Self::Screen => {
                    bincode::Encode::encode(&14u32, encoder)?;
                }
                Self::Overlay => {
                    bincode::Encode::encode(&15u32, encoder)?;
                }
                Self::Darken => {
                    bincode::Encode::encode(&16u32, encoder)?;
                }
                Self::Lighten => {
                    bincode::Encode::encode(&17u32, encoder)?;
                }
                Self::ColorDodge => {
                    bincode::Encode::encode(&18u32, encoder)?;
                }
                Self::ColorBurn => {
                    bincode::Encode::encode(&19u32, encoder)?;
                }
                Self::HardLight => {
                    bincode::Encode::encode(&20u32, encoder)?;
                }
                Self::SoftLight => {
                    bincode::Encode::encode(&21u32, encoder)?;
                }
                Self::Difference => {
                    bincode::Encode::encode(&22u32, encoder)?;
                }
                Self::Exclusion => {
                    bincode::Encode::encode(&23u32, encoder)?;
                }
                Self::Multiply => {
                    bincode::Encode::encode(&24u32, encoder)?;
                }
                Self::Hue => {
                    bincode::Encode::encode(&25u32, encoder)?;
                }
                Self::Saturation => {
                    bincode::Encode::encode(&26u32, encoder)?;
                }
                Self::Color => {
                    bincode::Encode::encode(&27u32, encoder)?;
                }
                Self::Luminosity => {
                    bincode::Encode::encode(&28u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for BlendMode {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Clear),
                1u32 => Ok(Self::Src),
                2u32 => Ok(Self::Dst),
                3u32 => Ok(Self::SrcOver),
                4u32 => Ok(Self::DstOver),
                5u32 => Ok(Self::SrcIn),
                6u32 => Ok(Self::DstIn),
                7u32 => Ok(Self::SrcOut),
                8u32 => Ok(Self::DstOut),
                9u32 => Ok(Self::SrcATop),
                10u32 => Ok(Self::DstATop),
                11u32 => Ok(Self::Xor),
                12u32 => Ok(Self::Plus),
                13u32 => Ok(Self::Modulate),
                14u32 => Ok(Self::Screen),
                15u32 => Ok(Self::Overlay),
                16u32 => Ok(Self::Darken),
                17u32 => Ok(Self::Lighten),
                18u32 => Ok(Self::ColorDodge),
                19u32 => Ok(Self::ColorBurn),
                20u32 => Ok(Self::HardLight),
                21u32 => Ok(Self::SoftLight),
                22u32 => Ok(Self::Difference),
                23u32 => Ok(Self::Exclusion),
                24u32 => Ok(Self::Multiply),
                25u32 => Ok(Self::Hue),
                26u32 => Ok(Self::Saturation),
                27u32 => Ok(Self::Color),
                28u32 => Ok(Self::Luminosity),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 28u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for BlendMode {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Clear {} => {
                    buffer.write_string("Clear");
                }
                Self::Src {} => {
                    buffer.write_string("Src");
                }
                Self::Dst {} => {
                    buffer.write_string("Dst");
                }
                Self::SrcOver {} => {
                    buffer.write_string("SrcOver");
                }
                Self::DstOver {} => {
                    buffer.write_string("DstOver");
                }
                Self::SrcIn {} => {
                    buffer.write_string("SrcIn");
                }
                Self::DstIn {} => {
                    buffer.write_string("DstIn");
                }
                Self::SrcOut {} => {
                    buffer.write_string("SrcOut");
                }
                Self::DstOut {} => {
                    buffer.write_string("DstOut");
                }
                Self::SrcATop {} => {
                    buffer.write_string("SrcATop");
                }
                Self::DstATop {} => {
                    buffer.write_string("DstATop");
                }
                Self::Xor {} => {
                    buffer.write_string("Xor");
                }
                Self::Plus {} => {
                    buffer.write_string("Plus");
                }
                Self::Modulate {} => {
                    buffer.write_string("Modulate");
                }
                Self::Screen {} => {
                    buffer.write_string("Screen");
                }
                Self::Overlay {} => {
                    buffer.write_string("Overlay");
                }
                Self::Darken {} => {
                    buffer.write_string("Darken");
                }
                Self::Lighten {} => {
                    buffer.write_string("Lighten");
                }
                Self::ColorDodge {} => {
                    buffer.write_string("ColorDodge");
                }
                Self::ColorBurn {} => {
                    buffer.write_string("ColorBurn");
                }
                Self::HardLight {} => {
                    buffer.write_string("HardLight");
                }
                Self::SoftLight {} => {
                    buffer.write_string("SoftLight");
                }
                Self::Difference {} => {
                    buffer.write_string("Difference");
                }
                Self::Exclusion {} => {
                    buffer.write_string("Exclusion");
                }
                Self::Multiply {} => {
                    buffer.write_string("Multiply");
                }
                Self::Hue {} => {
                    buffer.write_string("Hue");
                }
                Self::Saturation {} => {
                    buffer.write_string("Saturation");
                }
                Self::Color {} => {
                    buffer.write_string("Color");
                }
                Self::Luminosity {} => {
                    buffer.write_string("Luminosity");
                }
            }
            buffer
        }
    }
    impl Deserialize for BlendMode {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Clear" => Ok(Self::Clear),
                "Src" => Ok(Self::Src),
                "Dst" => Ok(Self::Dst),
                "SrcOver" => Ok(Self::SrcOver),
                "DstOver" => Ok(Self::DstOver),
                "SrcIn" => Ok(Self::SrcIn),
                "DstIn" => Ok(Self::DstIn),
                "SrcOut" => Ok(Self::SrcOut),
                "DstOut" => Ok(Self::DstOut),
                "SrcATop" => Ok(Self::SrcATop),
                "DstATop" => Ok(Self::DstATop),
                "Xor" => Ok(Self::Xor),
                "Plus" => Ok(Self::Plus),
                "Modulate" => Ok(Self::Modulate),
                "Screen" => Ok(Self::Screen),
                "Overlay" => Ok(Self::Overlay),
                "Darken" => Ok(Self::Darken),
                "Lighten" => Ok(Self::Lighten),
                "ColorDodge" => Ok(Self::ColorDodge),
                "ColorBurn" => Ok(Self::ColorBurn),
                "HardLight" => Ok(Self::HardLight),
                "SoftLight" => Ok(Self::SoftLight),
                "Difference" => Ok(Self::Difference),
                "Exclusion" => Ok(Self::Exclusion),
                "Multiply" => Ok(Self::Multiply),
                "Hue" => Ok(Self::Hue),
                "Saturation" => Ok(Self::Saturation),
                "Color" => Ok(Self::Color),
                "Luminosity" => Ok(Self::Luminosity),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<BlendMode> for skia_safe::BlendMode {
        fn from(blend_mode: BlendMode) -> Self {
            match blend_mode {
                BlendMode::Clear => skia_safe::BlendMode::Clear,
                BlendMode::Src => skia_safe::BlendMode::Src,
                BlendMode::Dst => skia_safe::BlendMode::Dst,
                BlendMode::SrcOver => skia_safe::BlendMode::SrcOver,
                BlendMode::DstOver => skia_safe::BlendMode::DstOver,
                BlendMode::SrcIn => skia_safe::BlendMode::SrcIn,
                BlendMode::DstIn => skia_safe::BlendMode::DstIn,
                BlendMode::SrcOut => skia_safe::BlendMode::SrcOut,
                BlendMode::DstOut => skia_safe::BlendMode::DstOut,
                BlendMode::SrcATop => skia_safe::BlendMode::SrcATop,
                BlendMode::DstATop => skia_safe::BlendMode::DstATop,
                BlendMode::Xor => skia_safe::BlendMode::Xor,
                BlendMode::Plus => skia_safe::BlendMode::Plus,
                BlendMode::Modulate => skia_safe::BlendMode::Modulate,
                BlendMode::Screen => skia_safe::BlendMode::Screen,
                BlendMode::Overlay => skia_safe::BlendMode::Overlay,
                BlendMode::Darken => skia_safe::BlendMode::Darken,
                BlendMode::Lighten => skia_safe::BlendMode::Lighten,
                BlendMode::ColorDodge => skia_safe::BlendMode::ColorDodge,
                BlendMode::ColorBurn => skia_safe::BlendMode::ColorBurn,
                BlendMode::HardLight => skia_safe::BlendMode::HardLight,
                BlendMode::SoftLight => skia_safe::BlendMode::SoftLight,
                BlendMode::Difference => skia_safe::BlendMode::Difference,
                BlendMode::Exclusion => skia_safe::BlendMode::Exclusion,
                BlendMode::Multiply => skia_safe::BlendMode::Multiply,
                BlendMode::Hue => skia_safe::BlendMode::Hue,
                BlendMode::Saturation => skia_safe::BlendMode::Saturation,
                BlendMode::Color => skia_safe::BlendMode::Color,
                BlendMode::Luminosity => skia_safe::BlendMode::Luminosity,
            }
        }
    }
    /// Explain: https://developer.android.com/reference/android/graphics/Shader.TileMode#summary
    pub enum TileMode {
        /// Replicate the edge color if the shader draws outside of its original bounds
        Clamp,
        /// Render the shader's image pixels only within its original bounds.
        Decal,
        /// Repeat the shader's image horizontally and vertically, alternating mirror images so that adjacent images always seam
        Mirror,
        /// Repeat the shader's image horizontally and vertically.
        Repeat,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TileMode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TileMode::Clamp => "Clamp",
                    TileMode::Decal => "Decal",
                    TileMode::Mirror => "Mirror",
                    TileMode::Repeat => "Repeat",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TileMode {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TileMode {
        #[inline]
        fn eq(&self, other: &TileMode) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TileMode {
        #[inline]
        fn clone(&self) -> TileMode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TileMode {}
    #[automatically_derived]
    impl ::core::hash::Hash for TileMode {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TileMode {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for TileMode {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Clamp => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Decal => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Mirror => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
                Self::Repeat => {
                    bincode::Encode::encode(&3u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for TileMode {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Clamp),
                1u32 => Ok(Self::Decal),
                2u32 => Ok(Self::Mirror),
                3u32 => Ok(Self::Repeat),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 3u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for TileMode {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Clamp {} => {
                    buffer.write_string("Clamp");
                }
                Self::Decal {} => {
                    buffer.write_string("Decal");
                }
                Self::Mirror {} => {
                    buffer.write_string("Mirror");
                }
                Self::Repeat {} => {
                    buffer.write_string("Repeat");
                }
            }
            buffer
        }
    }
    impl Deserialize for TileMode {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Clamp" => Ok(Self::Clamp),
                "Decal" => Ok(Self::Decal),
                "Mirror" => Ok(Self::Mirror),
                "Repeat" => Ok(Self::Repeat),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    impl From<TileMode> for skia_safe::TileMode {
        fn from(tile_mode: TileMode) -> Self {
            match tile_mode {
                TileMode::Clamp => skia_safe::TileMode::Clamp,
                TileMode::Decal => skia_safe::TileMode::Decal,
                TileMode::Mirror => skia_safe::TileMode::Mirror,
                TileMode::Repeat => skia_safe::TileMode::Repeat,
            }
        }
    }
    pub enum ColorSpace {
        Srgb,
        DisplayP3,
        AdobeRgb,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ColorSpace {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ColorSpace::Srgb => "Srgb",
                    ColorSpace::DisplayP3 => "DisplayP3",
                    ColorSpace::AdobeRgb => "AdobeRgb",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ColorSpace {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ColorSpace {
        #[inline]
        fn eq(&self, other: &ColorSpace) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ColorSpace {
        #[inline]
        fn clone(&self) -> ColorSpace {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ColorSpace {}
    pub enum TextAlign {
        Left,
        Center,
        Right,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TextAlign {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TextAlign::Left => "Left",
                    TextAlign::Center => "Center",
                    TextAlign::Right => "Right",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TextAlign {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TextAlign {
        #[inline]
        fn eq(&self, other: &TextAlign) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TextAlign {
        #[inline]
        fn clone(&self) -> TextAlign {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TextAlign {}
    #[automatically_derived]
    impl ::core::hash::Hash for TextAlign {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TextAlign {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for TextAlign {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Left => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Center => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Right => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for TextAlign {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Left),
                1u32 => Ok(Self::Center),
                2u32 => Ok(Self::Right),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for TextAlign {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Left {} => {
                    buffer.write_string("Left");
                }
                Self::Center {} => {
                    buffer.write_string("Center");
                }
                Self::Right {} => {
                    buffer.write_string("Right");
                }
            }
            buffer
        }
    }
    impl Deserialize for TextAlign {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Left" => Ok(Self::Left),
                "Center" => Ok(Self::Center),
                "Right" => Ok(Self::Right),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    pub enum TextBaseline {
        Top,
        Middle,
        Bottom,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TextBaseline {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    TextBaseline::Top => "Top",
                    TextBaseline::Middle => "Middle",
                    TextBaseline::Bottom => "Bottom",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TextBaseline {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TextBaseline {
        #[inline]
        fn eq(&self, other: &TextBaseline) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TextBaseline {
        #[inline]
        fn clone(&self) -> TextBaseline {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for TextBaseline {}
    #[automatically_derived]
    impl ::core::hash::Hash for TextBaseline {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for TextBaseline {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for TextBaseline {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Top => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Middle => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Bottom => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for TextBaseline {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Top),
                1u32 => Ok(Self::Middle),
                2u32 => Ok(Self::Bottom),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for TextBaseline {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Top {} => {
                    buffer.write_string("Top");
                }
                Self::Middle {} => {
                    buffer.write_string("Middle");
                }
                Self::Bottom {} => {
                    buffer.write_string("Bottom");
                }
            }
            buffer
        }
    }
    impl Deserialize for TextBaseline {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Top" => Ok(Self::Top),
                "Middle" => Ok(Self::Middle),
                "Bottom" => Ok(Self::Bottom),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    /// Example: https://developer.mozilla.org/ko/docs/Web/CSS/object-fit
    pub enum ImageFit {
        /// The replaced content is sized to fill the element's content box.
        /// The entire object will completely fill the box.
        /// If the object's aspect ratio does not match the aspect ratio of its box,
        /// then the object will be stretched to fit.
        Fill,
        /// The replaced content is scaled to maintain its aspect ratio while fitting within the element's content box.
        /// The entire object is made to fill the box, while preserving its aspect ratio, so the object will be letterboxed
        /// if its aspect ratio does not match the aspect ratio of the box.
        Contain,
        /// The replaced content is sized to maintain its aspect ratio while filling the element's entire content box.
        /// If the object's aspect ratio does not match the aspect ratio of its box, then the object will be clipped to fit.
        Cover,
        /// The content is sized as if `none` or `contain` were specified, whichever would result in a smaller concrete object size.
        ScaleDown,
        /// The replaced content is not resized.
        None,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ImageFit {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ImageFit::Fill => "Fill",
                    ImageFit::Contain => "Contain",
                    ImageFit::Cover => "Cover",
                    ImageFit::ScaleDown => "ScaleDown",
                    ImageFit::None => "None",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ImageFit {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ImageFit {
        #[inline]
        fn eq(&self, other: &ImageFit) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ImageFit {
        #[inline]
        fn clone(&self) -> ImageFit {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ImageFit {}
    #[automatically_derived]
    impl ::core::hash::Hash for ImageFit {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ImageFit {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for ImageFit {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Fill => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Contain => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Cover => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
                Self::ScaleDown => {
                    bincode::Encode::encode(&3u32, encoder)?;
                }
                Self::None => {
                    bincode::Encode::encode(&4u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for ImageFit {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Fill),
                1u32 => Ok(Self::Contain),
                2u32 => Ok(Self::Cover),
                3u32 => Ok(Self::ScaleDown),
                4u32 => Ok(Self::None),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 4u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for ImageFit {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Fill {} => {
                    buffer.write_string("Fill");
                }
                Self::Contain {} => {
                    buffer.write_string("Contain");
                }
                Self::Cover {} => {
                    buffer.write_string("Cover");
                }
                Self::ScaleDown {} => {
                    buffer.write_string("ScaleDown");
                }
                Self::None {} => {
                    buffer.write_string("None");
                }
            }
            buffer
        }
    }
    impl Deserialize for ImageFit {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Fill" => Ok(Self::Fill),
                "Contain" => Ok(Self::Contain),
                "Cover" => Ok(Self::Cover),
                "ScaleDown" => Ok(Self::ScaleDown),
                "None" => Ok(Self::None),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
    pub type GlyphId = skia_safe::GlyphId;
    pub type GlyphIds = Vec<GlyphId>;
    pub enum MouseButton {
        Left,
        Middle,
        Right,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MouseButton {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MouseButton::Left => "Left",
                    MouseButton::Middle => "Middle",
                    MouseButton::Right => "Right",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MouseButton {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MouseButton {
        #[inline]
        fn eq(&self, other: &MouseButton) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MouseButton {
        #[inline]
        fn clone(&self) -> MouseButton {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MouseButton {}
    #[automatically_derived]
    impl ::core::hash::Hash for MouseButton {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MouseButton {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl bincode::Encode for MouseButton {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            match self {
                Self::Left => {
                    bincode::Encode::encode(&0u32, encoder)?;
                }
                Self::Middle => {
                    bincode::Encode::encode(&1u32, encoder)?;
                }
                Self::Right => {
                    bincode::Encode::encode(&2u32, encoder)?;
                }
            }
            Ok(())
        }
    }
    impl bincode::Decode<()> for MouseButton {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            let discriminant: u32 = bincode::Decode::decode(decoder)?;
            match discriminant {
                0u32 => Ok(Self::Left),
                1u32 => Ok(Self::Middle),
                2u32 => Ok(Self::Right),
                _ => Err(bincode::error::DecodeError::UnexpectedVariant {
                    type_name: core::any::type_name::<Self>(),
                    allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2u32 },
                    found: discriminant,
                }),
            }
        }
    }
    impl Serialize for MouseButton {
        fn serialize(&self) -> Vec<u8> {
            use BufMutExt;
            use bytes::BufMut;
            let mut buffer = ::alloc::vec::Vec::new();
            buffer.write_string(std::any::type_name::<Self>());
            match self {
                Self::Left {} => {
                    buffer.write_string("Left");
                }
                Self::Middle {} => {
                    buffer.write_string("Middle");
                }
                Self::Right {} => {
                    buffer.write_string("Right");
                }
            }
            buffer
        }
    }
    impl Deserialize for MouseButton {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            use BufExt;
            use bytes::Buf;
            buf.read_name(std::any::type_name::<Self>())?;

        Self::deserialize_without_name(buf)
    }
    fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {    let variant_name = buf.read_string();
            match variant_name.as_ref() {
                "Left" => Ok(Self::Left),
                "Middle" => Ok(Self::Middle),
                "Right" => Ok(Self::Right),
                _ => Err(DeserializeError::InvalidEnumVariant {
                    expected: std::any::type_name::<Self>().to_string(),
                    actual: variant_name,
                }),
            }
        }
    }
}
mod xy_in {
    mod draw_command {
        use crate::*;
        use namui_type::*;
        impl XyIn for DrawCommand {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                match self {
                    DrawCommand::Path { command } => command.xy_in(xy),
                    DrawCommand::Text { command } => command.xy_in(xy),
                    DrawCommand::Image { command } => command.xy_in(xy),
                }
            }
        }
        impl XyIn for PathDrawCommand {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                NativePath::get(&self.path).contains(Some(&self.paint), xy)
            }
        }
        impl XyIn for TextDrawCommand {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                self.bounding_box().is_some_and(|x| x.is_xy_inside(xy))
            }
        }
        impl XyIn for ImageDrawCommand {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                let path = Path::new().add_rect(self.rect);
                NativePath::get(&path).contains(self.paint.as_ref(), xy)
            }
        }
    }
    mod rendering_tree {
        use crate::*;
        use std::ops::ControlFlow;
        impl XyIn for RenderingTree {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                xy_in(self, xy, &[])
            }
        }
        impl XyIn for [&RenderingTree] {
            fn xy_in(&self, xy: Xy<Px>) -> bool {
                self.iter().any(|node| node.xy_in(xy))
            }
        }
        pub struct VisitUtils<'a> {
            pub rendering_tree: &'a RenderingTree,
            pub ancestors: &'a [&'a RenderingTree],
        }
        impl VisitUtils<'_> {
            pub fn to_local_xy(&self, xy: Xy<Px>) -> Xy<Px> {
                self.rendering_tree.to_local_xy(xy, self.ancestors)
            }
        }
        pub trait Visit {
            fn visit_rln<F>(&self, callback: &mut F, ancestors: &[&Self]) -> ControlFlow<()>
            where
                F: FnMut(&Self, VisitUtils) -> ControlFlow<()>;
            fn to_local_xy(&self, xy: Xy<Px>, ancestors: &[&Self]) -> Xy<Px>;
            #[allow(dead_code)]
            fn get_xy(&self, ancestors: &[&RenderingTree]) -> Xy<Px>;
        }
        impl Visit for RenderingTree {
            fn visit_rln<F>(&self, callback: &mut F, ancestors: &[&Self]) -> ControlFlow<()>
            where
                F: FnMut(&Self, VisitUtils) -> ControlFlow<()>,
            {
                let mut next_ancestors = Vec::from(ancestors);
                next_ancestors.push(self);
                match self {
                    RenderingTree::Children(children) => {
                        for child in children.iter().rev() {
                            if let ControlFlow::Break(_) =
                                child.visit_rln(callback, &next_ancestors)
                            {
                                return ControlFlow::Break(());
                            }
                        }
                    }
                    RenderingTree::Special(special) => {
                        if let ControlFlow::Break(_) = special
                            .inner_rendering_tree_ref()
                            .visit_rln(callback, &next_ancestors)
                        {
                            return ControlFlow::Break(());
                        }
                    }
                    RenderingTree::Empty | RenderingTree::Node(_) => {}
                }
                let utils = VisitUtils {
                    ancestors,
                    rendering_tree: self,
                };
                callback(self, utils)
            }
            fn to_local_xy(&self, xy: Xy<Px>, ancestors: &[&Self]) -> Xy<Px> {
                let mut result_xy = xy;
                for ancestor in ancestors.iter() {
                    if let RenderingTree::Special(special) = ancestor {
                        match special {
                            SpecialRenderingNode::Translate(translate) => {
                                result_xy.x -= translate.x;
                                result_xy.y -= translate.y;
                            }
                            SpecialRenderingNode::Absolute(absolute) => {
                                result_xy = xy;
                                result_xy.x -= absolute.x;
                                result_xy.y -= absolute.y;
                            }
                            SpecialRenderingNode::Rotate(rotate) => {
                                result_xy =
                                    rotate.get_counter_wise_matrix().transform_xy(result_xy);
                            }
                            SpecialRenderingNode::Scale(scale) => {
                                result_xy.x /= *scale.x;
                                result_xy.y /= *scale.y;
                            }
                            SpecialRenderingNode::Transform(transform) => {
                                result_xy =
                                    transform.matrix.inverse().unwrap().transform_xy(result_xy);
                            }
                            SpecialRenderingNode::Clip(_)
                            | SpecialRenderingNode::OnTop(_)
                            | SpecialRenderingNode::MouseCursor(_) => {}
                        }
                    }
                }
                result_xy
            }
            #[allow(dead_code)]
            fn get_xy(&self, ancestors: &[&RenderingTree]) -> Xy<Px> {
                let mut xy = Xy {
                    x: px(0.0),
                    y: px(0.0),
                };
                for ancestor in ancestors.iter().rev() {
                    if let RenderingTree::Special(special) = ancestor {
                        match special {
                            SpecialRenderingNode::Translate(translate) => {
                                xy.x += translate.x;
                                xy.y += translate.y;
                            }
                            SpecialRenderingNode::Absolute(absolute) => {
                                xy.x += absolute.x;
                                xy.y += absolute.y;
                                break;
                            }
                            SpecialRenderingNode::Rotate(rotate) => {
                                let matrix = rotate.get_matrix();
                                xy = matrix.transform_xy(xy);
                            }
                            SpecialRenderingNode::Scale(scale) => {
                                xy.x *= *scale.x;
                                xy.y *= *scale.y;
                            }
                            SpecialRenderingNode::Transform(transform) => {
                                xy = transform.matrix.transform_xy(xy);
                            }
                            SpecialRenderingNode::Clip(_)
                            | SpecialRenderingNode::OnTop(_)
                            | SpecialRenderingNode::MouseCursor(_) => {}
                        }
                    }
                }
                xy
            }
        }
        fn xy_in(rendering_tree: &RenderingTree, xy: Xy<Px>, ancestors: &[&RenderingTree]) -> bool {
            let mut result = false;
            let _ = rendering_tree.visit_rln(
                &mut |node, utils| {
                    if let RenderingTree::Node(node) = node {
                        let local_xy = utils.to_local_xy(xy);
                        if node.xy_in(local_xy) && is_xy_clip_in_by_ancestors(xy, utils.ancestors) {
                            result = true;
                            ControlFlow::Break(())
                        } else {
                            ControlFlow::Continue(())
                        }
                    } else {
                        ControlFlow::Continue(())
                    }
                },
                ancestors,
            );
            result
        }
        fn is_xy_clip_in_by_ancestors(xy: Xy<Px>, ancestors: &[&RenderingTree]) -> bool {
            let mut ancestors = ancestors.to_vec();
            while let Some(closest_ancestor) = ancestors.pop() {
                if let RenderingTree::Special(special) = closest_ancestor {
                    if let SpecialRenderingNode::Clip(clip) = special {
                        let utils = VisitUtils {
                            ancestors: &ancestors,
                            rendering_tree: closest_ancestor,
                        };
                        let local_xy = utils.to_local_xy(xy);
                        if !clip.clip_in(local_xy) {
                            return false;
                        }
                    } else if let SpecialRenderingNode::OnTop(_) = special {
                        return true;
                    }
                }
            }
            true
        }
        trait ClipIn {
            fn clip_in(&self, xy: Xy<Px>) -> bool;
        }
        impl ClipIn for ClipNode {
            fn clip_in(&self, xy: Xy<Px>) -> bool {
                let xy_in = self.path.xy_in(xy);
                match self.clip_op {
                    ClipOp::Intersect => xy_in,
                    ClipOp::Difference => !xy_in,
                }
            }
        }
    }
    use crate::*;
    use namui_type::*;
    pub use rendering_tree::*;
    pub trait XyIn {
        fn xy_in(&self, xy: Xy<Px>) -> bool;
    }
    impl XyIn for Path {
        fn xy_in(&self, xy: Xy<Px>) -> bool {
            NativePath::get(self).contains(None, xy)
        }
    }
    impl<T> XyIn for &T
    where
        T: XyIn,
    {
        fn xy_in(&self, xy: Xy<Px>) -> bool {
            T::xy_in(*self, xy)
        }
    }
}
pub use bounding_box::*;
pub use command::*;
pub use event::*;
use namui_type::*;
pub use paragraph::*;
pub use rendering_tree::*;
pub use skia_types::*;
pub use types::*;
pub use xy_in::*;
