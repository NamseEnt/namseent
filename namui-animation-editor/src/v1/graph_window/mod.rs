use namui::{
    animation::{KeyframeGraph, KeyframePoint, KeyframeValue, Layer},
    prelude::*,
    types::{Degree, PixelSize, Time},
};
use namui_prebuilt::{
    table::{horizontal, vertical},
    typography::center_text,
    *,
};
use std::sync::{Arc, RwLock};

pub(crate) struct GraphWindow {
    id: String,
}

pub(crate) struct Props {}

enum Event {}

impl GraphWindow {
    pub(crate) fn new() -> Self {
        Self {
            id: namui::nanoid(),
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {}

    fn get_row_column_count(&self, wh: Wh<f32>, props: Props) -> (usize, usize) {
        (8, 8)
    }
}

impl table::CellRender<Props> for GraphWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        let (row_count, column_count) = self.get_row_column_count(wh, props);

        // 8 rows => 8 row + 7 inner border
        /*
           1 0 1 0 1 0 1
           0 0 0 0 0 0 0
           1 0 1 0 1 0 1
           0 0 0 0 0 0 0
           1 0 1 0 1 0 1
           0 0 0 0 0 0 0
           1 0 1 0 1 0 1
        */
        let border_width = 1.0;
        let mut rows = vec![];
        for row_index in 0..(row_count * 2 - 1) {
            enum CellType {
                Cell,
                Border,
            }
            let row_type = match row_index % 2 {
                0 => CellType::Cell,
                _ => CellType::Border,
            };
            let row = match row_type {
                CellType::Cell => {
                    ratio!(1.0, |wh| {
                        let mut columns = vec![];
                        for column_index in 0..(column_count * 2 - 1) {
                            let column_type = match column_index % 2 {
                                0 => CellType::Cell,
                                _ => CellType::Border,
                            };
                            let column = match column_type {
                                CellType::Cell => ratio!(1.0, |wh| {
                                    simple_rect(wh, Color::TRANSPARENT, 0.0, Color::BLACK)
                                }),
                                CellType::Border => fixed!(border_width, |wh| {
                                    simple_rect(wh, Color::TRANSPARENT, 0.0, Color::WHITE)
                                }),
                            };
                            columns.push(column);
                        }
                        horizontal(columns)(wh)
                    })
                }
                CellType::Border => {
                    fixed!(border_width, |wh| {
                        simple_rect(wh, Color::TRANSPARENT, 0.0, Color::WHITE)
                    })
                }
            };
            rows.push(row);
        }

        render([
            vertical(rows)(wh),
            simple_rect(wh, Color::WHITE, 1.0, Color::TRANSPARENT),
        ])
    }
}
