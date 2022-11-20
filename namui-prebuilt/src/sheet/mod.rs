use namui::prelude::*;

struct Sheet<Row, Column> {
    rows: Vec<Row>,
    columns: Vec<Column>,
    row_heights: Box<dyn Fn(&Row) -> Px>,
    column_widths: Box<dyn Fn(&Column) -> Px>,
    cells: Box<dyn Fn(&Row, &Column) -> Cell>,
}
struct Cell {
    content: CellContent,
    on_edit: Option<Box<dyn Fn()>>,
}
enum CellContent {
    Empty,
    Text { text: String },
    Image { image_source: ImageSource },
}

mod cell {
    use super::*;
    pub(super) fn empty() -> Cell {
        Cell {
            content: CellContent::Empty,
            on_edit: None,
        }
    }
    pub(super) fn text(text: impl AsRef<str>, edit_with_text_input: bool) -> Cell {
        Cell {
            content: CellContent::Text {
                text: text.as_ref().to_string(),
            },
            on_edit: if edit_with_text_input {
                Some(Box::new(|| todo!()))
            } else {
                None
            },
        }
    }
    pub(super) fn image(image_source: ImageSource) -> Cell {
        Cell {
            content: CellContent::Image { image_source },
            on_edit: None,
        }
    }
}

impl Cell {
    fn on_edit(self, callback: impl Fn() + 'static) -> Self {
        Self {
            content: self.content,
            on_edit: Some(Box::new(callback)),
        }
    }
}

fn usage() {
    enum RowType {
        Header,
        Data(Data),
    }
    enum ColumnType {
        Image,
        Label { key: String, label_index: usize },
    }
    let label_keys = ["name", "description", "author"];
    struct Data {
        image_id: &'static str,
        label_values: Vec<&'static str>,
    }
    let data = [
        Data {
            image_id: "image1",
            label_values: vec!["image1", "image1 description", "author1"],
        },
        Data {
            image_id: "image2",
            label_values: vec!["image2", "image2 description", "author2"],
        },
        Data {
            image_id: "image3",
            label_values: vec!["image3", "image3 description", "author3"],
        },
    ];

    let sheet = Sheet {
        rows: [RowType::Header]
            .into_iter()
            .chain(data.map(|data| RowType::Data(data)))
            .collect(),
        columns: [ColumnType::Image]
            .into_iter()
            .chain(
                label_keys
                    .iter()
                    .enumerate()
                    .map(|(index, key)| ColumnType::Label {
                        key: key.to_string(),
                        label_index: index,
                    }),
            )
            .collect(),
        row_heights: Box::new(|row| match row {
            RowType::Header => 36.px(),
            RowType::Data(_) => 108.px(),
        }),
        column_widths: Box::new(|column| match column {
            ColumnType::Image => 108.px(),
            ColumnType::Label { .. } => 64.px(),
        }),
        cells: Box::new(|row, column| match row {
            RowType::Header => match column {
                ColumnType::Image => cell::text("image", false),
                ColumnType::Label { key, .. } => cell::text(key, false),
            },
            RowType::Data(data) => match column {
                ColumnType::Image => cell::image(ImageSource::Url(
                    Url::parse(&format!("https://example.com/{}.png", data.image_id)).unwrap(),
                ))
                .on_edit(|| namui::event::send("Open image selector")),
                ColumnType::Label { label_index, .. } => {
                    cell::text(data.label_values[*label_index], true)
                }
            },
        }),
    };
}
