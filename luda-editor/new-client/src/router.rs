use super::*;

pub struct Router;

pub enum Route {
    Home,
    NewTeamPage,
}

static ROUTE_ATOM: Atom<Route> = Atom::uninitialized();

impl Component for Router {
    fn render(self, ctx: &RenderCtx) {
        let (route, _set_route) = ctx.init_atom(&ROUTE_ATOM, || Route::Home);
        match *route.as_ref() {
            Route::Home => {
                ctx.add(home::Home);
            }
            Route::NewTeamPage => {
                ctx.add(new_team_page::NewTeamPage);
            }
        }
    }
}

pub fn route(route: Route) {
    ROUTE_ATOM.set(route);
}
