mod dependencies;

use crate::*;
pub use dependencies::*;
use luda_rpc::rkyv;

type SigRef<'a, T> = Sig<'a, T, &'a T>;

pub fn server_rpc<
    'a,
    Req: rkyv::Serialize<rkyv::ser::serializers::AllocSerializer<1024>> + Send + 'a,
    Deps: Dependencies + 'a,
    Response: Send + 'static,
    Error: Send + 'static,
>(
    ctx: &'a RenderCtx,
    request: impl FnOnce(Deps) -> Option<Req>,
    dependencies: Deps,
) -> SigRef<'a, Option<Result<Response, Error>>> {
    let (server_connection, _) = ctx.atom(&SERVER_CONNECTION_ATOM);
    let server_connection = *server_connection;
    let (response, set_response) = ctx.state(|| None);
    let dependencies_changed = dependencies.changed(ctx);
    let deps_sig = ctx.controlled_memo(|_| {
        if dependencies_changed {
            return ControlledMemo::Changed(());
        }
        ControlledMemo::Unchanged(())
    });

    ctx.effect("server get", || {
        deps_sig.record_as_used();

        let Some(req) = request(dependencies) else {
            return EffectCleanUp::None;
        };

        let set_response = set_response.cloned();
        let bytes = rkyv::to_bytes(&req).unwrap().to_vec();
        let request_packet = RequestPacket { bytes };

        let join_handle = tokio::spawn(async move {
            let response = server_connection.request(request_packet).await;
            set_response.set(Some(response));
        });

        EffectCleanUp::Once(Box::new(move || {
            join_handle.abort();
        }))
    });

    response
}

#[derive(Debug, Clone, Copy)]
pub struct ServerConnection {
    sender: namui::network::ws::WsSender,
    // receiver: namui::network::ws::WsReceiver,
}
impl ServerConnection {
    pub async fn request<Response, Error>(
        &self,
        request_packet: RequestPacket,
    ) -> Result<Response, Error> {
        todo!()
    }
}

struct RequestPacket {
    bytes: Vec<u8>,
}

pub async fn connect_to_server(jwt: String) -> Result<ServerConnection> {
    let (sender, mut receiver) = namui::network::ws::connect("ws://localhost:8080").await?;

    sender.send(jwt.as_bytes());

    let data = receiver
        .recv()
        .await
        .ok_or(anyhow!("Failed to get auth response"))?;

    Ok(ServerConnection {
        sender,
        // receiver,
    })
}

// // GetTeams

// pub struct GetTeamsReq {}
// impl Request for GetTeamsReq {
//     fn as_packet(&self) -> RequestPacket {
//         RequestPacket {}
//     }
// }
// pub struct GetTeamsRes {
//     pub teams: Vec<Team>,
// }
// #[derive(Debug)]
// pub enum GetTeamsErr {
//     InternalServerError(String),
//     NetworkError(String),
// }

// pub type GetTeamsOptionResult = Option<Result<GetTeamsRes, GetTeamsErr>>;

// pub trait GetTeamsReqOptional: Into<Option<GetTeamsReq>> {}
// impl GetTeamsReqOptional for GetTeamsReq {}
// impl GetTeamsReqOptional for Option<GetTeamsReq> {}

// pub fn get_teams<'a, Request: GetTeamsReqOptional, Deps: Dependencies + 'a>(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
// ) -> Sig<'a, GetTeamsOptionResult, &'a GetTeamsOptionResult> {
//     server_get(ctx, |deps| request(deps).into(), dependencies)
// }

// pub fn get_teams_render<'a, Request: GetTeamsReqOptional, Deps: Dependencies + 'a>(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
//     on_loading: impl FnOnce(),
//     on_err: impl FnOnce(&GetTeamsErr),
//     on_res: impl FnOnce(&GetTeamsRes),
// ) {
//     match get_teams(ctx, request, dependencies).as_ref() {
//         Some(result) => match result {
//             Ok(res) => on_res(res),
//             Err(err) => on_err(err),
//         },
//         None => on_loading(),
//     }
// }

// // GetProjects

