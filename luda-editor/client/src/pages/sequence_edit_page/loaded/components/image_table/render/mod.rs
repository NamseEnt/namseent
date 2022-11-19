mod header_row;
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

        render([
            self.context_menu
                .as_ref()
                .map(|context_menu| context_menu.render())
                .into(),
            table::vertical([
                table::fixed(36.px(), |wh| self.render_header_row(wh, &label_keys)),
                table::ratio(1, |wh| {
                    self.list_view.render(list_view::Props {
                        xy: Xy::zero(),
                        height: wh.height,
                        scroll_bar_width: 10.px(),
                        item_wh: Wh::new(wh.width, ROW_HEIGHT),
                        items: rows.into_iter().enumerate(),
                        item_render: |_wh, (row_index, row)| row.render(&self, row_index),
                    })
                }),
            ])(props.wh),
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
