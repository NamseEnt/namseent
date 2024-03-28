mod top_bar;

use crate::app::notification;
use crate::app::notification::Notification;
use crate::color;
use crate::RPC;
use namui::prelude::*;
use namui::text_input::Style;
use namui_prebuilt::button::TextButton;
use namui_prebuilt::list_view::AutoListView;
use namui_prebuilt::typography;
use namui_prebuilt::{simple_rect, table::hooks::*};
use rpc::list_user_acls::UserAcl;
use rpc::types::ProjectAclUserPermission;
use std::fmt::Write;
use std::ops::Deref;
use top_bar::TopBar;

const BUTTON_WIDTH: Px = px(128.0);

#[namui::component]
pub struct ProjectAclManagePage {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
}

impl Component for ProjectAclManagePage {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, project_id } = self;

        const TOP_BAR_HEIGHT: Px = px(48.0);
        const MAX_LIST_WIDTH: Px = px(768.0);
        const ITEM_HEIGHT: Px = px(36.0);
        const SCROLL_BAR_WIDTH: Px = px(4.0);

        let project_id = ctx.track_eq(&project_id);
        let list_width = wh.width.min(MAX_LIST_WIDTH);
        let (acls, set_acls) = ctx.state::<Vec<rpc::list_user_acls::UserAcl>>(Vec::new);

        let update_acl = |user_id: Uuid, permission: Option<ProjectAclUserPermission>| {
            let project_id = *project_id;
            RPC.edit_user_acl(rpc::edit_user_acl::Request {
                project_id,
                user_id,
                permission,
            })
            .callback(move |result| match result {
                Ok(_) => {
                    start_fetch_graphic_assets(project_id, set_acls);
                }
                Err(error) => {
                    notification::error!("Failed to update user acl - {user_id}: {error}").push();
                }
            });
        };

        ctx.effect("Fetch acls", || {
            start_fetch_graphic_assets(*project_id, set_acls);
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
                        ratio(1, |_, _| {}),
                        fixed(
                            list_width,
                            vertical([
                                fixed(ITEM_HEIGHT, |wh, ctx| {
                                    ctx.add(typography::body::left(
                                        wh.height,
                                        "Enter user id(ex: 2bb95042-efbb-4a43-a7f1-d1d10328d09b)",
                                        color::STROKE_NORMAL,
                                    ));
                                }),
                                fixed(ITEM_HEIGHT, |wh, ctx| {
                                    ctx.add(EditorAdder {
                                        wh,
                                        update_acl: &update_acl,
                                    });
                                }),
                                fixed(ITEM_HEIGHT, |_, _| {}),
                                ratio(1, |wh, ctx| {
                                    let item_wh = Wh::new(wh.width, ITEM_HEIGHT);
                                    ctx.add(AutoListView {
                                        height: wh.height,
                                        scroll_bar_width: SCROLL_BAR_WIDTH,
                                        item_wh,
                                        items: acls
                                            .iter()
                                            .map(|acl| {
                                                (
                                                    acl.user_id.to_string(),
                                                    ListItem {
                                                        wh: item_wh,
                                                        acl,
                                                        update_acl: &update_acl,
                                                    },
                                                )
                                            })
                                            .collect(),
                                    });
                                }),
                            ]),
                        ),
                        ratio(1, |_, _| {}),
                    ]),
                ),
            ])(wh, ctx)
        });

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ));

        
    }
}

#[component]
struct ListItem<'a> {
    wh: Wh<Px>,
    acl: &'a UserAcl,
    update_acl: &'a dyn Fn(Uuid, Option<ProjectAclUserPermission>),
}
impl Component for ListItem<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            acl,
            update_acl,
        } = self;
        const PADDING: Px = px(8.0);
        ctx.compose(|ctx| {
            horizontal([
                ratio(
                    1,
                    padding(PADDING, |wh, ctx| {
                        ctx.add(typography::body::left(
                            wh.height,
                            acl.user_name.clone(),
                            color::STROKE_NORMAL,
                        ));
                    }),
                ),
                fixed(BUTTON_WIDTH, |wh, ctx| {
                    ctx.add(TextButton {
                        rect: wh.to_rect(),
                        text: "Remove",
                        text_color: color::STROKE_NORMAL,
                        stroke_color: color::STROKE_NORMAL,
                        stroke_width: 1.px(),
                        fill_color: color::BACKGROUND,
                        mouse_buttons: vec![MouseButton::Left],
                        on_mouse_up_in: &|_| {
                            update_acl(acl.user_id, None);
                        },
                    });
                }),
            ])(wh, ctx)
        });
        ctx.component(simple_rect(
            wh,
            color::STROKE_NORMAL,
            1.px(),
            color::BACKGROUND,
        ));
        
    }
}

