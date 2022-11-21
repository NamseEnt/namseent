pub mod cell;

use crate::*;
use cell::*;
use namui::prelude::*;

pub struct Sheet<Row, Column> {
    vh_list_view: vh_list_view::VHListView,
    selection: Option<Ltrb<usize>>,
    clip_board: Option<Vec<RowColumn<Row, Column>>>,
    text_input: TextInput,
    editing_cell: Option<CellIndex>,
}

pub struct Props<Row, Column, Rows, Columns, RowHeight, ColumnWidth, TCell>
where
    Rows: IntoIterator<Item = Row>,
    Columns: IntoIterator<Item = Column>,
    RowHeight: Fn(&Row) -> Px,
    ColumnWidth: Fn(&Column) -> Px,
    TCell: Fn(&Row, &Column) -> Box<dyn Cell>,
{
    pub wh: Wh<Px>,
    pub rows: Rows,
    pub columns: Columns,
    pub row_height: RowHeight,
    pub column_width: ColumnWidth,
    pub cell: TCell,
}

impl<Row, Column> Sheet<Row, Column> {
    pub fn new() -> Self {
        Self {
            vh_list_view: vh_list_view::VHListView::new(),
            selection: None,
            clip_board: None,
            text_input: TextInput::new(),
            editing_cell: None,
        }
    }
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
            items: props.rows,
            item_height: |row| (props.row_height)(row),
            item_render: |wh, row| {
                let mut right = 0.px();
                render(columns.iter().map(|column| {
                    let left = right;
                    let width = (props.column_width)(column);
                    right = left + width;

                    let cell = (props.cell)(&row, column);
                    translate(left, 0.px(), {
                        let cell_wh = Wh::new(width, wh.height);
                        clip(
                            PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::zero(), cell_wh)),
                            ClipOp::Intersect,
                            cell.render(cell_wh),
                        )
                    })
                }))
            },
        })
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.vh_list_view.update(event);
    }
}

struct CellIndex {
    pub row: usize,
    pub column: usize,
}

pub struct CopyPasteEvent<Row, Column> {
    copy_pastes: Vec<CopyPaste<Row, Column>>,
}

struct RowColumn<Row, Column> {
    row: Row,
    column: Column,
}
struct CopyPaste<Row, Column> {
    from: RowColumn<Row, Column>,
    to: RowColumn<Row, Column>,
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

    let sheet = Sheet::<RowType, ColumnType>::new();

    sheet.render(Props {
        wh: Wh::new(100.px(), 100.px()),
        rows: [RowType::Header]
            .into_iter()
            .chain(data.map(|data| RowType::Data(data))),
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
            ),
        row_height: |row| match row {
            RowType::Header => 36.px(),
            RowType::Data(_) => 108.px(),
        },
        column_width: |column| match column {
            ColumnType::Image => 108.px(),
            ColumnType::Label { .. } => 64.px(),
        },
        cell: |row, column| match row {
            RowType::Header => match column {
                ColumnType::Image => cell::text("image").into(),
                ColumnType::Label { key, .. } => cell::text(key).into(),
            },
            RowType::Data(data) => match column {
                ColumnType::Image => cell::image(ImageSource::Url(
                    Url::parse(&format!("https://example.com/{}.png", data.image_id)).unwrap(),
                ))
                .on_edit(|| namui::event::send("Open image selector"))
                .into(),
                ColumnType::Label { label_index, .. } => {
                    cell::text(data.label_values[*label_index])
                        .edit_with_text_input(|string| namui::event::send("Update label value"))
                        .into()
                }
            },
        },
    });
}
