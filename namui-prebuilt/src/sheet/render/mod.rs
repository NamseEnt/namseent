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
        let rows = props.rows.into_iter().collect::<Vec<_>>();

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
                                    builder.on_mouse_down_in(move |event| {
                                        if event.button == Some(MouseButton::Left) {
                                            namui::event::send(InternalEvent::CellMouseLeftDown {
                                                cell_index,
                                            })
                                        }
                                    });
                                }),
                                clip(
                                    PathBuilder::new()
                                        .add_rect(Rect::from_xy_wh(Xy::zero(), cell_wh)),
                                    ClipOp::Intersect,
                                    cell.render(cell::Props {
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
                            let borders = cell.borders();
                            match borders.left {
                                Line::None => {}
                                Line::Single => {
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
                                Line::Double => {}
                                Line::BoldSingle => {}
                            }

                            match borders.right {
                                Line::None => {}
                                Line::Single => {
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
                                Line::Double => {}
                                Line::BoldSingle => {}
                            }

                            match borders.top {
                                Line::None => {}
                                Line::Single => {
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
                                Line::Double => {}
                                Line::BoldSingle => {}
                            }

                            match borders.bottom {
                                Line::None => {}
                                Line::Single => {
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
                                Line::Double => {}
                                Line::BoldSingle => {}
                            }

                            render(rendering_trees)
                        }))
                    },
                })
            })),
        ])
    }
}
