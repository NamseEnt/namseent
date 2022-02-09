use super::*;

pub const SPREADSHEET_ID: &str = "1TSSmaIuBjTLVSYcCL0olqWeu9MTaYP4LEq9M1xzn1OU";
pub const API_KEY: &str = "AIzaSyBhMI9rz9l_f5NFsZlSh48K6Ee3Cbf4Oxw";

#[derive(Deserialize, Debug)]
struct SpreadsheetValuesGet {
    values: Box<[Box<[String]>]>,
}
impl SpreadsheetValuesGet {
    pub(crate) fn into_subtitles(&self) -> Vec<Subtitle> {
        self.values
            .iter()
            .skip(1)
            .filter_map(|row| {
                if row.len() < 7 {
                    return None;
                }

                let id = row[0].clone();
                // TODO : Make error if id is empty
                let korean_text = row[6].clone();
                Some(Subtitle {
                    id,
                    language_text_map: vec![(Language::Ko, korean_text)].into_iter().collect(),
                })
            })
            .collect()
    }
}

pub async fn get_subtitles_by_title(sheet_title: &str) -> Result<Vec<Subtitle>, String> {
    let range = format!("{}!A1:Z", sheet_title);
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?key={}",
        SPREADSHEET_ID, range, API_KEY
    );

    let result = namui::fetch_get_json::<SpreadsheetValuesGet>(&url).await;

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(error.to_string());
    }

    let spreadsheet_values_get = result.unwrap();

    Ok(spreadsheet_values_get.into_subtitles())
}

#[derive(Deserialize)]
struct SpreadsheetGet {
    sheets: Vec<SpreadsheetGetSheet>,
}

#[derive(Deserialize)]
struct SpreadsheetGetSheet {
    properties: SpreadsheetGetSheetProperties,
}

#[derive(Deserialize)]
struct SpreadsheetGetSheetProperties {
    title: String,
}

pub struct Sheet {
    pub title: String,
    pub subtitles: Vec<Subtitle>,
}

pub async fn get_sheets() -> Result<Vec<Sheet>, String> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}?key={}",
        SPREADSHEET_ID, API_KEY
    );

    let result = namui::fetch_get_json::<SpreadsheetGet>(&url).await;

    if result.is_err() {
        let error = result.err().unwrap();
        return Err(error.to_string());
    }

    let spreadsheet_get = result.unwrap();

    let sheets = spreadsheet_get.into_sheets().await;
    Ok(sheets)
}

impl SpreadsheetGet {
    async fn into_sheets(&self) -> Vec<Sheet> {
        let mut sheets = vec![];

        for google_spreadsheet_sheet in &self.sheets {
            let title = &google_spreadsheet_sheet.properties.title;
            let mut delay_sec = 1;

            let sheet = loop {
                let result = get_subtitles_by_title(&title).await;
                match result {
                    Ok(subtitles) => {
                        break Sheet {
                            title: title.clone(),
                            subtitles,
                        };
                    }
                    Err(error) => {
                        namui::log(format!("error on get_subtitles_by_title: {:?}", error));
                        tokio::time::sleep(std::time::Duration::from_secs(delay_sec)).await;
                        delay_sec *= 2;
                    }
                }
            };
            sheets.push(sheet);
        }

        sheets
    }
}
