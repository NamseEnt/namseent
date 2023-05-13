use rpc::{
    data::{Cut, Sequence},
    uuid,
};

fn main() {
    let input = include_str!("input.json");
    let input: Input = serde_json::from_str(input).unwrap();

    let mut sequence = Sequence::new(uuid(), "0주차-0-카페".to_string());
    let character_names = [
        "오하연",
        "피디",
        "선임피디",
        "김혜진",
        "나지",
        "카페 직원",
        "MCN 담당자",
        "연습생들",
        "연습생",
        "연습생 1",
        "연습생 2",
        "프로듀서 1",
        "???",
        "댄스 트레이너",
        "피디들",
    ];

    for page in input.pages {
        let mut cut = Cut::new(uuid());

        if page.texts.len() == 0 {
            // nothing
        } else if page.texts.len() >= 2 {
            let mut it = page.texts.into_iter();
            let character = it.next().unwrap();
            let texts = it.collect::<Vec<_>>();
            assert!(character_names.contains(&character.content.as_ref()));

            cut.character_name = character.content;

            cut.line = texts
                .into_iter()
                .map(|text| text.content)
                .collect::<Vec<_>>()
                .join("\n");
        } else {
            cut.line = page
                .texts
                .into_iter()
                .map(|text| text.content)
                .collect::<Vec<_>>()
                .join("\n");
        }

        // cut.character_name

        sequence.cuts.push(cut);
    }

    // println!("{text_set:?}");
}

#[derive(serde::Deserialize, Debug)]
struct Input {
    pages: Vec<Page>,
}

#[derive(serde::Deserialize, Debug)]
struct Page {
    images: Vec<Image>,
    texts: Vec<Text>,
}

#[derive(serde::Deserialize, Debug)]
struct Image {
    url: String,
    xywh: Xywh,
}

#[derive(serde::Deserialize, Debug)]
struct Xywh {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(serde::Deserialize, Debug)]
struct Text {
    content: String,
    font: Font,
}

#[derive(serde::Deserialize, Debug)]
struct Font {
    size: usize,
    weight: usize,
    family: String,
    bold: bool,
    italic: bool,
    strikethrough: bool,
    underline: bool,
}
