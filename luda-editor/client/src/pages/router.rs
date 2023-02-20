use super::*;
use namui::prelude::*;
use std::sync::Once;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

static HASH_CHANGE_EVENT_LISTENER: Once = Once::new();

pub struct Router {
    route: Route,
}

pub struct Props {
    pub wh: Wh<Px>,
}

enum InternalEvent {
    PathChanged(String),
}

pub enum Route {
    ProjectListPage(project_list_page::ProjectListPage),
    SequenceListPage(sequence_list_page::SequenceListPage),
    SequenceEditPage(sequence_edit_page::SequenceEditPage),
}
impl Route {
    pub fn from_path(path: &String) -> Self {
        if path.starts_with("/sequence_list") {
            let rest = path.clone().split_off("/sequence_list".len());
            if let Ok(project_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::SequenceListPage(sequence_list_page::SequenceListPage::new(
                    project_id,
                ));
            }
        }

        if path.starts_with("/sequence_edit") {
            let rest = path.clone().split_off("/sequence_edit".len());
            if let Ok(sequence_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::SequenceEditPage(sequence_edit_page::SequenceEditPage::new(
                    sequence_id,
                ));
            }
        }

        Self::ProjectListPage(project_list_page::ProjectListPage::new())
    }
}

impl Router {
    pub fn new() -> Self {
        Self::register_hash_change_event_listener();
        let path = get_path_from_hash();
        let route = Route::from_path(&path);
        Self { route }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::PathChanged(path) => self.route = Route::from_path(path),
        });
        match &mut self.route {
            Route::ProjectListPage(project_list_page) => project_list_page.update(event),
            Route::SequenceListPage(sequence_list_page) => sequence_list_page.update(event),
            Route::SequenceEditPage(sequence_edit_page) => sequence_edit_page.update(event),
        }
    }
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        match &self.route {
            Route::ProjectListPage(project_list_page) => {
                project_list_page.render(project_list_page::Props { wh: props.wh })
            }
            Route::SequenceListPage(sequence_list_page) => {
                sequence_list_page.render(sequence_list_page::Props { wh: props.wh })
            }
            Route::SequenceEditPage(sequence_edit_page) => {
                sequence_edit_page.render(sequence_edit_page::Props { wh: props.wh })
            }
        }
    }

    fn register_hash_change_event_listener() {
        HASH_CHANGE_EVENT_LISTENER.call_once(|| {
            let window = web_sys::window().unwrap();
            let listener = Closure::<dyn FnMut(_)>::new(move |_: HashChangeEvent| {
                namui::event::send(InternalEvent::PathChanged(get_path_from_hash()));
            });

            window
                .add_event_listener_with_callback("hashchange", listener.as_ref().unchecked_ref())
                .unwrap();

            listener.forget();
        })
    }

    pub fn move_to(path: String) {
        let window = web_sys::window().unwrap();
        window.location().set_hash(&path).unwrap();
    }
}

fn get_path_from_hash() -> String {
    let window = web_sys::window().unwrap();
    let hash = window.location().hash().unwrap_or("#".to_string());
    let path = hash.trim_start_matches('#');
    match path.starts_with("/") {
        true => path.to_string(),
        false => "/".to_string(),
    }
}
