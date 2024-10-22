use super::*;
use luda_rpc::*;

pub struct Home<'a> {
    pub initial_selection: &'a Selection,
}

#[derive(Clone)]
pub enum Selection {
    Nothing,
    Team { team_id: String },
    Project { team_id: String, project_id: String },
}

impl Component for Home<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { initial_selection } = self;
        let (selection, set_selection) = ctx.state(|| initial_selection.clone());

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        ctx.compose(|ctx| {
            horizontal([
                ratio(1, |wh, ctx| {
                    ctx.add(TeamList {
                        wh,
                        on_select_team: &|team| {
                            set_selection.set(Selection::Team {
                                team_id: team.id.clone(),
                            });
                        },
                    });
                }),
                ratio(1, |wh, ctx| {
                    ctx.add(Team {
                        wh,
                        team_id: match selection.as_ref() {
                            Selection::Team { team_id } => Some(team_id),
                            Selection::Project { team_id, .. } => Some(team_id),
                            _ => None,
                        },
                        on_select_project: &|project| {
                            let team_id = match selection.as_ref() {
                                Selection::Team { team_id } => team_id.clone(),
                                Selection::Project { team_id, .. } => team_id.clone(),
                                _ => unreachable!(),
                            };
                            set_selection.set(Selection::Project {
                                team_id,
                                project_id: project.id.clone(),
                            });
                        },
                    });
                }),
                ratio(1, |wh, ctx| {
                    ctx.add(EpisodeList {
                        wh,
                        team_id: match selection.as_ref() {
                            Selection::Team { team_id } => Some(team_id),
                            Selection::Project { team_id, .. } => Some(team_id),
                            _ => None,
                        },
                        project_id: match selection.as_ref() {
                            Selection::Project { project_id, .. } => Some(project_id),
                            _ => None,
                        },
                        on_select_episode: &|episode| {
                            let team_id = match selection.as_ref() {
                                Selection::Team { team_id } => team_id.clone(),
                                Selection::Project { team_id, .. } => team_id.clone(),
                                _ => unreachable!(),
                            };
                            let project_id = match selection.as_ref() {
                                Selection::Project { project_id, .. } => project_id.clone(),
                                _ => unreachable!(),
                            };
                            router::route(router::Route::EpisodeEditor {
                                team_id,
                                project_id,
                                episode_id: episode.id.clone(),
                            });
                        },
                    });
                }),
            ])(screen_wh, ctx);
        });
    }
}

struct TeamList<'a> {
    wh: Wh<Px>,
    on_select_team: &'a dyn Fn(&luda_rpc::Team),
}

impl Component for TeamList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, on_select_team } = self;

        let title = || {
            [fixed(24.px(), |wh, ctx| {
                ctx.add(typography::center_text(
                    wh,
                    "팀 리스트",
                    Color::WHITE,
                    16.int_px(),
                ));
            })]
        };

        use crate::rpc::team::get_my_teams::*;
        get_my_teams_render(
            ctx,
            |_| Some((RefRequest {}, ())),
            (),
            || {
                ctx.compose(|ctx| vertical(title())(wh, ctx));
            },
            |err| {
                ctx.add(typography::center_text(
                    wh,
                    format!("로딩 실패: {err:?}"),
                    Color::RED,
                    16.int_px(),
                ));
            },
            |(Response { teams }, _)| {
                ctx.compose(|ctx| {
                    vertical(
                        title()
                            .into_iter()
                            .chain(teams.iter().map(|team| {
                                fixed(24.px(), move |wh, ctx| {
                                    ctx.add(
                                        typography::center_text(
                                            wh,
                                            &team.name,
                                            Color::WHITE,
                                            16.int_px(),
                                        )
                                        .attach_event(
                                            |event| {
                                                if let Event::MouseUp { event } = event {
                                                    if event.is_local_xy_in()
                                                        && event.button == Some(MouseButton::Left)
                                                    {
                                                        on_select_team(team);
                                                    }
                                                }
                                            },
                                        ),
                                    );
                                })
                            }))
                            .chain([fixed(24.px(), |wh, ctx| {
                                ctx.add(simple_button(wh, "새 팀 만들기", |_event| {
                                    router::route(router::Route::NewTeam);
                                }));
                            })])
                            .chain([fixed(24.px(), |wh, ctx| {
                                ctx.add(simple_button(wh, "팀 가입하기", |_event| {}));
                            })]),
                    )(wh, ctx)
                });
            },
        );
    }
}

struct Team<'a> {
    wh: Wh<Px>,
    team_id: Option<&'a String>,
    on_select_project: &'a dyn Fn(&Project),
}
impl Component for Team<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            team_id,
            on_select_project,
        } = self;

        ctx.compose(|ctx| {
            vertical([
                fixed(24.px(), |wh, ctx| {
                    ctx.add(AssetManageOpenButton { wh, team_id });
                }),
                ratio(1, |wh, ctx| {
                    ctx.add(ProjectList {
                        wh,
                        team_id,
                        on_select_project,
                    });
                }),
            ])(wh, ctx);
        });
    }
}

