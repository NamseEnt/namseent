use super::*;

impl ImageTable {
    pub fn render_header_row(&self, wh: Wh<Px>, label_keys: &BTreeSet<String>) -> RenderingTree {
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
}
