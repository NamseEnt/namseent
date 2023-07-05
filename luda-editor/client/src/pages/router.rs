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
    PathChanged(RoutePath),
}

pub enum Route {
    ProjectListPage(project_list_page::ProjectListPage),
    SequenceListPage(sequence_list_page::SequenceListPage),
    SequenceEditPage(sequence_edit_page::SequenceEditPage),
}
impl From<RoutePath> for Route {
    fn from(path: RoutePath) -> Self {
        match path {
            RoutePath::ProjectList => {
                Self::ProjectListPage(project_list_page::ProjectListPage::new())
            }
            RoutePath::SequenceList { project_id } => {
                Self::SequenceListPage(sequence_list_page::SequenceListPage::new(project_id))
            }
            RoutePath::SequenceEdit {
                project_id,
                sequence_id,
            } => {
                return Self::SequenceEditPage(sequence_edit_page::SequenceEditPage::new(
                    project_id,
                    sequence_id,
                ))
            }
        }
    }
}

impl Router {
    pub fn new() -> Self {
        Self::register_hash_change_event_listener();
        let path = get_path_from_hash();
        let route = Route::from(path);
        Self { route }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<InternalEvent>(|event| match event {
            InternalEvent::PathChanged(path) => self.route = Route::from(path.clone()),
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

    pub fn move_to(path: RoutePath) {
        let window = web_sys::window().unwrap();
        window.location().set_hash(&path.to_string()).unwrap();
    }
}

fn get_path_from_hash() -> RoutePath {
    let window = web_sys::window().unwrap();
    let hash = window.location().hash().unwrap_or("#".to_string());
    let path = hash.trim_start_matches('#');
    RoutePath::from(path.to_string())
}

#[derive(Clone)]
pub enum RoutePath {
    ProjectList,
    SequenceList { project_id: Uuid },
    SequenceEdit { project_id: Uuid, sequence_id: Uuid },
}
impl From<String> for RoutePath {
    fn from(mut path_string: String) -> Self {
        if path_string.starts_with("/sequence_list") {
            let rest = path_string.split_off("/sequence_list".len());
            if let Ok(project_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::SequenceList { project_id };
            }
        }

        if path_string.starts_with("/sequence_edit") {
            let rest = path_string.split_off("/sequence_edit".len());
            let mut items = rest.split('/');
            items.next();
            let project_id = items.next();
            let sequence_id = items.next();

            if let (Some(project_id), Some(sequence_id)) = (project_id, sequence_id) {
                if let (Ok(project_id), Ok(sequence_id)) =
                    (Uuid::parse_str(project_id), Uuid::parse_str(sequence_id))
                {
                    return Self::SequenceEdit {
                        project_id,
                        sequence_id,
                    };
                }
            }
        }

        Self::ProjectList
    }
}
impl ToString for RoutePath {
    fn to_string(&self) -> String {
        match self {
            Self::ProjectList => "/".to_string(),
            Self::SequenceList { project_id } => format!("/sequence_list/{project_id}"),
            Self::SequenceEdit {
                project_id,
                sequence_id,
            } => format!("/sequence_edit/{project_id}/{sequence_id}"),
        }
    }
}
