mod render;
mod update;

use crate::components::*;
use namui::prelude::*;
use namui_prebuilt::*;
use rpc::data::*;

pub struct ImageTable {
    project_id: Uuid,
    list_view: list_view::ListView,
    images: Vec<ImageWithLabels>,
    sort_order_by: Option<SortOrderBy>,
    pub saving_count: usize,
    context_menu: Option<context_menu::ContextMenu>,
    sheet: sheet::Sheet,
}

enum Row {
    Header,
    Image { image: ImageWithLabels },
}

enum Column {
    Image,
    Label { key: String },
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Error(String),
}

enum InternalEvent {
    LoadImages(Vec<ImageWithLabels>),
    LeftClickOnLabelHeader {
        key: String,
    },
    PutImageMetaDataSuccess,
    RightClickOnImageRow {
        image_id: Uuid,
        global_xy: Xy<Px>,
    },
    EscKeyDown,
    EditLabel {
        image_id: Uuid,
        key: String,
        value: String,
    },
}

enum SortOrderBy {
    Ascending { key: String },
    Descending { key: String },
}

impl ImageTable {
    pub fn new(project_id: Uuid) -> ImageTable {
        request_reload_images(project_id);
        let table = ImageTable {
            project_id,
            list_view: list_view::ListView::new(),
            images: vec![],
            sort_order_by: None,
            saving_count: 0,
            context_menu: None,
            sheet: sheet::Sheet::new(sheet::ColorPalette {
                text_color: Color::WHITE,
                stroke_color: Color::WHITE,
                background_color: Color::BLACK,
                selected_text_color: Color::WHITE,
                selected_stroke_color: Color::from_u8(129, 198, 232, 255),
                selected_background_color: Color::BLACK,
            }),
        };

        table.request_reload_images();
        table
    }
    pub fn request_reload_images(&self) {
        request_reload_images(self.project_id);
    }
}

pub fn request_reload_images(project_id: Uuid) {
    crate::RPC
        .list_images(rpc::list_images::Request { project_id })
        .callback(|result| match result {
            Ok(response) => {
                namui::event::send(InternalEvent::LoadImages(response.images));
            }
            Err(error) => {
                namui::event::send(Event::Error(error.to_string()));
            }
        })
}
