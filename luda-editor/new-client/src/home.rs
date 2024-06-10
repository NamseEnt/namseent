use super::*;

pub struct Home;

impl Component for Home {
    fn render(self, ctx: &RenderCtx) {
        let (teams, set_teams) = ctx.state::<OptionDataFetch<Vec<Team>>>(|| None);
        let (projects, set_projects) = ctx.state::<OptionDataFetch<Vec<Project>>>(|| None);
        let (episodes, set_episodes) = ctx.state::<OptionDataFetch<Vec<Episode>>>(|| None);

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        ctx.compose(|ctx| {
            horizontal([
                ratio(1, |wh, ctx| {
                    option_data_fetch(&ctx, &teams, wh, |teams| TeamList {
                        wh,
                        teams: teams.as_slice(),
                    });
                }),
                ratio(1, |wh, ctx| {
                    option_data_fetch(&ctx, &projects, wh, |projects| ProjectList {
                        wh,
                        projects: projects.as_slice(),
                    });
                }),
                ratio(1, |wh, ctx| {
                    option_data_fetch(&ctx, &episodes, wh, |episodes| EpisodeList {
                        wh,
                        episodes: episodes.as_slice(),
                    });
                }),
            ])(screen_wh, ctx);
        });
    }
}

struct Team {
    name: String,
}
struct Project {
    name: String,
}

struct Episode {
    name: String,
}

struct TeamList<'a> {
    wh: Wh<Px>,
    teams: &'a [Team],
}

impl Component for TeamList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, teams } = self;

        ctx.compose(|ctx| {
            vertical(
                [].into_iter()
                    .chain([fixed(24.px(), |wh, ctx| {
                        ctx.add(typography::center_text(
                            wh,
                            "팀 리스트",
                            Color::WHITE,
                            16.int_px(),
                        ));
                    })])
                    .chain(teams.iter().map(|team| {
                        fixed(24.px(), |wh, ctx| {
                            ctx.add(typography::center_text(
                                wh,
                                &team.name,
                                Color::WHITE,
                                16.int_px(),
                            ));
                        })
                    }))
                    .chain([fixed(24.px(), |wh, ctx| {
                        ctx.add(simple_button(wh, "새 팀 만들기", |_event| {
                            router::route(router::Route::NewTeamPage);
                        }));
                    })])
                    .chain([fixed(24.px(), |wh, ctx| {
                        ctx.add(simple_button(wh, "팀 가입하기", |_event| {}));
                    })]),
            )(wh, ctx)
        });
    }
}

struct ProjectList<'a> {
    wh: Wh<Px>,
    projects: &'a [Project],
}

impl Component for ProjectList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, projects } = self;

        ctx.compose(|ctx| {
            vertical(
                [].into_iter()
                    .chain(projects.iter().map(|project| {
                        fixed(24.px(), |wh, ctx| {
                            ctx.add(typography::center_text(
                                wh,
                                &project.name,
                                Color::WHITE,
                                16.int_px(),
                            ));
                        })
                    }))
                    .chain([fixed(24.px(), |wh, ctx| {
                        ctx.add(simple_button(wh, "새 프로젝트", |_event| {}));
                    })]),
            )(wh, ctx)
        });
    }
}

struct EpisodeList<'a> {
    wh: Wh<Px>,
    episodes: &'a [Episode],
}

impl Component for EpisodeList<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, episodes } = self;

        ctx.compose(|ctx| {
            vertical(
                [].into_iter()
                    .chain(episodes.iter().map(|episode| {
                        fixed(24.px(), |wh, ctx| {
                            ctx.add(typography::center_text(
                                wh,
                                &episode.name,
                                Color::WHITE,
                                16.int_px(),
                            ));
                        })
                    }))
                    .chain([fixed(24.px(), |wh, ctx| {
                        ctx.add(simple_button(wh, "새 에피소드", |_event| {}));
                    })]),
            )(wh, ctx)
        });
    }
}
