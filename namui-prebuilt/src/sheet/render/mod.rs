use super::*;

impl Sheet {
    pub fn render<Row, Column, Rows, Columns, RowHeight, ColumnWidth, TCell>(
        &self,
        props: Props<Row, Column, Rows, Columns, RowHeight, ColumnWidth, TCell>,
    ) -> RenderingTree
    where
        Rows: IntoIterator<Item = Row>,
        Columns: IntoIterator<Item = Column>,
        RowHeight: Fn(&Row) -> Px,
        ColumnWidth: Fn(&Column) -> Px,
        TCell: Fn(&Row, &Column) -> Cell,
    {
        let columns = props.columns.into_iter().collect::<Vec<_>>();
        let rows = props.rows.into_iter().collect::<Vec<_>>();

        let next_clipboard_items = [self
            .selections
            .iter()
            .take(1)
            .map(|cell_index| {
                let row = &rows[cell_index.row];
                let column = &columns[cell_index.column];
                let cell = (props.cell)(&row, &column);
                cell.inner.copy()
            })
            .collect::<Vec<_>>()]
        .into_iter()
        .collect::<Vec<_>>();

        let on_paste = {
            let selection_left_top = self
                .selections
                .iter()
                .min_by_key(|selection| (selection.row, selection.column));
            selection_left_top.and_then(|selection_left_top| {
                let row = &rows[selection_left_top.row];
                let column = &columns[selection_left_top.column];
                let cell = (props.cell)(&row, &column);
                cell.inner.on_paste()
            })
        };

        render([
            self.vh_list_view.render(vh_list_view::Props {
                xy: Xy::zero(),
                wh: props.wh,
                scroll_bar_width: 10.px(),
                items: rows.iter().enumerate(),
                item_height: |(_row_index, row)| (props.row_height)(row) + 1.px(),
                item_render: |wh, (row_index, row)| {
                    let mut right = 0.px();
                    let top = 1.px();
                    render(columns.iter().enumerate().map(|(column_index, column)| {
                        let left = right + 1.px();
                        let width = (props.column_width)(column);
                        right = left + width;

                        let cell_index = CellIndex {
                            row: row_index,
                            column: column_index,
                        };
                        let is_selected = self.selections.contains(&cell_index);
                        let cell_wh = Wh::new(width, wh.height - 1.px());
                        let cell = (props.cell)(&row, column);
                        translate(
                            left,
                            top,
                            render([
                                simple_rect(
                                    cell_wh,
                                    Color::TRANSPARENT,
                                    0.px(),
                                    Color::TRANSPARENT,
                                )
                                .attach_event(|builder| {
                                    let on_mouse_down = cell.on_mouse_down.clone();
                                    builder.on_mouse_down_in(move |event| {
                                        if event.button == Some(MouseButton::Left) {
                                            namui::event::send(InternalEvent::CellMouseLeftDown {
                                                cell_index,
                                            })
                                        }
                                        if let Some(on_mouse_down) = on_mouse_down.as_ref() {
                                            on_mouse_down(event)
                                        }
                                    });
                                }),
                                clip(
                                    PathBuilder::new()
                                        .add_rect(Rect::from_xy_wh(Xy::zero(), cell_wh)),
                                    ClipOp::Intersect,
                                    cell.inner.render(cell::Props {
                                        wh: cell_wh,
                                        is_editing: self.editing_cell == Some(cell_index),
                                        is_selected,
                                        text_input: &self.text_input,
                                        color_palette: self.color_palette,
                                    }),
                                ),
                            ]),
                        )
                    }))
                },
            }),
            render([false, true].into_iter().map(|need_to_be_selected| {
                self.vh_list_view.render(vh_list_view::Props {
                    xy: Xy::zero(),
                    wh: props.wh,
                    scroll_bar_width: 10.px(),
                    items: rows.iter().enumerate(),
                    item_height: |(_row_index, row)| (props.row_height)(row) + 1.px(),
                    item_render: |row_with_border_extra_height_wh, (row_index, row)| {
                        let mut right = 0.px();
                        let top = 1.px();
                        render(columns.iter().enumerate().map(|(column_index, column)| {
                            let left = right + 1.px();
                            let column_width = (props.column_width)(column);
                            right = left + column_width;

                            let cell_index = CellIndex {
                                row: row_index,
                                column: column_index,
                            };
                            let is_selected = self.selections.contains(&cell_index);
                            if need_to_be_selected != is_selected {
                                return RenderingTree::Empty;
                            }
                            let cell_wh = Wh::new(
                                column_width,
                                row_with_border_extra_height_wh.height - 1.px(),
                            );
                            let cell = (props.cell)(&row, column);
                            let stroke_color = if is_selected {
                                self.color_palette.selected_stroke_color
                            } else {
                                self.color_palette.stroke_color
                            };

                            let mut rendering_trees = vec![];
                            let borders = cell.inner.borders();
                            match borders.left {
                                None => {}
                                Some(Line::Single) => {
                                    let stroke_width = if is_selected { 4.px() } else { 1.px() };
                                    rendering_trees.push(translate(
                                        left + stroke_width / 2 - 1.px(),
                                        top - 1.px(),
                                        path(
                                            PathBuilder::new()
                                                .line_to(0.px(), cell_wh.height + 2.px()),
                                            PaintBuilder::new()
                                                .set_color(stroke_color)
                                                .set_style(PaintStyle::Stroke)
                                                .set_stroke_width(stroke_width)
                                                .set_anti_alias(true),
                                        ),
                                    ))
                                }
                            }

                            match borders.right {
                                None => {}
                                Some(Line::Single) => {
                                    let stroke_width = if is_selected { 4.px() } else { 1.px() };
                                    rendering_trees.push(translate(
                                        right - stroke_width / 2 + 1.px(),
                                        top - 1.px(),
                                        path(
                                            PathBuilder::new()
                                                .line_to(0.px(), cell_wh.height + 2.px()),
                                            PaintBuilder::new()
                                                .set_color(stroke_color)
                                                .set_style(PaintStyle::Stroke)
                                                .set_stroke_width(stroke_width)
                                                .set_anti_alias(true),
                                        ),
                                    ))
                                }
                            }

                            match borders.top {
                                None => {}
                                Some(Line::Single) => {
                                    let stroke_width = if is_selected { 4.px() } else { 1.px() };
                                    rendering_trees.push(translate(
                                        left - 1.px(),
                                        top + stroke_width / 2 - 1.px(),
                                        path(
                                            PathBuilder::new()
                                                .line_to(cell_wh.width + 2.px(), 0.px()),
                                            PaintBuilder::new()
                                                .set_color(stroke_color)
                                                .set_style(PaintStyle::Stroke)
                                                .set_stroke_width(stroke_width)
                                                .set_anti_alias(true),
                                        ),
                                    ))
                                }
                            }

                            match borders.bottom {
                                None => {}
                                Some(Line::Single) => {
                                    let stroke_width = if is_selected { 4.px() } else { 1.px() };
                                    rendering_trees.push(translate(
                                        left - 1.px(),
                                        top + cell_wh.height - stroke_width / 2 + 1.px(),
                                        path(
                                            PathBuilder::new()
                                                .line_to(cell_wh.width + 2.px(), 0.px()),
                                            PaintBuilder::new()
                                                .set_color(stroke_color)
                                                .set_style(PaintStyle::Stroke)
                                                .set_stroke_width(stroke_width)
                                                .set_anti_alias(true),
                                        ),
                                    ))
                                }
                            }

                            render(rendering_trees)
                        }))
                    },
                })
            })),
        ])
        .attach_event(move |builder| {
            let next_clipboard_items = next_clipboard_items.clone();
            let clip_board = self.clip_board.clone();
            let on_paste = on_paste.clone();
            // TODO: Move using arrow key
            // TODO: Select using shift + arrow key
            builder.on_key_down(move |event| {
                if [Code::ControlLeft, Code::KeyC]
                    .iter()
                    .all(|code| event.pressing_codes.contains(code))
                {
                    namui::event::send(InternalEvent::CtrlCDown {
                        clipboard_items: next_clipboard_items.clone(),
                    });
                } else if let Some(clip_board) = clip_board.as_ref() {
                    if let Some(on_paste) = on_paste.as_ref() {
                        if [Code::ControlLeft, Code::KeyV]
                            .iter()
                            .all(|code| event.pressing_codes.contains(code))
                        {
                            on_paste(clip_board[0][0].clone())
                        }
                    }
                }
            });
        })
    }
}
