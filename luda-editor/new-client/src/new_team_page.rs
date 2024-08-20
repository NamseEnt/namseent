use super::*;
use rpc::team::create_new_team::*;

pub struct NewTeamPage;

impl Component for NewTeamPage {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = namui::screen::size().map(|x| x.into_px());
        let (team_name, set_team_name) = ctx.state(String::new);
        let (team_name_validate_err, set_team_name_validate_err) =
            ctx.state::<Option<String>>(|| None);
        let (create_new_team_err, set_create_new_team_err) = ctx.state::<Option<Error>>(|| None);

        let (submit, on_progress) = make_create_new_team_fn(
            ctx,
            || {
                if team_name.is_empty() {
                    set_team_name_validate_err.set(Some(
                        "팀 이름이 비어있습니다. 팀 이름을 입력해주세요".to_string(),
                    ));
                    return None;
                }

                set_team_name_validate_err.set(None);
                Some((RefRequest { name: &team_name }, ()))
            },
            move |result| match result {
                Ok((Response { team_id }, _)) => {
                    toast::positive("팀 생성 완료");
                    router::route(router::Route::Home {
                        initial_selection: home::Selection::Team { team_id },
                    });
                }
                Err(err) => {
                    set_create_new_team_err.set(Some(err));
                }
            },
        );

        ctx.compose(|ctx| {
            vertical([
                fixed(24.px(), |wh, ctx| {
                    ctx.add(typography::title::left(
                        wh.height,
                        "새 팀 만들기",
                        Color::WHITE,
                    ));
                }),
                fixed(16.px(), |wh, ctx| {
                    ctx.add(namui::text(TextParam {
                        text: "팀 이름".to_string(),
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
                        start_text: team_name.as_ref(),
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
                            set_team_name.set(text);
                        },
                    });
                }),
                if let Some(team_name_validate_err) = team_name_validate_err.as_ref() {
                    fixed(16.px(), |wh, ctx| {
                        ctx.add(namui::text(TextParam {
                            text: team_name_validate_err.to_string(),
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
                if let Some(err) = create_new_team_err.as_ref() {
                    let text = match err {
                        Error::NeedLogin => "로그인이 필요합니다".to_string(),
                        Error::TooManyTeams => "팀을 더 이상 만들 수 없습니다".to_string(),
                        Error::DuplicatedName => "이미 존재하는 팀 이름입니다".to_string(),
                        Error::InternalServerError { err } => format!("서버 오류: {}", err),
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
