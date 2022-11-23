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
    text_input: text_input::TextInput,
    editing_target: Option<EditingTarget>,
    pub saving_count: usize,
    context_menu: Option<context_menu::ContextMenu>,
    selection: Option<Selection>,
    cell_drag_context: Option<CellDragContext>,
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

struct EditingTarget {
    image_id: Uuid,
    label_key: String,
}

type Selection = Ltrb<usize>;

struct CellDragContext {
    start_row_index: usize,
    start_column_index: usize,
    last_row_index: usize,
    last_column_index: usize,
}

impl ImageTable {
    pub fn new(project_id: Uuid) -> ImageTable {
        request_reload_images(project_id);
        let table = ImageTable {
            project_id,
            list_view: list_view::ListView::new(),
            images: vec![],
            sort_order_by: None,
            text_input: text_input::TextInput::new(),
            editing_target: None,
            saving_count: 0,
            context_menu: None,
            selection: None,
            cell_drag_context: None,
            sheet: sheet::Sheet::new(sheet::ColorPalette {
                text_color: Color::WHITE,
                stroke_color: Color::WHITE,
                background_color: Color::BLACK,
                selected_text_color: Color::WHITE,
                selected_stroke_color: Color::from_u8(129, 198, 232, 255),
                selected_background_color: Color::BLACK, //Color::from_u8(37, 49, 109, 255)
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
