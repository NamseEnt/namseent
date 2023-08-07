use super::*;
use namui::prelude::*;

#[namui::component]
pub struct Router {
    pub wh: Wh<Px>,
}

impl Component for Router {
    fn render<'a>(self, ctx: &'a RenderCtx) -> RenderDone {
        let (route, set_route) = ctx.state(|| Route::from(get_path_from_hash()));

        ctx.web_event(move |web_event| {
            if let namui::web::WebEvent::HashChange { .. } = web_event {
                namui::log!("Hash change");
                set_route.set(Route::from(get_path_from_hash()));
            }
        });

        let wh = self.wh;

        ctx.compose(|ctx| match *route {
            Route::ProjectListPage => {
                ctx.add(project_list_page::ProjectListPage2 { wh });
            }
            Route::SequenceListPage { project_id } => {
                // ctx.add(sequence_list_page::SequenceListPage { wh, project_id })
            }
            Route::SequenceEditPage {
                project_id,
                sequence_id,
            } => {
                ctx.add(sequence_edit_page::SequenceEditPage {
                    wh,
                    project_id,
                    sequence_id,
                });
            }
        });

        ctx.done()
    }
}

#[derive(Debug)]
pub enum Route {
    ProjectListPage,
    SequenceListPage { project_id: Uuid },
    SequenceEditPage { project_id: Uuid, sequence_id: Uuid },
}
impl From<RoutePath> for Route {
    fn from(path: RoutePath) -> Self {
        match path {
            RoutePath::ProjectList => Self::ProjectListPage,
            RoutePath::SequenceList { project_id } => Self::SequenceListPage { project_id },
            RoutePath::SequenceEdit {
                project_id,
                sequence_id,
            } => {
                return Self::SequenceEditPage {
                    project_id,
                    sequence_id,
                }
            }
        }
    }
}

pub fn move_to(path: RoutePath) {
    web::execute_function_sync(
        "
        window.location.hash = hash;
    ",
    )
    .arg("hash", &path.to_string())
    .run::<()>();
}

fn get_path_from_hash() -> RoutePath {
    let hash: String = web::execute_function_sync(
        "
        return window.location.hash;
    ",
    )
    .run();
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
