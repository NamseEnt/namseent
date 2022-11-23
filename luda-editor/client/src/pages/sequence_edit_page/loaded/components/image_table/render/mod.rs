use super::*;
use crate::storage::get_project_image_url;
use namui_prebuilt::*;
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
                            .build(),
                        Column::Label { key } => {
                            let key = key.clone();

                            let text = match self.sort_order_by.as_ref() {
                                Some(sort_order_by) => match sort_order_by {
                                    SortOrderBy::Ascending { key: sort_key }
                                    | SortOrderBy::Descending { key: sort_key }
                                        if sort_key.ne(&key) =>
                                    {
                                        key.clone()
                                    }
                                    SortOrderBy::Ascending { key } => format!("{} ▲", key),
                                    SortOrderBy::Descending { key } => format!("{} ▼", key),
                                },
                                None => key.to_string(),
                            };

                            cell::text(text)
                                .font_size(18.int_px())
                                .borders(Side::All, Line::Single)
                                .build()
                                .on_mouse_down(move |event| {
                                    if event.button == Some(MouseButton::Left) {
                                        namui::event::send(InternalEvent::LeftClickOnLabelHeader {
                                            key: key.clone(),
                                        });
                                    }
                                })
                        }
                    },
                    Row::Image { image } => match column {
                        Column::Image => cell::image(ImageSource::Url(
                            get_project_image_url(project_id, image.id).unwrap(),
                        ))
                        .borders(Side::All, Line::Single)
                        .build(),
                        Column::Label { key } => cell::text(get_label_value(image, key))
                            .font_size(18.int_px())
                            .borders(Side::All, Line::Single)
                            .on_change({
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
                            .build(),
                    }
                    .on_mouse_down({
                        let image_id = image.id;
                        move |event| {
                            if event.button == Some(MouseButton::Right) {
                                namui::event::send(InternalEvent::RightClickOnImageRow {
                                    image_id,
                                    global_xy: event.global_xy,
                                });
                            }
                        }
                    }),
                },
            })
        };

        render([
            self.context_menu
                .as_ref()
                .map(|context_menu| context_menu.render())
                .into(),
            sheet,
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

        let image_rows = images
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
