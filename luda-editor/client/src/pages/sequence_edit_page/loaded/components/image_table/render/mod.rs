// mod header_row;
// mod row;

use super::*;
use crate::storage::get_project_image_url;
use namui_prebuilt::*;
// use row::*;
use std::collections::BTreeSet;

impl ImageTable {
    pub fn render(&self, props: Props) -> RenderingTree {
        let project_id = self.project_id;
        let label_keys = self.label_keys();
        let rows = self.sorted_rows();
        let columns = [Column::Image]
            .into_iter()
            .chain(label_keys.into_iter().map(|key| Column::Label { key }));

        let sheet = {
            use sheet::{cell, cell::*};
            self.sheet.render(sheet::Props {
                wh: props.wh,
                rows,
                columns,
                row_height: |row| match row {
                    Row::Header => 36.px(),
                    Row::Image { .. } => 280.px(),
                },
                column_width: |_| 200.px(),
                cell: |row, column| match row {
                    Row::Header => match column {
                        Column::Image => cell::text("Image")
                            .font_size(18.int_px())
                            .borders(Side::All, Line::Single)
                            .into(),
                        Column::Label { key } => cell::text(key)
                            .font_size(18.int_px())
                            .borders(Side::All, Line::Single)
                            .into(),
                    },
                    Row::Image { image } => match column {
                        Column::Image => cell::image(ImageSource::Url(
                            get_project_image_url(project_id, image.id).unwrap(),
                        ))
                        .borders(Side::All, Line::Single)
                        .into(),
                        Column::Label { key } => cell::text(get_label_value(image, key))
                            .font_size(18.int_px())
                            .borders(Side::All, Line::Single)
                            .edit_with_text_input({
                                let image_id = image.id;
                                let label_key = key.clone();
                                move |text| {
                                    namui::event::send(InternalEvent::EditLabel {
                                        image_id,
                                        key: label_key.clone(),
                                        value: text.to_string(),
                                    });
                                }
                            })
                            .into(),
                    },
                },
            })
        };

        render([
            self.context_menu
                .as_ref()
                .map(|context_menu| context_menu.render())
                .into(),
            sheet,
            // table::vertical([
            //     table::fixed(36.px(), |wh| self.render_header_row(wh, &label_keys)),
            //     table::ratio(1, |wh| {
            //         self.list_view.render(list_view::Props {
            //             xy: Xy::zero(),
            //             height: wh.height,
            //             scroll_bar_width: 10.px(),
            //             item_wh: Wh::new(wh.width, ROW_HEIGHT),
            //             items: rows.into_iter().enumerate(),
            //             item_render: |_wh, (row_index, row)| row.render(&self, row_index),
            //         })
            //     }),
            // ])(props.wh),
        ])
        .attach_event(|builder| {
            builder.on_key_down(|event| {
                if event.code == Code::Escape {
                    namui::event::send(InternalEvent::EscKeyDown);
                }
            });
        })
    }

    fn label_keys(&self) -> BTreeSet<String> {
        self.images
            .iter()
            .flat_map(|image| image.labels.iter().map(|label| label.key.clone()))
            .collect()
    }

    fn sorted_rows(&self) -> Vec<Row> {
        let sort_key = self
            .sort_order_by
            .as_ref()
            .map(|sort_order_by| match sort_order_by {
                SortOrderBy::Ascending { key } | SortOrderBy::Descending { key } => key,
            });
        let mut images = self.images.clone();

        if let Some(sort_key) = sort_key {
            images.sort_by_key(|image| get_label_value(image, sort_key));
            match self.sort_order_by.as_ref().unwrap() {
                SortOrderBy::Ascending { .. } => {}
                SortOrderBy::Descending { .. } => {
                    images.reverse();
                }
            };
        }

        let image_rows = self
            .images
            .iter()
            .map(|image| Row::Image {
                image: image.clone(),
            })
            .collect::<Vec<_>>();
        [Row::Header]
            .into_iter()
            .chain(image_rows.into_iter())
            .collect()
    }
}

fn get_label_value(image: &ImageWithLabels, key: impl AsRef<str>) -> String {
    let key = key.as_ref();
    image
        .labels
        .iter()
        .find(|label| label.key == key)
        .map(|label| label.value.clone())
        .unwrap_or_default()
}
