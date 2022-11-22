use super::*;

impl<Row, Column> Sheet<Row, Column> {
    pub fn render<Rows, Columns, RowHeight, ColumnWidth, TCell>(
        &self,
        props: Props<Row, Column, Rows, Columns, RowHeight, ColumnWidth, TCell>,
    ) -> RenderingTree
    where
        Rows: IntoIterator<Item = Row>,
        Columns: IntoIterator<Item = Column>,
        RowHeight: Fn(&Row) -> Px,
        ColumnWidth: Fn(&Column) -> Px,
        TCell: Fn(&Row, &Column) -> Box<dyn Cell>,
    {
        let columns = props.columns.into_iter().collect::<Vec<_>>();
        self.vh_list_view.render(vh_list_view::Props {
            xy: Xy::zero(),
            wh: props.wh,
            scroll_bar_width: 10.px(),
            items: props.rows.into_iter().enumerate(),
            item_height: |(_row_index, row)| (props.row_height)(row),
            item_render: |wh, (row_index, row)| {
                let mut right = 0.px();
                render(columns.iter().enumerate().map(|(column_index, column)| {
                    let left = right;
                    let width = (props.column_width)(column);
                    right = left + width;

                    let cell_index = CellIndex {
                        row: row_index,
                        column: column_index,
                    };
                    let cell_wh = Wh::new(width, wh.height);
                    let cell = (props.cell)(&row, column).render(cell::Props {
                        wh: cell_wh,
                        is_editing: self.editing_cell == Some(cell_index),
                        is_selected: self.selections.contains(&cell_index),
                        text_input: &self.text_input,
                    });
                    translate(
                        left,
                        0.px(),
                        render([
                            simple_rect(cell_wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
                                .attach_event(|builder| {
                                    builder.on_mouse_down_in(move |event| {
                                        if event.button == Some(MouseButton::Left) {
                                            namui::event::send(InternalEvent::CellMouseLeftDown {
                                                cell_index,
                                            })
                                        }
                                    });
                                }),
                            clip(
                                PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), cell_wh)),
                                ClipOp::Intersect,
                                cell,
                            ),
                        ]),
                    )
                }))
            },
        })
    }
}
