use super::*;
use luda_rpc::Scene;

pub struct EpisodeEditor<'a> {
    episode_id: &'a String,
}

impl Component for EpisodeEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { episode_id } = self;
        // let (server_connection, _) = ctx.atom(&SERVER_CONNECTION_ATOM);

        crate::rpc::episode_editor::join_episode_editor::join_episode_editor(
            ctx,
            |episode_id| {
                Option::<(
                    luda_rpc::episode_editor::join_episode_editor::RefRequest,
                    (),
                )>::None
                // Some((
                //     crate::rpc::episode_editor::join_episode_editor::RefRequest { episode_id },
                //     (),
                // ))
            },
            episode_id,
        );

        // ctx.async_effect(
        //     "try get lock",
        //     (server_connection, episode_id),
        //     |(server_connection, episode_id)| async move {
        //         match server_connection
        //             .join_episode_editor(
        //                 crate::rpc::episode_editor::join_episode_editor::RefRequest {
        //                     episode_id: &episode_id,
        //                 },
        //             )
        //             .await
        //         {
        //             Ok(response) => {
        //                 // response.scenes
        //             }
        //             Err(_error) => {
        //                 todo!("Show error and go home")
        //             }
        //         }
        //     },
        // );

        /*
        락을 건다
        - 실패하면 지금은 에러를 띄우고 홈으로 돌아가게 하자.
        - 나중엔 락이 풀릴 때까지 읽기모드로. 누가 락을 걸고 있는지 보여주고.

        렌더링에 필요한 모든 데이터를 가져온다. 최신이여야만 한다.
        열심히 수정한다.
        */

        // let (scenes, set_scenes) = ctx.state(|| None);
        let (selected_scene_id, set_selected_scene_id) = ctx.state(|| Option::<String>::None);

        // let scenes = crate::rpc::scene::get_scenes::get_scenes(
        //     ctx,
        //     |_| Some((crate::rpc::scene::get_scenes::RefRequest { episode_id }, ())),
        //     (),
        // );

        let scene_list = scene_list(&[]);
        let scene_editor = table::ratio(1, |wh, ctx| {});
        let properties_panel = table::ratio(1, |wh, ctx| {});

        let wh = namui::screen::size().map(|x| x.into_px());
        ctx.compose(|ctx| horizontal([scene_list, scene_editor, properties_panel])(wh, ctx));
    }
}

fn scene_list(scenes: &[Scene]) -> TableCell<'_> {
    table::fixed(160.px(), |wh, ctx| {
        vertical([
            table::fixed(40.px(), |wh, ctx| {
                ctx.add(typography::center_text(
                    wh,
                    "Scene",
                    Color::WHITE,
                    16.int_px(),
                ));
            }),
            table::ratio(1, |wh, ctx| {
                ctx.add(list_view::AutoListView {
                    height: wh.height,
                    scroll_bar_width: 20.px(),
                    item_wh: Wh::new(wh.width, 200.px()),
                    items: scenes.iter().enumerate().map(|(index, scene)| {
                        (scene.id.as_str().into(), SceneListCell { index, scene })
                    }),
                });
            }),
        ])(wh, ctx)
    })
}

struct SceneListCell<'a> {
    index: usize,
    scene: &'a Scene,
}

impl Component for SceneListCell<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { index, scene } = self;
        /*
        썸네일을 어떻게 보여줄 것인가?
        저장하고 보여줄 것인가, 매번 새로 그릴 것인가?
        뭐가 되었든, 처음에는 그려야한다.
        그리고 난 다음에 저장할 것인가? 말 것인가?
        1. 매번 그리는게 부담되는가?
        2. 저장된 것을 불러오는 것은 부담되는가?
        3.
        */

        // for sprite in scene.sprites {
        //     sprite.sprite_id
        // }
        todo!()
    }
}
