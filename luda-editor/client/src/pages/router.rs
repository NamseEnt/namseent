use super::*;
use namui::prelude::*;

#[namui::component]
pub struct Router {
    pub wh: Wh<Px>,
}

impl Component for Router {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (route, set_route) = ctx.state(|| Route::from(get_hash()));

        namui::web::event_listener_hash_change(move |event| {
            let new_url = event.new_url();
            let hash = new_url.split('#').nth(1).unwrap_or("");
            set_route.set(Route::from(hash.to_string()));
        });

        let wh = self.wh;

        ctx.compose(|ctx| match *route {
            Route::ProjectListPage => {
                ctx.add(project_list_page::ProjectListPage { wh });
            }
            Route::SequenceListPage { project_id } => {
                ctx.add(sequence_list_page::SequenceListPage { wh, project_id });
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Route {
    ProjectListPage,
    SequenceListPage { project_id: Uuid },
    SequenceEditPage { project_id: Uuid, sequence_id: Uuid },
}
impl From<String> for Route {
    fn from(mut path_string: String) -> Self {
        path_string = path_string.trim_start_matches('#').to_string();

        if path_string.starts_with("/sequence_list") {
            let rest = path_string.split_off("/sequence_list".len());
            if let Ok(project_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::SequenceListPage { project_id };
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
                    return Self::SequenceEditPage {
                        project_id,
                        sequence_id,
                    };
                }
            }
        }

        Self::ProjectListPage
    }
}
impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Self::ProjectListPage => "/".to_string(),
            Self::SequenceListPage { project_id } => format!("/sequence_list/{project_id}"),
            Self::SequenceEditPage {
                project_id,
                sequence_id,
            } => format!("/sequence_edit/{project_id}/{sequence_id}"),
        }
    }
}

pub fn move_to(route: Route) {
    web::execute_function(
        "
        window.location.hash = hash;
    ",
    )
    .arg("hash", route.to_string())
    .run::<()>();
}

fn get_hash() -> String {
    web::execute_function(
        "
        return window.location.hash;
    ",
    )
    .run()
}
