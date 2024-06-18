mod auto_column_list;
mod cg_list;
mod cg_viewer;
mod image_list;
mod image_viewer;
mod side_bar;
mod top_bar;
mod upload_asset;

use crate::{
    app::notification::{self},
    pages::graphic_asset_manage_page::{
        cg_list::CgList,
        cg_viewer::CgViewer,
        image_list::ImageList,
        image_viewer::ImageViewer,
        side_bar::SideBar,
        top_bar::TopBar,
        upload_asset::{add_new_image, upload_file},
    },
};
use futures::join;
use namui::*;
use namui_prebuilt::{simple_rect, table::*};
use rpc::data::{CgFile, ImageWithLabels, ScreenCg, ScreenCgPart};
use std::ops::Deref;

static TAB_ATOM: Atom<Tab> = Atom::uninitialized_new();
static IMAGES_ATOM: Atom<Vec<ImageWithLabels>> = Atom::uninitialized_new();
static CG_FILES_ATOM: Atom<Vec<CgFile>> = Atom::uninitialized_new();
static SELECTED_ASSET_ATOM: Atom<Option<SelectedAsset>> = Atom::uninitialized_new();

pub struct GraphicAssetManagePage {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for GraphicAssetManagePage {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, project_id } = self;

        const TOP_BAR_HEIGHT: Px = px(48.0);
        const SIDE_BAR_WIDTH: Px = px(192.0);

        let project_id = ctx.track_eq(&project_id);
        let (tab, _set_tab) = ctx.atom_init(&TAB_ATOM, || Tab::Image);
        let (selected_asset, _) = ctx.atom_init(&SELECTED_ASSET_ATOM, || None);
        let _ = ctx.atom_init(&IMAGES_ATOM, Vec::new);
        let (cg_files, _) = ctx.atom_init(&CG_FILES_ATOM, Vec::new);

        ctx.effect("Fetch graphic assets every project_id changes", || {
            start_fetch_graphic_assets(*project_id);
        });

        let on_viewer_close = || {
            SELECTED_ASSET_ATOM.set(None);
        };
        ctx.compose(|ctx| {
            let Some(selected_asset) = selected_asset.deref() else {
                return;
            };

            match selected_asset {
                SelectedAsset::Image(image) => {
                    ctx.add_with_key(
                        "image_viewer",
                        ImageViewer {
                            wh,
                            image,
                            project_id: *project_id,
                            on_close: &on_viewer_close,
                        },
                    );
                }
                SelectedAsset::Cg(screen_cg) => {
                    let cg_file = cg_files
                        .iter()
                        .find(|cg_file| screen_cg.id == cg_file.id)
                        .unwrap();
                    ctx.add_with_key(
                        "cg_viewer",
                        CgViewer {
                            wh,
                            project_id: *project_id,
                            cg_file,
                            screen_cg,
                            on_event: &|event| match event {
                                cg_viewer::Event::Close => on_viewer_close(),
                                cg_viewer::Event::UnselectCgPart { cg_part_name } => {
                                    update_cg_part(cg_part_name, |part| part.unselect())
                                }
                                cg_viewer::Event::TurnOnCgPartVariant {
                                    cg_part_name,
                                    cg_part_variant_name,
                                } => update_cg_part(cg_part_name, move |part| {
                                    part.turn_on(cg_part_variant_name.clone())
                                }),
                                cg_viewer::Event::TurnOffCgPartVariant {
                                    cg_part_name,
                                    cg_part_variant_name,
                                } => update_cg_part(cg_part_name, move |part| {
                                    part.turn_off(cg_part_variant_name.clone())
                                }),
                            },
                        },
                    );
                }
            }
        });

        ctx.compose(|ctx| {
            vertical([
                fixed(TOP_BAR_HEIGHT, |wh, ctx| {
                    ctx.add(TopBar {
                        wh: Wh::new(wh.width, TOP_BAR_HEIGHT),
                        project_id: *project_id,
                    });
                }),
                ratio(
                    1,
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
                    ]),
                ),
            ])(wh, ctx)
        });

        ctx.component(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                match event {
                    Event::KeyDown { event } => {
                        if !(event.code == Code::KeyV && namui::keyboard::ctrl_press()) {
                            return;
                        }
                        let project_id = *project_id;
                        spawn_local(async move {
                            if let Ok(buffers) = clipboard::read_image_buffers().await {
                                for png_bytes in buffers {
                                    add_new_image(project_id, png_bytes);
                                }
                            }
                        });
                    }
                    Event::DragAndDrop { event } => {
                        let project_id = *project_id;
                        for file in event.files {
                            spawn_local(async move {
                                upload_file(&file, project_id).await;
                            });
                        }
                    }
                    _ => {}
                }
            }),
        );

        
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

#[derive(Debug, Clone)]
enum SelectedAsset {
    Image(ImageWithLabels),
    Cg(ScreenCg),
}

fn start_fetch_graphic_assets(project_id: Uuid) {
    spawn_local(async move {
        let fetch_images = || async {
            let loading_notification = notification::info!("Loading images...").set_loading(true);
            let notification_id = loading_notification.push();
            match crate::RPC
                .list_images(rpc::list_images::Request { project_id })
                .await
            {
                Ok(rpc::list_images::Response { images }) => IMAGES_ATOM.set(images),
                Err(error) => {
                    notification::error!("Loading images failed: {error}").push();
                }
            };
            notification::remove_notification(notification_id);
        };

        let fetch_cg_files = || async {
            let notification_id = notification::info!("Loading cg_files...")
                .set_loading(true)
                .push();
            match crate::RPC
                .list_cg_files(rpc::list_cg_files::Request { project_id })
                .await
            {
                Ok(rpc::list_cg_files::Response { cg_files }) => CG_FILES_ATOM.set(cg_files),
                Err(error) => {
                    notification::error!("Loading images failed: {error}").push();
                }
            };
            notification::remove_notification(notification_id);
        };

        join!(fetch_images(), fetch_cg_files());
    })
}

fn update_cg_part<Update>(part_name: String, update: Update)
where
    Update: Fn(&mut ScreenCgPart) + Send + Sync + 'static,
{
    SELECTED_ASSET_ATOM.mutate(move |selected_asset| {
        let Some(SelectedAsset::Cg(screen_cg)) = selected_asset else {
            return;
        };
        let part = screen_cg
            .parts
            .iter_mut()
            .find(|part| part.name() == part_name)
            .unwrap();
        update(part);
    });
}
