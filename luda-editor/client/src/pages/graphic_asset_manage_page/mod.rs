mod auto_column_list;
mod cg_list;
mod image_list;
mod side_bar;
mod top_bar;

use crate::{
    app::notification::{self, Notification},
    pages::graphic_asset_manage_page::{
        cg_list::CgList, image_list::ImageList, side_bar::SideBar, top_bar::TopBar,
    },
};
use futures::join;
use namui::prelude::*;
use namui_prebuilt::table::hooks::*;
use rpc::data::{CgFile, ImageWithLabels};

static TAB_ATOM: Atom<Tab> = Atom::uninitialized_new();
static IMAGES_ATOM: Atom<Vec<ImageWithLabels>> = Atom::uninitialized_new();
static CG_FILES_ATOM: Atom<Vec<CgFile>> = Atom::uninitialized_new();

#[namui::component]
pub struct GraphicAssetManagePage {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for GraphicAssetManagePage {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        const TOP_BAR_HEIGHT: Px = px(48.0);
        const SIDE_BAR_WIDTH: Px = px(192.0);

        let Self { wh, project_id } = self;

        let project_id = ctx.track_eq(&project_id);
        let (tab, _set_tab) = ctx.atom_init(&TAB_ATOM, || Tab::Image);
        let (_, set_images) = ctx.atom_init(&IMAGES_ATOM, || Vec::new());
        let (_, set_cg_files) = ctx.atom_init(&CG_FILES_ATOM, || Vec::new());

        let start_fetch_graphic_assets = |project_id: Uuid| {
            spawn_local(async move {
                let fetch_images = || async {
                    let loading_notification =
                        Notification::info("Loading images...".to_string()).set_loading(true);
                    let notification_id = notification::push_notification(loading_notification);
                    match crate::RPC
                        .list_images(rpc::list_images::Request { project_id })
                        .await
                    {
                        Ok(rpc::list_images::Response { images }) => set_images.set(images),
                        Err(error) => {
                            let _ = notification::push_notification(Notification::error(format!(
                                "Loading images failed: {error}"
                            )));
                        }
                    };
                    notification::remove_notification(notification_id);
                };

                let fetch_cg_files = || async {
                    let loading_notification =
                        Notification::info("Loading cg_files...".to_string()).set_loading(true);
                    let notification_id = notification::push_notification(loading_notification);
                    match crate::RPC
                        .list_cg_files(rpc::list_cg_files::Request { project_id })
                        .await
                    {
                        Ok(rpc::list_cg_files::Response { cg_files }) => set_cg_files.set(cg_files),
                        Err(error) => {
                            let _ = notification::push_notification(Notification::error(format!(
                                "Loading images failed: {error}"
                            )));
                        }
                    };
                    notification::remove_notification(notification_id);
                };

                join!(fetch_images(), fetch_cg_files());
            })
        };

        ctx.effect("Fetch graphic assets every project_id changes", || {
            start_fetch_graphic_assets(*project_id);
        });

        ctx.compose(|ctx| {
            vertical([
                fixed(TOP_BAR_HEIGHT, |wh, ctx| {
                    ctx.add(TopBar {
                        wh: Wh::new(wh.width, TOP_BAR_HEIGHT),
                        project_id: *project_id,
                    });
                }),
                ratio(1, |wh, ctx| {
                    horizontal([
                        fixed(SIDE_BAR_WIDTH, |wh, ctx| {
                            ctx.add(SideBar { wh });
                        }),
                        ratio(1, |wh, ctx| {
                            match *tab {
                                Tab::Image => ctx.add(ImageList {
                                    wh,
                                    project_id: *project_id,
                                }),
                                Tab::Cg => ctx.add(CgList {
                                    wh,
                                    project_id: *project_id,
                                }),
                            };
                        }),
                    ])(wh, ctx);
                }),
            ])(wh, ctx)
        });

        ctx.done()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Image,
    Cg,
}
impl ToString for Tab {
    fn to_string(&self) -> String {
        match self {
            Tab::Image => "Image".to_string(),
            Tab::Cg => "Cg".to_string(),
        }
    }
}
