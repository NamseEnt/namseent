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
            Route::ProjectList => {
                ctx.add(project_list_page::ProjectListPage { wh });
            }
            Route::SequenceList { project_id } => {
                ctx.add(sequence_list_page::SequenceListPage { wh, project_id });
            }
            Route::SequenceEdit {
                project_id,
                sequence_id,
            } => {
                ctx.add(sequence_edit_page::SequenceEditPage {
                    wh,
                    project_id,
                    sequence_id,
                });
            }
            Route::GraphicAssetManage { project_id } => {
                ctx.add(graphic_asset_manage_page::GraphicAssetManagePage { wh, project_id });
            }
            Route::ProjectAclManage { project_id } => {
                ctx.add(project_acl_manage_page::ProjectAclManagePage { wh, project_id });
            }
        });

        ctx.done()
    }
}

#[derive(Debug)]
pub enum Route {
    ProjectList,
    SequenceList { project_id: Uuid },
    SequenceEdit { project_id: Uuid, sequence_id: Uuid },
    GraphicAssetManage { project_id: Uuid },
    ProjectAclManage { project_id: Uuid },
}
impl From<String> for Route {
    fn from(mut path_string: String) -> Self {
        path_string = path_string.trim_start_matches('#').to_string();

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

        if path_string.starts_with("/graphic_asset_manage") {
            let rest = path_string.split_off("/graphic_asset_manage".len());
            if let Ok(project_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::GraphicAssetManage { project_id };
            }
        }

        if path_string.starts_with("/project_acl_manage") {
            let rest = path_string.split_off("/project_acl_manage".len());
            if let Ok(project_id) = Uuid::parse_str(rest.trim_matches('/')) {
                return Self::ProjectAclManage { project_id };
            }
        }

        Self::ProjectList
    }
}
impl ToString for Route {
    fn to_string(&self) -> String {
        match self {
            Self::ProjectList => "/".to_string(),
            Self::SequenceList { project_id } => format!("/sequence_list/{project_id}"),
            Self::SequenceEdit {
                project_id,
                sequence_id,
            } => format!("/sequence_edit/{project_id}/{sequence_id}"),
            Self::GraphicAssetManage { project_id } => {
                format!("/graphic_asset_manage/{project_id}")
            }
            Self::ProjectAclManage { project_id } => {
                format!("/project_acl_manage/{project_id}")
            }
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