#[component]
struct EditorAdder<'a> {
    wh: Wh<Px>,
    update_acl: &'a dyn Fn(Uuid, Option<ProjectAclUserPermission>),
}
impl Component for EditorAdder<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self { wh, update_acl } = self;
        const PADDING: Ltrb<Px> = Ltrb {
            left: px(8.0),
            top: px(0.0),
            right: px(8.0),
            bottom: px(0.0),
        };
        let (input_value, set_input_value) = ctx.state::<String>(String::new);
        let text_input_instance = TextInputInstance::new(ctx);
        let add_user_as_editor = || {
            let Ok(user_id) = Uuid::parse_str(&input_value) else {
                notification::error!("Invalid user id - {input_value}").push();
                return;
            };
            update_acl(user_id, Some(ProjectAclUserPermission::Editor));
        };

        ctx.compose(|ctx| {
            horizontal([
                ratio(1, |wh, ctx| {
                    ctx.add(TextInput {
                        instance: text_input_instance,
                        rect: wh.to_rect(),
                        text: input_value.deref().clone(),
                        text_align: TextAlign::Left,
                        text_baseline: TextBaseline::Middle,
                        font: Font {
                            size: typography::adjust_font_size(wh.height),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: Style {
                            rect: RectStyle {
                                stroke: Some(RectStroke {
                                    color: color::STROKE_NORMAL,
                                    width: 1.px(),
                                    border_position: BorderPosition::Inside,
                                }),
                                fill: Some(RectFill {
                                    color: color::BACKGROUND,
                                }),
                                round: None,
                            },
                            text: TextStyle {
                                color: color::STROKE_NORMAL,
                                ..Default::default()
                            },
                            padding: PADDING,
                        },
                        prevent_default_codes: vec![Code::Enter],
                        on_event: &|event| match event {
                            text_input::Event::TextUpdated { text } => {
                                set_input_value.set(text.to_string());
                            }
                            text_input::Event::KeyDown { event } => {
                                if event.code == Code::Enter {
                                    add_user_as_editor();
                                }
                            }
                            _ => {}
                        },
                    });
                }),
                fixed(BUTTON_WIDTH, |wh, ctx| {
                    ctx.add(TextButton {
                        rect: wh.to_rect(),
                        text: "Add",
                        text_color: color::STROKE_NORMAL,
                        stroke_color: color::STROKE_NORMAL,
                        stroke_width: 1.px(),
                        fill_color: color::BACKGROUND,
                        mouse_buttons: vec![MouseButton::Left],
                        on_mouse_up_in: &|_| {
                            add_user_as_editor();
                        },
                    });
                }),
            ])(wh, ctx)
        });
        
    }
}

fn start_fetch_graphic_assets(
    project_id: Uuid,
    set_acls: SetState<Vec<rpc::list_user_acls::UserAcl>>,
) {
    spawn_local(async move {
        let mut acls = Vec::new();
        let mut last_key = None;
        loop {
            match RPC
                .list_user_acls(rpc::list_user_acls::Request {
                    project_id,
                    last_key,
                })
                .await
            {
                Ok(response) => {
                    acls.extend(response.user_acls);
                    if let Some(next_key) = response.next_key {
                        last_key = Some(next_key);
                        continue;
                    }
                }
                Err(error) => {
                    let mut error_message = "Failed to fetch acls, ".to_string();
                    match error {
                        rpc::list_user_acls::Error::Unauthorized => {
                            let _ =
                                write!(error_message, "Please login with project owner account");
                        }
                        rpc::list_user_acls::Error::Unknown(error) => {
                            let _ = write!(error_message, "{:?}", error);
                        }
                    }
                    Notification::error(error_message).push();
                }
            }
            break;
        }
        set_acls.set(acls);
    });
}
