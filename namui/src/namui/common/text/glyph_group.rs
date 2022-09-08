use crate::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct GlyphGroup {
    pub glyph_ids: Vec<u16>,
    pub end_index: usize,
    pub width: Px,
    pub font: Arc<Font>,
}
pub(crate) fn get_glyph_groups(
    text: &str,
    fonts: &Vec<Arc<Font>>,
    paint: &Arc<Paint>,
) -> Vec<GlyphGroup> {
    let mut groups: Vec<GlyphGroup> = vec![];
    let mut non_calculated_char_and_indexes: Vec<(char, usize)> = text
        .chars()
        .enumerate()
        .map(|(index, char)| (char, index))
        .collect();
    let mut fonts = fonts.iter().peekable();

    while !non_calculated_char_and_indexes.is_empty() && fonts.peek().is_some() {
        let font = fonts.next().unwrap();

        let text = non_calculated_char_and_indexes
            .iter()
            .map(|(char, _)| char)
            .collect::<String>();

        let glyph_ids = font.get_glyph_ids(&text);

        let mut available_glyph_id_and_indexes = vec![];
        for (index, glyph_id) in glyph_ids.iter().enumerate() {
            if *glyph_id != 0 {
                available_glyph_id_and_indexes.push((*glyph_id, index));
                non_calculated_char_and_indexes.retain(|(_, index2)| index != *index2);
            }
        }

        if available_glyph_id_and_indexes.is_empty() {
            continue;
        }

        let available_glyph_id_and_index_and_width: Vec<(u16, usize, Px)> = {
            let available_glyph_ids: Vec<_> = available_glyph_id_and_indexes
                .iter()
                .map(|(glyph_id, _)| *glyph_id)
                .collect();

            let widths = font.get_glyph_widths(available_glyph_ids.into(), Option::Some(paint));

            widths
                .into_iter()
                .zip(available_glyph_id_and_indexes.into_iter())
                .map(|(width, (glyph_id, index))| (glyph_id, index, width))
                .collect()
        };

        for (glyph_id, index, width) in available_glyph_id_and_index_and_width {
            if let Some(last_group) = groups.last_mut() {
                if last_group.end_index + 1 == index {
                    last_group.glyph_ids.push(glyph_id);
                    last_group.width += width;
                    last_group.end_index = index;
                    continue;
                }
            }
            groups.push(GlyphGroup {
                glyph_ids: vec![glyph_id],
                end_index: index,
                width,
                font: font.clone(),
            });
        }
    }
    groups.sort_by(|a, b| a.end_index.cmp(&b.end_index));
    groups
}
impl GlyphGroup {
    pub(crate) fn start_index(&self) -> usize {
        self.end_index + 1 - self.glyph_ids.len()
    }
}
