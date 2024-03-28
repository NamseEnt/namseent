use namui::*;
use namui_prebuilt::simple_rect;
use std::{sync::Arc, vec};

#[namui::component]
pub struct Game {}
impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let map_wh = Wh::new(48, 48);
        let check_points = vec![
            Xy::new(6, 0),
            Xy::new(6, 23),
            Xy::new(41, 23),
            Xy::new(41, 6),
            Xy::new(24, 6),
            Xy::new(24, 41),
            Xy::new(47, 41),
        ];

        let (game_phase, set_game_phase) =
            ctx.state(|| GamePhase::TowerBuild(TowerBuildPhase::ChooseLocation));
        let (game_block_map, set_game_block_map) =
            ctx.state(|| GameBlockMap::new(map_wh, check_points));
        let (monster_spawner, set_monster_spawner) = ctx.state::<Option<MonsterSpawner>>(|| None);
        let (monsters, set_monsters) = ctx.state::<Vec<Monster>>(Vec::new);

        ctx.effect("game_phase", || {
            println!("game_phase: {:?}", game_phase.as_ref());
        });

        let monster_path = ctx.memo(|| {
            let now = std::time::Instant::now();
            let monster_path = Arc::new(game_block_map.monster_path());
            namui::log!("monster_path calculate time: {:?}", now.elapsed());
            monster_path
        });

        let roll_towers = || -> [Tower; 5] {
            [
                Tower::EasyTower,
                Tower::EasyTower,
                Tower::EasyTower,
                Tower::EasyTower,
                Tower::EasyTower,
            ]
        };

        let on_monster_exit_to_end = || {
            // TODO
        };

        ctx.interval("move monsters", Duration::from_secs_f32(1.0 / 60.0), |dt| {
            if monsters.is_empty() {
                return;
            }

            let monster_path = monster_path.clone();
            set_monsters.mutate(move |monsters| {
                monsters.retain_mut(|monster| {
                    let moved_progress_on_dt = monster.velocity * dt;
                    let progress = monster.location.progress + moved_progress_on_dt;
                    let steps = progress as usize;

                    if steps > 1 {
                        namui::log!(
                            "[Warning]steps({steps}) is bigger than 1. \
                                dt({dt:?}) or speed({velocity:?}) is too big.",
                            velocity = monster.velocity
                        );
                    }

                    let rest_progress = progress - steps as f64;
                    monster.location.progress = rest_progress;

                    if steps == 0 {
                        return true;
                    }

                    monster.location.path_index += steps;

                    if monster.location.path_index >= monster_path.len() {
                        on_monster_exit_to_end();
                        return false;
                    }
                    true
                })
            });

            if monsters.is_empty() && monster_spawner.is_none() {
                set_game_phase.set(GamePhase::TowerBuild(TowerBuildPhase::ChooseLocation));
            }
        });

        ctx.interval("spawn monsters", Duration::from_secs_f32(1.0), |_| {
            if monster_spawner.is_none() {
                return;
            }

            let new_monster = Monster {
                location: MonsterLocation {
                    progress: 0.0,
                    path: monster_path.as_ref().clone(),
                    path_index: 0,
                },
                velocity: Per::new(3.0, Duration::from_secs_f32(1.0)),
            };

            set_monsters.mutate(move |monsters| monsters.push(new_monster));

            set_monster_spawner.mutate(|monster_spawner| {
                {
                    let Some(monster_spawner) = monster_spawner else {
                        return;
                    };

                    if monster_spawner.count > 0 {
                        monster_spawner.count -= 1;
                        return;
                    }
                }
                *monster_spawner = None;
            })
        });

        if let &GamePhase::TowerBuild(TowerBuildPhase::ChooseTower {
            location,
            ref towers,
        }) = game_phase.as_ref()
        {
            ctx.on_raw_event(|event| {
                if let RawEvent::KeyUp { event } = event {
                    if event.code == Code::Escape {
                        set_game_phase.set(GamePhase::TowerBuild(TowerBuildPhase::ChooseLocation));
                        return;
                    }

                    if event.code == Code::KeyR {
                        set_game_phase.set(GamePhase::TowerBuild(TowerBuildPhase::ChooseTower {
                            location,
                            towers: roll_towers(),
                        }));
                        return;
                    }

                    let tower_index = match event.code {
                        Code::Digit1 => 1,
                        Code::Digit2 => 2,
                        Code::Digit3 => 3,
                        Code::Digit4 => 4,
                        Code::Digit5 => 5,
                        _ => {
                            return;
                        }
                    };

                    let tower = towers[tower_index].clone();

                    set_game_block_map
                        .mutate(move |game_block_map| game_block_map.build_tower(location, tower));
                    set_game_phase.set(GamePhase::RockBuild { left_counts: 4 });
                }
            });
        }

        const BLOCK_WIDTH: Px = px(64.0);
        const BLOCK_HEIGHT: Px = px(64.0);

        ctx.compose(|ctx| {
            for monster in monsters.as_ref() {
                monster.render(Wh::new(BLOCK_WIDTH, BLOCK_HEIGHT), ctx);
            }
        });

        ctx.compose(|ctx| {
            game_block_map.render(
                &MapRenderingProps {
                    block_wh: Wh::new(BLOCK_WIDTH, BLOCK_HEIGHT),
                    on_click_block: &|block, xy| {
                        println!("click block: {:?}, {:?}", block, xy);
                        match block {
                            Block::Empty => match game_phase.as_ref() {
                                GamePhase::TowerBuild(phase) => {
                                    if let TowerBuildPhase::ChooseLocation = phase {
                                        if !game_block_map.is_buildable(xy) {
                                            // show cannot build tower
                                            println!("cannot build tower at {:?}", xy);
                                            return;
                                        }
                                        set_game_phase.set(GamePhase::TowerBuild(
                                            TowerBuildPhase::ChooseTower {
                                                location: xy,
                                                towers: roll_towers(),
                                            },
                                        ));
                                    }
                                }
                                GamePhase::RockBuild { left_counts } => {
                                    if !game_block_map.is_buildable(xy) {
                                        // show cannot build rock
                                        println!("cannot build rock at {:?}", xy);
                                        return;
                                    }

                                    set_game_block_map.mutate(move |game_block_map| {
                                        game_block_map.build_tower(xy, Tower::Rock);
                                    });

                                    let next_game_phase = if *left_counts > 1 {
                                        GamePhase::RockBuild {
                                            left_counts: left_counts - 1,
                                        }
                                    } else {
                                        GamePhase::Defense
                                    };

                                    if next_game_phase == GamePhase::Defense {
                                        set_monster_spawner.set(Some(MonsterSpawner { count: 10 }));
                                    }

                                    set_game_phase.set(next_game_phase);
                                }
                                GamePhase::Defense => {}
                            },
                            Block::Tower(_) => {
                                // show upgrade or sell button
                            }
                        }
                    },
                },
                ctx,
            )
        });
    }
}

