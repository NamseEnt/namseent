use super::*;
use namui::prelude::*;
use std::sync::{Arc, Once};
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;

static HASH_CHANGE_EVENT_LISTENER: Once = Once::new();

pub struct Router {
    route: Route,
}

pub struct Props {
    pub wh: Wh<Px>,
}

pub enum Event {
    Route(Arc<dyn Fn() -> Route + 'static>),
}
unsafe impl Send for Event {}
unsafe impl Sync for Event {}

enum InternalEvent {
    HashChanged(String),
}

pub enum Route {
    ProjectListPage(project_list_page::ProjectListPage),
    SequenceListPage(sequence_list_page::SequenceListPage),
    SequenceEditPage(sequence_edit_page::SequenceEditPage),
}

impl Router {
    pub fn new(route: Route) -> Self {
        Self::register_hash_change_event_listener();
        Self { route }
    }
    pub fn update(&mut self, event: &namui::Event) {
        event.is::<Event>(|event| match event {
            Event::Route(route) => self.route = (route)(),
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
            let listener = Closure::<dyn FnMut(_)>::new(move |event: HashChangeEvent| {
                InternalEvent::HashChanged(get_path_from_url(event.new_url()));
            });

            window
                .add_event_listener_with_callback("hashchange", listener.as_ref().unchecked_ref())
                .unwrap();

            listener.forget();
        })
    }
}

fn get_path_from_url(url: String) -> String {
    let Some((_, hash)) = url.split_once("#") else {
        return "/".to_string();
    };
    match hash.starts_with("/") {
        true => hash.to_string(),
        false => "/".to_string(),
    }
}
