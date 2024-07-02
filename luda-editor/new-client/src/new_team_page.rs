use super::*;

pub struct NewTeamPage;

impl Component for NewTeamPage {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = namui::screen::size().map(|x| x.into_px());
        let (team_name, set_team_name) = ctx.state(String::new);
        let (team_name_validate_err, set_team_name_validate_err) =
            ctx.state::<Option<String>>(|| None);
        let (create_team_job, set_create_team_job) =
            ctx.state::<Job<CreateTeamRes, String>>(|| Job::NotStarted);

        let validate = || {
            if team_name.is_empty() {
                set_team_name_validate_err.set(Some(
                    "팀 이름이 비어있습니다. 팀 이름을 입력해주세요".to_string(),
                ));
                return;
            }

            set_team_name_validate_err.set(None);
        };

        let submit = || {
            if create_team_job.is_in_progress() {
                return;
            }

            validate();
            set_create_team_job.set(Job::InProgress);

            ctx.spawn(async move {
                let result = Result::<(), String>::Ok(());

                match result {
                    Ok(_) => {
                        toast::positive("팀 생성 완료");
                        router::route(router::Route::Home);
                    }
                    Err(err) => {
                        set_create_team_job.set(Job::Err(err));
                    }
                }
            });
        };

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
                        style: Default::default(),
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
                if let Job::Err(err) = create_team_job.as_ref() {
                    fixed(16.px(), |wh, ctx| {
                        ctx.add(namui::text(TextParam {
                            text: err.to_string(),
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
            ])(screen_wh, ctx);
        });
    }
}

struct CreateTeamRes {}

enum Job<Ok, Err> {
    NotStarted,
    InProgress,
    Ok(Ok),
    Err(Err),
}

impl<Ok, Err> Job<Ok, Err> {
    fn is_not_started(&self) -> bool {
        matches!(self, Job::NotStarted)
    }

    fn is_in_progress(&self) -> bool {
        matches!(self, Job::InProgress)
    }

    fn is_ok(&self) -> bool {
        matches!(self, Job::Ok(_))
    }

    fn is_err(&self) -> bool {
        matches!(self, Job::Err(_))
    }
}
