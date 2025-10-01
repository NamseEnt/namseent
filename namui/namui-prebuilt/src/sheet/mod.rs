// pub mod cell;
// mod render;
// mod update;

// use crate::*;
// use cell::*;
// use namui::*;
// use std::collections::HashSet;

// pub struct Sheet {
//     vh_list_view: vh_list_view::VHListView,
//     /// TODO: Support multiple selection
//     selections: HashSet<CellIndex>,
//     /// TODO: Support multiple copy/paste
//     /// TODO: Support OS clipboard compatibility using CSV format
//     clip_board: Option<Vec<Vec<ClipboardItem>>>,
//     text_input: TextInput,
//     editing_cell: Option<CellIndex>,
//     color_palette: ColorPalette,
// }

// pub struct Props<Row, Column, Rows, Columns, RowHeight, ColumnWidth, TCell>
// where
//     Rows: IntoIterator<Item = Row>,
//     Columns: IntoIterator<Item = Column>,
//     RowHeight: Fn(&Row) -> Px,
//     ColumnWidth: Fn(&Column) -> Px,
//     TCell: Fn(&Row, &Column) -> Cell,
// {
//     pub wh: Wh<Px>,
//     pub rows: Rows,
//     pub columns: Columns,
//     pub row_height: RowHeight,
//     pub column_width: ColumnWidth,
//     pub cell: TCell,
// }

// impl Sheet {
//     pub fn new(color_palette: ColorPalette) -> Self {
//         Self {
//             vh_list_view: vh_list_view::VHListView::new(),
//             selections: HashSet::new(),
//             clip_board: None,
//             text_input: TextInput::new(),
//             editing_cell: None,
//             color_palette,
//         }
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct ColorPalette {
//     pub text_color: Color,
//     pub stroke_color: Color,
//     pub background_color: Color,
//     pub selected_text_color: Color,
//     pub selected_stroke_color: Color,
//     pub selected_background_color: Color,
// }

// enum InternalEvent {
//     CellMouseLeftDown {
//         cell_index: CellIndex,
//     },
//     CtrlCDown {
//         clipboard_items: Vec<Vec<ClipboardItem>>,
//     },
// }

// #[derive(Debug, bincode::Decode, bincode::Encode, Clone, Copy, PartialEq, Eq, Hash)]
// struct CellIndex {
//     pub row: usize,
//     pub column: usize,
// }

// #[allow(dead_code)]
// fn usage_code() {
//     enum RowType {
//         Header,
//         Data(Data),
//     }
//     enum ColumnType {
//         Image,
//         Label { key: String, label_index: usize },
//     }
//     let label_keys = ["name", "description", "author"];
//     struct Data {
//         image_id: &'static str,
//         label_values: Vec<&'static str>,
//     }
//     let data = [
//         Data {
//             image_id: "image1",
//             label_values: vec!["image1", "image1 description", "author1"],
//         },
//         Data {
//             image_id: "image2",
//             label_values: vec!["image2", "image2 description", "author2"],
//         },
//         Data {
//             image_id: "image3",
//             label_values: vec!["image3", "image3 description", "author3"],
//         },
//     ];

//     let color_palette = ColorPalette {
//         text_color: Color::WHITE,
//         stroke_color: Color::WHITE,
//         background_color: Color::BLACK,
//         selected_text_color: Color::WHITE,
//         selected_stroke_color: Color::WHITE,
//         selected_background_color: Color::from_u8(37, 49, 109, 255),
//     };

//     let sheet = Sheet::new(color_palette);

//     sheet.render(Props {
//         wh: Wh::new(100.px(), 100.px()),
//         rows: [RowType::Header]
//             .into_iter()
//             .chain(data.map(|data| RowType::Data(data))),
//         columns: [ColumnType::Image]
//             .into_iter()
//             .chain(
//                 label_keys
//                     .iter()
//                     .enumerate()
//                     .map(|(index, key)| ColumnType::Label {
//                         key: key.to_string(),
//                         label_index: index,
//                     }),
//             ),
//         row_height: |row| match row {
//             RowType::Header => 36.px(),
//             RowType::Data(_) => 108.px(),
//         },
//         column_width: |column| match column {
//             ColumnType::Image => 108.px(),
//             ColumnType::Label { .. } => 64.px(),
//         },
//         cell: |row, column| match row {
//             RowType::Header => match column {
//                 ColumnType::Image => cell::text("image").borders(Side::All, Line::Single).build(),
//                 ColumnType::Label { key, .. } => cell::text(key).build(),
//             },
//             RowType::Data(data) => match column {
//                 ColumnType::Image => cell::image(ImageSource::Url(
//                     Url::parse(&format!("https://example.com/{}.png", data.image_id)).unwrap(),
//                 ))
//                 .build(),
//                 ColumnType::Label { label_index, .. } => {
//                     cell::text(data.label_values[*label_index])
//                         .on_change(|_: &String| namui::event::send("Update label value"))
//                         .build()
//                 }
//             },
//         },
//     });
// }
