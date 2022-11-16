mod row;

use super::*;
use crate::storage::get_project_image_url;
use namui_prebuilt::typography::center_text;
use row::*;
use std::collections::BTreeSet;

const ROW_HEIGHT: Px = px(280.0);
const COLUMN_WIDTH: Px = px(200.0);
const FONT_SIZE: IntPx = int_px(18);

impl ImageTable {
    pub fn render(&self, props: Props) -> RenderingTree {
        let label_keys = self.label_keys();
        let rows = self.sorted_rows();

        table::vertical([
            table::fixed(36.px(), |wh| self.render_header_row(wh, &label_keys)),
            table::ratio(1, |wh| {
                self.list_view.render(list_view::Props {
                    xy: Xy::zero(),
                    height: wh.height,
                    scroll_bar_width: 10.px(),
                    item_wh: Wh::new(wh.width, ROW_HEIGHT),
                    items: rows,
                    item_render: |_wh, row| row.render(&self),
                })
            }),
        ])(props.wh)
    }

    fn render_header_row(&self, wh: Wh<Px>, label_keys: &BTreeSet<String>) -> RenderingTree {
        let cells = ["Image"]
            .into_iter()
            .chain(label_keys.iter().map(|key| key.as_str()));

        table::horizontal(cells.enumerate().map(|(index, string)| {
            table::fixed(COLUMN_WIDTH, move |wh| {
                let title_with_sort_mark = {
                    if index > 0 {
                        match self.sort_order_by.as_ref() {
                            Some(sort_order_by) => match sort_order_by {
                                SortOrderBy::Ascending { key }
                                | SortOrderBy::Descending { key }
                                    if key.ne(string) =>
                                {
                                    string.to_string()
                                }
                                SortOrderBy::Ascending { key } => format!("{} ▲", key),
                                SortOrderBy::Descending { key } => format!("{} ▼", key),
                            },
                            None => string.to_string(),
                        }
                    } else {
                        string.to_string()
                    }
                };

                let cell = render([
                    border(wh),
                    center_text(wh, title_with_sort_mark, Color::WHITE, FONT_SIZE),
                ]);
                if index > 0 {
                    cell.attach_event(move |builder| {
                        let key = string.to_string();
                        builder.on_mouse_down_in(move |event| {
                            if event.button == Some(MouseButton::Left) {
                                namui::event::send(InternalEvent::LeftClickOnLabelHeader {
                                    key: key.clone(),
                                });
                            }
                        });
                    })
                } else {
                    cell
                }
            })
        }))(wh)
    }

    fn label_keys(&self) -> BTreeSet<String> {
        self.images
            .iter()
            .flat_map(|image| image.labels.iter().map(|label| label.key.clone()))
            .collect()
    }

    fn sorted_rows(&self) -> Vec<Row> {
        let label_keys = self.label_keys();
        let sort_index = self.sort_order_by.as_ref().map(|sort_order_by| {
            match sort_order_by {
                SortOrderBy::Ascending { key } => label_keys.iter().position(|k| k.eq(key)),
                SortOrderBy::Descending { key } => label_keys.iter().position(|k| k.eq(key)),
            }
            .unwrap_or_default()
        });
        let mut rows = self
            .images
            .iter()
            .map(|image| Row {
                image_id: image.id,
                labels: label_keys
                    .iter()
                    .map(|key| Label {
                        key: key.clone(),
                        value: image
                            .labels
                            .iter()
                            .find(|label| &label.key == key)
                            .map(|label| label.value.clone())
                            .unwrap_or_default(),
                    })
                    .collect(),
            })
            .collect::<Vec<_>>();
        if let Some(sort_index) = sort_index {
            rows.sort_by_key(|row| row.labels.get(sort_index).cloned());
            match self.sort_order_by.as_ref().unwrap() {
                SortOrderBy::Ascending { .. } => {}
                SortOrderBy::Descending { .. } => {
                    rows.reverse();
                }
            };
        }
        rows
    }
}

fn border(wh: Wh<Px>) -> RenderingTree {
    simple_rect(wh, Color::WHITE, 1.px(), Color::TRANSPARENT)
}
