use super::*;

pub struct Props {
    pub wh: Wh<Px>,
}

impl ImageEditModal {
    pub fn render_label_list(&self, props: Props) -> namui::RenderingTree {
        let row_height = 30.px();
        let row_count = (props.wh.height / row_height).floor() as usize;
        let column_max_count = (self.label_list.len() as f32 / row_count as f32).ceil() as usize;

        table::vertical([
            table::fit(
                table::FitAlign::LeftTop,
                typography::title::left_top("Label List", Color::WHITE).padding(12.px()),
            ),
            table::ratio(
                1,
                table::vertical((0..row_count).into_iter().map(|row_index| {
                    let texts_in_row = self
                        .label_list
                        .iter()
                        .skip(row_index * column_max_count)
                        .take(column_max_count)
                        .map(|label| {
                            let text = typography::body::left(
                                row_height,
                                format!("{}:{}", label.key, label.value),
                                Color::WHITE,
                            );

                            let width = text.get_bounding_box().unwrap().width();

                            table::fixed(width + 10.px(), |_wh| translate(5.px(), 0.px(), text))
                        });
                    table::fixed(row_height, table::horizontal(texts_in_row))
                })),
            ),
        ])(props.wh)
    }
}
