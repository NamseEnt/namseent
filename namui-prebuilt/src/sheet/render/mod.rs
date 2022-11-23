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
                    let cell_wh = Wh::new(width, wh.height - 1.px());
                    let cell = (props.cell)(&row, column);
                    let cell_rendering_tree = translate(
                        left,
                        top,
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
                                cell.render(cell::Props {
                                    wh: cell_wh,
                                    is_editing: self.editing_cell == Some(cell_index),
                                    is_selected: self.selections.contains(&cell_index),
                                    text_input: &self.text_input,
                                }),
                            ),
                        ]),
                    );
                    let border_rendering_tree = {
                        let mut rendering_trees = vec![];
                        let borders = cell.borders();
                        match borders.left {
                            Line::None => {}
                            Line::Single => {
                                let line_width = 1.px();
                                rendering_trees.push(translate(
                                    left + line_width / 2 - 1.px(),
                                    top - 0.5.px(),
                                    path(
                                        PathBuilder::new().line_to(0.px(), wh.height + 1.px()),
                                        PaintBuilder::new()
                                            .set_color(self.color_palette.primary_color)
                                            .set_style(PaintStyle::Stroke)
                                            .set_stroke_width(line_width),
                                    ),
                                ))
                            }
                            Line::Double => todo!(),
                        }

                        match borders.right {
                            Line::None => {}
                            Line::Single => {
                                let stroke_width = 1.px();
                                rendering_trees.push(translate(
                                    right - stroke_width / 2 + 1.px(),
                                    top - 0.5.px(),
                                    path(
                                        PathBuilder::new().line_to(0.px(), wh.height + 1.px()),
                                        PaintBuilder::new()
                                            .set_color(self.color_palette.primary_color)
                                            .set_style(PaintStyle::Stroke)
                                            .set_stroke_width(stroke_width),
                                    ),
                                ))
                            }
                            Line::Double => todo!(),
                        }

                        match borders.top {
                            Line::None => {}
                            Line::Single => {
                                let stroke_width = 1.px();
                                rendering_trees.push(translate(
                                    -0.5.px(),
                                    top + stroke_width / 2 - 1.px(),
                                    path(
                                        PathBuilder::new().line_to(wh.width + 1.px(), 0.px()),
                                        PaintBuilder::new()
                                            .set_color(self.color_palette.primary_color)
                                            .set_style(PaintStyle::Stroke)
                                            .set_stroke_width(stroke_width),
                                    ),
                                ))
                            }
                            Line::Double => todo!(),
                        }

                        match borders.bottom {
                            Line::None => {}
                            Line::Single => {
                                let stroke_width = 1.px();
                                rendering_trees.push(translate(
                                    -0.5.px(),
                                    top + cell_wh.height - stroke_width / 2 + 1.px(),
                                    path(
                                        PathBuilder::new().line_to(wh.width + 1.px(), 0.px()),
                                        PaintBuilder::new()
                                            .set_color(self.color_palette.primary_color)
                                            .set_style(PaintStyle::Stroke)
                                            .set_stroke_width(stroke_width),
                                    ),
                                ))
                            }
                            Line::Double => todo!(),
                        }

                        render(rendering_trees)
                    };

                    render([cell_rendering_tree, border_rendering_tree])
                }))
            },
        })
    }
}