// pub struct GetProjectsReq<'a> {
//     pub team_id: &'a String,
// }
// impl Request for GetProjectsReq<'_> {
//     fn as_packet(&self) -> RequestPacket {
//         todo!()
//     }
// }
// pub struct GetProjectsRes {
//     pub projects: Vec<Project>,
// }
// #[derive(Debug)]
// pub enum GetProjectsErr {
//     InternalServerError(String),
//     NetworkError(String),
// }

// pub type GetProjectsOptionResult = Option<Result<GetProjectsRes, GetProjectsErr>>;

// pub trait GetProjectsReqOptional<'a> {
//     fn into(self) -> Option<GetProjectsReq<'a>>;
// }
// impl<'a> GetProjectsReqOptional<'a> for GetProjectsReq<'a> {
//     fn into(self) -> Option<GetProjectsReq<'a>> {
//         Some(self)
//     }
// }
// impl<'a> GetProjectsReqOptional<'a> for Option<GetProjectsReq<'a>> {
//     fn into(self) -> Option<GetProjectsReq<'a>> {
//         self
//     }
// }

// pub fn get_projects<'a, Request: GetProjectsReqOptional<'a> + 'a, Deps: Dependencies + 'a>(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
// ) -> Sig<'a, GetProjectsOptionResult, &'a GetProjectsOptionResult> {
//     server_get(ctx, |deps| request(deps).into(), dependencies)
// }

// pub fn get_projects_render<
//     'a,
//     Request: GetProjectsReqOptional<'a> + 'a,
//     Deps: Dependencies + 'a,
// >(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
//     on_loading: impl FnOnce(),
//     on_err: impl FnOnce(&GetProjectsErr),
//     on_res: impl FnOnce(&GetProjectsRes),
// ) {
//     match get_projects(ctx, request, dependencies).as_ref() {
//         Some(result) => match result {
//             Ok(res) => on_res(res),
//             Err(err) => on_err(err),
//         },
//         None => on_loading(),
//     }
// }

// // GetEpisodes

// pub struct GetEpisodesReq<'a> {
//     pub project_id: &'a String,
// }
// impl Request for GetEpisodesReq<'_> {
//     fn as_packet(&self) -> RequestPacket {
//         todo!()
//     }
// }
// pub struct GetEpisodesRes {
//     pub episodes: Vec<Episode>,
// }
// #[derive(Debug)]
// pub enum GetEpisodesErr {
//     InternalServerError(String),
//     NetworkError(String),
// }

// pub type GetEpisodesOptionResult = Option<Result<GetEpisodesRes, GetEpisodesErr>>;

// pub trait GetEpisodesReqOptional<'a> {
//     fn into(self) -> Option<GetEpisodesReq<'a>>;
// }
// impl<'a> GetEpisodesReqOptional<'a> for GetEpisodesReq<'a> {
//     fn into(self) -> Option<GetEpisodesReq<'a>> {
//         Some(self)
//     }
// }
// impl<'a> GetEpisodesReqOptional<'a> for Option<GetEpisodesReq<'a>> {
//     fn into(self) -> Option<GetEpisodesReq<'a>> {
//         self
//     }
// }

// pub fn get_episodes<'a, Request: GetEpisodesReqOptional<'a> + 'a, Deps: Dependencies + 'a>(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
// ) -> Sig<'a, GetEpisodesOptionResult, &'a GetEpisodesOptionResult> {
//     server_get(ctx, |deps| request(deps).into(), dependencies)
// }

// pub fn get_episodes_render<
//     'a,
//     Request: GetEpisodesReqOptional<'a> + 'a,
//     Deps: Dependencies + 'a,
// >(
//     ctx: &'a RenderCtx,
//     request: impl FnOnce(Deps) -> Request,
//     dependencies: Deps,
//     on_loading: impl FnOnce(),
//     on_err: impl FnOnce(&GetEpisodesErr),
//     on_res: impl FnOnce(&GetEpisodesRes),
// ) {
//     match get_episodes(ctx, request, dependencies).as_ref() {
//         Some(result) => match result {
//             Ok(res) => on_res(res),
//             Err(err) => on_err(err),
//         },
//         None => on_loading(),
//     }
// }
