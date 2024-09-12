use super::*;

pub struct Router;

pub enum Route {
    Home { initial_selection: home::Selection },
    NewTeam,
    NewProject { team_id: String },
    AssetManage { team_id: String },
}

static ROUTE_ATOM: Atom<Route> = Atom::uninitialized();

impl Component for Router {
    fn render(self, ctx: &RenderCtx) {
        let (route, _set_route) = ctx.init_atom(&ROUTE_ATOM, || Route::Home {
            initial_selection: home::Selection::Nothing,
        });
        match route.as_ref() {
            Route::Home { initial_selection } => {
                ctx.add(home::Home { initial_selection });
            }
            Route::NewTeam => {
                ctx.add(new_team_page::NewTeamPage);
            }
            Route::NewProject { team_id } => {
                ctx.add(new_project_page::NewProjectPage { team_id });
            }
            Route::AssetManage { team_id } => {
                ctx.add(asset_manage_page::AssetManagePage { team_id });
            }
        }
    }
}

pub fn route(route: Route) {
    ROUTE_ATOM.set(route);
}
