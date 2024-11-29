use super::*;
use router::Route;
use rpc::project::create_new_project::*;

pub struct NewProjectPage {
    pub team_id: u128,
}

impl Component for NewProjectPage {
    fn render(self, ctx: &RenderCtx) {
        let Self { team_id } = self;
        let screen_wh = namui::screen::size().map(|x| x.into_px());
        let (project_name, set_project_name) = ctx.state(String::new);
        let (project_name_validate_err, set_project_name_validate_err) =
            ctx.state::<Option<String>>(|| None);
        let (create_new_project_err, set_create_new_project_err) =
            ctx.state::<Option<Error>>(|| None);

        let (submit, on_progress) = make_create_new_project_fn(
            ctx,
            || {
                if project_name.is_empty() {
                    set_project_name_validate_err.set(Some(
                        "프로젝트 이름이 비어있습니다. 프로젝트 이름을 입력해주세요".to_string(),
                    ));
                    return None;
                }

                set_project_name_validate_err.set(None);
                Some((
                    RefRequest {
                        team_id,
                        name: &project_name,
                    },
                    &team_id,
                ))
            },
            move |result| match result {
                Ok((Response { project_id }, team_id)) => {
                    toast::positive("프로젝트 생성 완료");
                    router::route(router::Route::Home {
                        initial_selection: home::Selection::Project {
                            team_id,
                            project_id,
                        },
                    });
                }
                Err(err) => {
                    set_create_new_project_err.set(Some(err));
                }
            },
        );

        let top_bar = table::fixed(
            24.px(),
            horizontal([
                ratio(1, |_, _| {}),
                fixed(24.px(), |wh, ctx| {
                    ctx.add(simple_button(wh, "X", |_| {
                        router::route(Route::Home {
                            initial_selection: home::Selection::Team { team_id },
                        });
                    }));
                }),
            ]),
        );

        ctx.compose(|ctx| {
            vertical([
                top_bar,
                fixed(24.px(), |wh, ctx| {
                    ctx.add(typography::title::left(
                        wh.height,
                        "새 프로젝트 만들기",
                        Color::WHITE,
                    ));
                }),
                fixed(16.px(), |wh, ctx| {
                    ctx.add(namui::text(TextParam {
                        text: "프로젝트 이름".to_string(),
                        x: 0.px(),
                        y: 12.px(),
                        align: TextAlign::Left,
                        baseline: TextBaseline::Middle,
                        font: Font {
                            name: "NotoSansKR-Regular".to_string(),
                            size: 12.int_px(),
                        },
                        style: TextStyle {
                            color: Color::WHITE,
                            ..Default::default()
                        },
                        max_width: Some(wh.width),
                    }));
                }),
                fixed(24.px(), |wh, ctx| {
                    ctx.add(TextInput {
                        rect: Rect::zero_wh(wh),
                        start_text: project_name.as_ref(),
                        text_align: TextAlign::Center,
                        text_baseline: TextBaseline::Middle,
                        font: Font {
                            size: 16.int_px(),
                            name: "NotoSansKR-Regular".to_string(),
                        },
                        style: Style {
                            rect: RectStyle {
                                stroke: Some(RectStroke {
                                    color: Color::WHITE,
                                    width: 1.px(),
                                    border_position: BorderPosition::Middle,
                                }),
                                fill: Some(RectFill {
                                    color: Color::grayscale_f01(0.3),
                                }),
                                round: Some(RectRound { radius: 4.px() }),
                            },
                            text: TextStyle {
                                color: Color::WHITE,
                                ..Default::default()
                            },
                            padding: Ltrb::all(8.px()),
                        },
                        prevent_default_codes: &[Code::Enter],
                        focus: None,
                        on_edit_done: &|text| {
                            set_project_name.set(text);
                        },
                    });
                }),
                if let Some(project_name_validate_err) = project_name_validate_err.as_ref() {
                    fixed(16.px(), |wh, ctx| {
                        ctx.add(namui::text(TextParam {
                            text: project_name_validate_err.to_string(),
                            x: 0.px(),
                            y: 12.px(),
                            align: TextAlign::Left,
                            baseline: TextBaseline::Middle,
                            font: Font {
                                name: "NotoSansKR-Regular".to_string(),
                                size: 12.int_px(),
                            },
                            style: TextStyle {
                                color: Color::RED,
                                ..Default::default()
                            },
                            max_width: Some(wh.width),
                        }));
                    })
                } else {
                    empty()
                },
                fixed(24.px(), |wh, ctx| {
                    ctx.add(simple_button(wh, "만들기", |_event| {
                        submit();
                    }));
                }),
                if let Some(err) = create_new_project_err.as_ref() {
                    let text = match err {
                        Error::NeedLogin => "로그인이 필요합니다".to_string(),
                        Error::TooManyProjects => "프로젝트를 더 이상 만들 수 없습니다".to_string(),
                        Error::InternalServerError { err } => format!("서버 오류: {}", err),
                        Error::PermissionDenied => "팀에 대한 권한이 없습니다".to_string(),
                    };
                    fixed(16.px(), |wh, ctx| {
                        ctx.add(namui::text(TextParam {
                            text,
                            x: 0.px(),
                            y: 12.px(),
                            align: TextAlign::Left,
                            baseline: TextBaseline::Middle,
                            font: Font {
                                name: "NotoSansKR-Regular".to_string(),
                                size: 12.int_px(),
                            },
                            style: TextStyle {
                                color: Color::RED,
                                ..Default::default()
                            },
                            max_width: Some(wh.width),
                        }));
                    })
                } else if on_progress {
                    fixed(16.px(), |wh, ctx| {
                        ctx.add(namui::text(TextParam {
                            text: "진행중...".to_string(),
                            x: 0.px(),
                            y: 12.px(),
                            align: TextAlign::Left,
                            baseline: TextBaseline::Middle,
                            font: Font {
                                name: "NotoSansKR-Regular".to_string(),
                                size: 12.int_px(),
                            },
                            style: TextStyle {
                                color: Color::WHITE,
                                ..Default::default()
                            },
                            max_width: Some(wh.width),
                        }));
                    })
                } else {
                    empty()
                },
            ])(screen_wh, ctx);
        });
    }
}
