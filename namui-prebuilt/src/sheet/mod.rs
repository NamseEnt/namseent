pub mod cell;

use cell::*;
use namui::prelude::*;

struct Sheet<Row, Column> {
    rows: Vec<Row>,
    columns: Vec<Column>,
    row_heights: Box<dyn Fn(&Row) -> Px>,
    column_widths: Box<dyn Fn(&Column) -> Px>,
    cells: Box<dyn Fn(&Row, &Column) -> Box<dyn Cell>>,
    // 위에꺼 전부 다 render로 넣자.
    selection: Option<Ltrb<usize>>,
    clip_board: Option<Vec<RowColumn<Row, Column>>>,
    text_input: TextInput,
    editing_cell: Option<CellIndex>,
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

/*
    클립보드로 붙여넣기를 sheet가 지원하려면, 데이터 변경 요청이 데이터 소스에 영향을 미쳐야한다.
    그러기 위해선 붙여넣기할 때 on_edit과 같은 변경 콜백을 사용자에게 전달해주거나,
    아니면 처음부터 데이터를 다 sheet가 가지고 있고, 나중에 사용자가 sheet에서 데이터를 빼올 수 있도록 하는 것도 방법 중 하나일거다.
    성능은 두번째 방법이 더 좋을거다.
    공간은 첫번째 방법이 더 좋을거다.
    싱크를 위해선 첫번째 방법이 더 좋을거다. 왜냐하면 두번째 방법은 싱크를 할 타이밍을 어떻게 정할지 모르기 때문이다.
    싱크는 변경이 일어날 때 일어나야한다.

    한번에 여러 셀을 복사 붙여넣기 한다면,
    이벤트를 N개 보내는 것보다는 한번에 N개의 변경을 보내는 것이 더 좋을거다.
    안그러면 N번의 렌더링이 일어날거다. 성능이 떨어질거다.
    변경은 어떻게 정의할까?
    텍스트에 변경이 있을 수도 있고, 이미지에 변경이 있을 수 있다.
    근데 그 텍스트라는게 String으로서 눈에 보이는 거지만, 그것의 데이터 소스는 숫자일지도 모른다.
    그럴 때 우리는 변경을 String으로 줘야할까, 아니면 숫자로 줘야할까?
    나는 사용자 마음대로 할 수 있게 해야한다고 생각한다.
    Box any를 사용해보자.

    Box any를 사용하면 디버깅이 어려워질 것 같다.
    그래서 디버깅을 위해선 변경을 어떻게 정의해야할까?

    sheet에서 다룰 타입을 limit하는 것은 어떨까?
    - String
    - Number
    - Image
    이미지의 경우 변경사항이 참 애매하다. 나는 이미지의 url보다는, url을 만드는 요소를 받고 싶다.

    cell마다 source를 연결해놓은건 어떨까? 그러면 복사 붙여넣기의 변경사항을 source의 타입으로 줄 수 있을거다.
    text input의 경우 parse를 해야하는게 맞을 것 같다. 숫자만 입력한다는 보장이 없잖은가.


*/

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
        }),
        selection: None,
        clip_board: None,
        text_input: TextInput::new(),
        editing_cell: None,
    };
}
