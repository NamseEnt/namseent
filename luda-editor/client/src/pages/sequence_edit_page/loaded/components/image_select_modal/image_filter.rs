use super::*;

impl ImageSelectModal {
    pub const ROW_CELL_COUNT: usize = 4;
    pub fn get_filtered_images(&self) -> Vec<&ImageWithLabels> {
        self.images
            .iter()
            .filter(|image| {
                self.selected_labels
                    .iter()
                    .all(|label| image.labels.contains(label))
            })
            .collect::<Vec<_>>()
    }

    pub fn get_selected_image_row_column(&self) -> Option<(usize, usize)> {
        self.selected_image.as_ref().and_then(|selected_image| {
            for (index, image) in self.get_filtered_images().into_iter().enumerate() {
                if image.id == selected_image.id {
                    let row = index / Self::ROW_CELL_COUNT;
                    let column = index % Self::ROW_CELL_COUNT;
                    return Some((row, column));
                }
            }
            None
        })
    }

    pub fn get_row_count(&self) -> usize {
        self.get_filtered_images().len() / Self::ROW_CELL_COUNT
    }

    pub fn get_row_column_on_keyboard_event(
        &self,
        code: Code,
        row_index: usize,
        column_index: usize,
        row_count: usize,
        column_count: usize,
        total_count: usize,
    ) -> Option<(usize, usize)> {
        if total_count == 0 {
            return None;
        }
        enum Arrow {
            Left,
            Up,
            Right,
            Down,
        }
        let arrow = match code {
            Code::ArrowLeft => Some(Arrow::Left),
            Code::ArrowUp => Some(Arrow::Up),
            Code::ArrowRight => Some(Arrow::Right),
            Code::ArrowDown => Some(Arrow::Down),
            _ => None,
        };
        arrow.map(|arrow| match arrow {
            Arrow::Left => {
                if column_index == 0 {
                    if row_index == 0 {
                        (0, 0)
                    } else {
                        (row_index - 1, column_count - 1)
                    }
                } else {
                    (row_index, column_index - 1)
                }
            }
            Arrow::Up => {
                if row_index == 0 {
                    (0, 0)
                } else {
                    (row_index - 1, column_index)
                }
            }
            Arrow::Right => {
                if column_index + 1 == column_count {
                    (row_index + 1, 0)
                } else {
                    (row_index, column_index + 1)
                }
            }
            Arrow::Down => {
                if row_index == row_count - 1 {
                    (row_count - 1, (total_count - 1) % column_count)
                } else {
                    (row_index + 1, column_index)
                }
            }
        })
    }
}