#[derive(Debug, PartialEq)]
enum GamePhase {
    TowerBuild(TowerBuildPhase),
    RockBuild { left_counts: usize },
    Defense,
}

#[derive(Debug, PartialEq)]
enum TowerBuildPhase {
    ChooseLocation,
    ChooseTower {
        location: Xy<usize>,
        towers: [Tower; 5],
    },
}

#[derive(Debug)]
struct Monster {
    location: MonsterLocation,
    velocity: Per<f64, Duration>,
}
impl Monster {
    fn render(&self, block_wh: Wh<Px>, ctx: &ComposeCtx) {
        let from = self.location.path[self.location.path_index];
        let to = self.location.path[self.location.path_index + 1];

        let xy = block_wh.as_xy()
            * (from.map(|x| x as f32) * (1.0 - self.location.progress) as f32
                + to.map(|x| x as f32) * self.location.progress as f32);

        ctx.translate(xy)
            .add(simple_rect(block_wh, Color::RED, 1.px(), Color::GREEN));
    }
}

#[derive(Debug)]
struct MonsterLocation {
    progress: f64,
    path_index: usize,
    path: Arc<MonsterPath>,
}

#[derive(Debug)]
struct MonsterSpawner {
    count: usize,
}

struct MapRenderingProps<'a> {
    block_wh: Wh<Px>,
    on_click_block: &'a dyn Fn(&Block, Xy<usize>),
}

