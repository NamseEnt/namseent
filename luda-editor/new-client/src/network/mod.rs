mod dependencies;

use crate::*;
use dashmap::DashMap;
pub use dependencies::*;
use luda_rpc::rkyv;
use std::sync::Arc;
use tokio::{sync::oneshot, task::JoinHandle};

type SigRef<'a, T> = Sig<'a, T, &'a T>;

type Serializer = rkyv::ser::serializers::AllocSerializer<1024>;

pub fn server_rpc<
    'a,
    Req: rkyv::Serialize<Serializer> + Send + 'a,
    Deps: Dependencies + 'a,
    Response: rkyv::Archive + Send + 'static,
    Error: rkyv::Archive + Send + 'static,
>(
    ctx: &'a RenderCtx,
    request: impl FnOnce(Deps) -> Option<Req>,
    dependencies: Deps,
    api_index: u16,
) -> SigRef<'a, Option<Result<Response, Error>>>
where
    <Response as luda_rpc::rkyv::Archive>::Archived: luda_rpc::rkyv::Deserialize<
        Response,
        luda_rpc::rkyv::de::deserializers::SharedDeserializeMap,
    >,
    <Error as luda_rpc::rkyv::Archive>::Archived:
        luda_rpc::rkyv::Deserialize<Error, luda_rpc::rkyv::de::deserializers::SharedDeserializeMap>,
{
    let (server_connection, _) = ctx.atom(&SERVER_CONNECTION_ATOM);
    let server_connection = server_connection.clone();
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
        let request_packet = RequestPacket::new(api_index, bytes);

        let join_handle: JoinHandle<Result<()>> = tokio::spawn(async move {
            let response_packet = server_connection.request(request_packet).await?;

            let response = match response_packet.status {
                ResponseStatus::Response => {
                    let response = unsafe {
                        rkyv::from_bytes_unchecked(&response_packet.out_payload).unwrap()
                    };
                    Ok(response)
                }
                ResponseStatus::Error => {
                    let error = unsafe {
                        rkyv::from_bytes_unchecked(&response_packet.out_payload).unwrap()
                    };
                    Err(error)
                }
            };
            set_response.set(Some(response));
            Ok(())
        });

        EffectCleanUp::Once(Box::new(move || {
            join_handle.abort();
        }))
    });

    response
}

#[derive(Debug, Clone)]
pub struct ServerConnection {
    sender: namui::network::ws::WsSender,
    // receiver: namui::network::ws::WsReceiver,
    response_map: Arc<DashMap<u32, oneshot::Sender<Vec<u8>>>>,
}
impl ServerConnection {
    pub async fn new(url: impl ToString) -> Result<Self> {
        let (sender, mut receiver) = namui::network::ws::connect(url).await?;

        tokio::spawn(async move {});

        Ok(Self {
            sender,
            response_map: Default::default(),
        })
    }
    async fn request(
        &self,
        request_packet: RequestPacket,
    ) -> Result<ResponsePacket, oneshot::error::RecvError> {
        let packet_id = request_packet.packet_id;

        let (tx, rx) = oneshot::channel();
        self.response_map.insert(packet_id, tx);

        self.sender.send(request_packet.into_bytes());

        let out_packet_bytes = rx.await?;
        assert!(out_packet_bytes.len() >= 5);

        let (out_payload, header) = {
            let mut out_packet_bytes = out_packet_bytes;
            let header = out_packet_bytes.split_off(out_packet_bytes.len() - 5);
            (out_packet_bytes, header)
        };

        let status = match header[0] {
            0 => ResponseStatus::Response,
            1 => ResponseStatus::Error,
            _ => unreachable!("Invalid status: {}", header[0]),
        };

        Ok(ResponsePacket {
            status,
            out_payload,
        })
    }
}

struct RequestPacket {
    packet_id: u32,
    api_index: u16,
    in_payload: Vec<u8>,
}

#[repr(u8)]
enum ResponseStatus {
    Response = 0,
    Error = 1,
}

struct ResponsePacket {
    status: ResponseStatus,
    out_payload: Vec<u8>,
}

impl RequestPacket {
    fn new(api_index: u16, in_payload: Vec<u8>) -> Self {
        let packet_id = {
            static PACKET_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            PACKET_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };
        Self {
            packet_id,
            api_index,
            in_payload,
        }
    }
    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = self.in_payload;
        bytes.extend_from_slice(&self.packet_id.to_le_bytes());
        bytes.extend_from_slice(&self.api_index.to_le_bytes());
        bytes
    }
}

pub async fn connect_to_server(jwt: String) -> Result<ServerConnection> {
    todo!()
    // sender.send(jwt.as_bytes());

    // let data = receiver
    //     .recv()
    //     .await
    //     .ok_or(anyhow!("Failed to get auth response"))?;

    // Ok(ServerConnection {
    //     sender,
    //     // receiver,
    // })
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
