use super::*;
use namui::prelude::*;
use std::sync::Arc;

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

pub enum Route {
    ProjectListPage(project_list_page::ProjectListPage),
    SequenceListPage(sequence_list_page::SequenceListPage),
    SequenceEditPage(sequence_edit_page::SequenceEditPage),
}

impl Router {
    pub fn new(route: Route) -> Self {
        Self { route }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<Event>() {
            match event {
                Event::Route(route) => self.route = (route)(),
            }
        }
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
}