#[derive(Debug, PartialEq, Clone)]
enum Tower {
    EasyTower,
    Rock,
}
impl Tower {
    fn render(&self, props: &MapRenderingProps) -> RenderingTree {
        match self {
            Tower::EasyTower => {
                let xy = props.block_wh.as_xy() / 10.0;
                let wh = props.block_wh - xy.as_wh() * 2.0;
                namui::path(
                    Path::new().add_oval(Rect::from_xy_wh(props.block_wh.as_xy() / 10.0, wh)),
                    Paint::new(Color::RED),
                )
            }
            Tower::Rock => {
                let xy = props.block_wh.as_xy() / 10.0;
                let wh = props.block_wh - xy.as_wh() * 2.0;
                namui::path(
                    Path::new().add_rect(Rect::from_xy_wh(props.block_wh.as_xy() / 10.0, wh)),
                    Paint::new(Color::grayscale_f01(0.5)),
                )
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct EasyTower {}

#[derive(Debug, Clone, PartialEq)]
enum Block {
    Empty,
    Tower(Tower),
}

impl Block {
    fn is_buildable(&self) -> bool {
        matches!(self, Block::Empty)
    }
    fn is_monster_passable(&self) -> bool {
        match self {
            Block::Empty => true,
            Block::Tower(_) => false,
        }
    }

    fn render(&self, props: &MapRenderingProps) -> RenderingTree {
        match self {
            Block::Empty => simple_rect(
                props.block_wh,
                Color::grayscale_f01(0.2),
                1.px(),
                Color::WHITE,
            ),
            Block::Tower(tower) => tower.render(props),
        }
    }
}

#[derive(Debug)]
struct GameBlockMap {
    wh: Wh<usize>,
    /// height[width]
    block_array: Vec<Vec<Block>>,
    check_points: Vec<Xy<usize>>,
}

type MonsterPath = Vec<Xy<usize>>;

impl GameBlockMap {
    fn new(wh: Wh<usize>, check_points: Vec<Xy<usize>>) -> Self {
        assert!(check_points
            .iter()
            .all(|xy| xy.x < wh.width && xy.y < wh.height));

        let array = vec![vec![Block::Empty; wh.width]; wh.height];

        Self {
            wh,
            block_array: array,
            check_points,
        }
    }

    // Tip: You can save the monster_passable_map to avoid recalculate it every time
    fn monster_passable_map(&self) -> Vec<Vec<bool>> {
        self.block_array
            .iter()
            .map(|row| {
                row.iter()
                    .map(|block| block.is_monster_passable())
                    .collect()
            })
            .collect::<Vec<Vec<bool>>>()
    }

    fn monster_path(&self) -> MonsterPath {
        let monster_passable_array = self.monster_passable_map();

        let mut path = vec![];
        for i in 0..self.check_points.len() - 1 {
            let start = self.check_points[i];
            let end = self.check_points[i + 1];
            let mut sub_path = crate::path_finding::find_path(
                start,
                end,
                monster_passable_array.as_ref(),
                self.wh.width,
                self.wh.height,
            )
            .unwrap();
            if i != 0 {
                sub_path.remove(0);
            }
            path.extend(sub_path);
        }
        path
    }

    fn iter(&self) -> GameBlockMapIter {
        GameBlockMapIter {
            game_block_map: self,
            next_xy: Some(Xy::new(0, 0)),
        }
    }

    pub fn render<'a>(&'a self, props: &MapRenderingProps, ctx: &'a ComposeCtx) {
        for (block, xy) in self.iter() {
            ctx.compose(|ctx| {
                ctx.translate(Xy::new(
                    props.block_wh.width * xy.x,
                    props.block_wh.height * xy.y,
                ))
                .add(simple_rect(
                    props.block_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    Color::TRANSPARENT,
                ))
                .attach_event(|event| {
                    if let Event::MouseDown { event } = event {
                        if event.is_local_xy_in() {
                            (props.on_click_block)(block, xy);
                        }
                    }
                })
                .add(block.render(props));
            });
        }
    }
    pub fn is_buildable(&self, xy: Xy<usize>) -> bool {
        if xy.x >= self.wh.width
            && xy.y >= self.wh.height
            && !self.block_array[xy.y][xy.x].is_buildable()
            && self.check_points.contains(&xy)
        {
            return false;
        }

        let mut monster_passable_map = self.monster_passable_map();
        monster_passable_map[xy.y][xy.x] = true;

        crate::path_finding::find_path(
            *self.check_points.first().unwrap(),
            *self.check_points.last().unwrap(),
            &monster_passable_map,
            self.wh.width,
            self.wh.height,
        )
        .is_some()
    }

    /// Assume that xy is buildable, not blocking monster path
    fn build_tower(&mut self, xy: Xy<usize>, tower: Tower) {
        self.block_array[xy.y][xy.x] = Block::Tower(tower);
    }
}

struct GameBlockMapIter<'a> {
    game_block_map: &'a GameBlockMap,
    next_xy: Option<Xy<usize>>,
}

impl<'a> Iterator for GameBlockMapIter<'a> {
    type Item = (&'a Block, Xy<usize>);
    fn next(&mut self) -> Option<Self::Item> {
        let xy = self.next_xy?;
        let block = &self.game_block_map.block_array[xy.y][xy.x];

        self.next_xy = if xy.x < self.game_block_map.wh.width - 1 {
            Some(Xy::new(xy.x + 1, xy.y))
        } else if xy.y < self.game_block_map.wh.height - 1 {
            Some(Xy::new(0, xy.y + 1))
        } else {
            None
        };

        Some((block, xy))
    }
}