struct AssetManageOpenButton<'a> {
    wh: Wh<Px>,
    team_id: Option<&'a String>,
}
impl Component for AssetManageOpenButton<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, team_id } = self;

        ctx.add(simple_button(wh, "에셋 관리", |_event| {
            let Some(team_id) = team_id else {
                toast::negative("팀을 먼저 선택해주세요");
                return;
            };
            router::route(router::Route::AssetManage {
                team_id: team_id.clone(),
            });
        }));
    }
}

struct ProjectList<'a> {
    wh: Wh<Px>,
    team_id: Option<&'a String>,
    on_select_project: &'a dyn Fn(&Project),
}

impl Component for ProjectList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            team_id,
            on_select_project,
        } = self;

        let title = || {
            [fixed(24.px(), |wh, ctx| {
                ctx.add(typography::center_text(
                    wh,
                    "프로젝트 리스트",
                    Color::WHITE,
                    16.int_px(),
                ));
            })]
        };

        use crate::rpc::project::get_projects::*;
        get_projects_render(
            ctx,
            |team_id| {
                let team_id = team_id.as_ref()?;
                Some((RefRequest { team_id }, *team_id))
            },
            team_id,
            || {
                ctx.compose(|ctx| vertical(title())(wh, ctx));
            },
            |err| {
                ctx.add(typography::center_text(
                    wh,
                    format!("로딩 실패: {err:?}"),
                    Color::RED,
                    16.int_px(),
                ));
            },
            |(Response { projects }, team_id)| {
                ctx.compose(|ctx| {
                    vertical(
                        title()
                            .into_iter()
                            .chain(projects.iter().map(|project| {
                                fixed(24.px(), move |wh, ctx| {
                                    ctx.add(
                                        typography::center_text(
                                            wh,
                                            &project.name,
                                            Color::WHITE,
                                            16.int_px(),
                                        )
                                        .attach_event(
                                            |event| {
                                                if let Event::MouseUp { event } = event {
                                                    if event.is_local_xy_in()
                                                        && event.button == Some(MouseButton::Left)
                                                    {
                                                        on_select_project(project);
                                                    }
                                                }
                                            },
                                        ),
                                    );
                                })
                            }))
                            .chain([fixed(24.px(), |wh, ctx| {
                                ctx.add(simple_button(wh, "새 프로젝트", |_event| {
                                    router::route(router::Route::NewProject {
                                        team_id: team_id.clone(),
                                    });
                                }));
                            })]),
                    )(wh, ctx)
                });
            },
        );
    }
}

struct EpisodeList<'a> {
    wh: Wh<Px>,
    team_id: Option<&'a String>,
    project_id: Option<&'a String>,
    on_select_episode: &'a dyn Fn(&Episode),
}

impl Component for EpisodeList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            team_id,
            project_id,
            on_select_episode,
        } = self;

        let title = || {
            [fixed(24.px(), |wh, ctx| {
                ctx.add(typography::center_text(
                    wh,
                    "에피소드 리스트",
                    Color::WHITE,
                    16.int_px(),
                ));
            })]
        };

        use crate::rpc::episode::get_episodes::*;
        get_episodes_render(
            ctx,
            |project_id| {
                Some((
                    RefRequest {
                        project_id: project_id?,
                    },
                    (),
                ))
            },
            project_id,
            || {
                ctx.compose(|ctx| vertical(title())(wh, ctx));
            },
            |err| {
                ctx.add(typography::center_text(
                    wh,
                    format!("로딩 실패: {err:?}"),
                    Color::RED,
                    16.int_px(),
                ));
            },
            |(Response { episodes }, _)| {
                ctx.compose(|ctx| {
                    vertical(
                        title()
                            .into_iter()
                            .chain(episodes.iter().map(|episode| {
                                fixed(24.px(), |wh, ctx| {
                                    ctx.add(
                                        typography::center_text(
                                            wh,
                                            &episode.name,
                                            Color::WHITE,
                                            16.int_px(),
                                        )
                                        .attach_event(
                                            |event| {
                                                if let Event::MouseUp { event } = event {
                                                    if event.is_local_xy_in()
                                                        && event.button == Some(MouseButton::Left)
                                                    {
                                                        on_select_episode(episode);
                                                    }
                                                }
                                            },
                                        ),
                                    );
                                })
                            }))
                            .chain([fixed(24.px(), |wh, ctx| {
                                ctx.add(simple_button(wh, "새 에피소드", |_event| {
                                    let (Some(team_id), Some(project_id)) = (team_id, project_id)
                                    else {
                                        toast::negative(
                                            "오류가 발생했습니다. 새로고침 후 다시 시도해주세요",
                                        );
                                        return;
                                    };
                                    router::route(router::Route::NewEpisode {
                                        team_id: team_id.clone(),
                                        project_id: project_id.clone(),
                                    });
                                }));
                            })]),
                    )(wh, ctx)
                });
            },
        );
    }
}
