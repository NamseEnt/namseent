use crate::card::{Rank, Suit};
use crate::game_state::MonsterKind;
use crate::game_state::background::BackgroundKind;
use crate::game_state::field_particle::particle_kind::ParticleKind;
use crate::game_state::tower::{AnimationKind, TowerKind};
use crate::icon::IconKind;
use crate::rarity::Rarity;
use crate::theme::{palette, typography};
use namui::skia::load_image_from_resource_location;
use namui::tokio::task::JoinSet;
use namui::*;
use namui_prebuilt::simple_rect;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::OnceLock;

static BACKGROUND_ASSET_LOADER: OnceLock<AssetLoader<BackgroundKind>> = OnceLock::new();
static FACE_CARD_ASSET_LOADER: OnceLock<AssetLoader<(Rank, Suit)>> = OnceLock::new();
static ICON_ASSET_LOADER: OnceLock<AssetLoader<IconKind>> = OnceLock::new();
static MONSTER_ASSET_LOADER: OnceLock<AssetLoader<MonsterKind>> = OnceLock::new();
static PARTICLE_ASSET_LOADER: OnceLock<AssetLoader<ParticleKind>> = OnceLock::new();
static TOWER_ASSET_LOADER: OnceLock<AssetLoader<(TowerKind, AnimationKind)>> = OnceLock::new();

pub fn get_background_asset(key: BackgroundKind) -> Option<Image> {
    BACKGROUND_ASSET_LOADER.get()?.get(key)
}
pub fn get_face_card_asset(key: (Rank, Suit)) -> Option<Image> {
    FACE_CARD_ASSET_LOADER.get()?.get(key)
}
pub fn get_icon_asset(key: IconKind) -> Option<Image> {
    ICON_ASSET_LOADER.get()?.get(key)
}
pub fn get_monster_asset(key: MonsterKind) -> Option<Image> {
    MONSTER_ASSET_LOADER.get()?.get(key)
}
pub fn get_particle_asset(key: ParticleKind) -> Option<Image> {
    PARTICLE_ASSET_LOADER.get()?.get(key)
}
pub fn get_tower_asset(key: (TowerKind, AnimationKind)) -> Option<Image> {
    TOWER_ASSET_LOADER.get()?.get(key)
}

pub struct LoadingScreen<'a> {
    pub screen_wh: Wh<Px>,
    pub on_complete: &'a dyn Fn(),
}

enum State {
    Loading {
        progress: f32,
    },
    Error {
        resource_location: ResourceLocation,
        error: anyhow::Error,
    },
}

impl Component for LoadingScreen<'_> {
    fn render(self, ctx: &RenderCtx) {
        let (state, set_state) = ctx.state(|| State::Loading { progress: 0.0 });

        ctx.effect("complete on progress 1", || {
            if let State::Loading { progress } = state.as_ref()
                && *progress >= 1.0
            {
                (self.on_complete)();
            }
        });

        ctx.effect("start load", || {
            let mut set = start_load_assets();

            ctx.spawn(async move {
                let total_count = set.len();
                while let Some(result) = set.join_next().await {
                    match result.unwrap() {
                        Ok(_) => {
                            set_state.set(State::Loading {
                                progress: (total_count - set.len()) as f32 / total_count as f32,
                            });
                        }
                        Err((location, error)) => {
                            set_state.set(State::Error {
                                resource_location: location,
                                error,
                            });
                            return;
                        }
                    }
                }
            });
        });

        match state.as_ref() {
            State::Error {
                resource_location,
                error,
            } => {
                ctx.translate(Xy::new(0.px(), self.screen_wh.height / 2.0 - 20.px()))
                    .add(
                        typography::headline(format!(
                            "Failed to load resource: {resource_location}",
                        ))
                        .size(typography::FontSize::Large)
                        .align(typography::TextAlign::Center {
                            wh: Wh::new(self.screen_wh.width, 0.px()),
                        })
                        .build(),
                    );

                ctx.translate(Xy::new(0.px(), self.screen_wh.height / 2.0 + 20.px()))
                    .add(
                        typography::paragraph(error.to_string())
                            .size(typography::FontSize::Medium)
                            .align(typography::TextAlign::Center {
                                wh: Wh::new(self.screen_wh.width, 0.px()),
                            })
                            .build(),
                    );

                ctx.add(simple_rect(
                    self.screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    palette::SURFACE,
                ));
            }
            &State::Loading { progress } => {
                let progress_percent = (progress * 100.0) as u32;

                ctx.translate(Xy::new(0.px(), self.screen_wh.height / 2.0 - 20.px()))
                    .add(
                        typography::headline(format!("{progress_percent}%"))
                            .size(typography::FontSize::Large)
                            .align(typography::TextAlign::Center {
                                wh: Wh::new(self.screen_wh.width, 0.px()),
                            })
                            .build(),
                    );

                let progress_bar_width = 400.px();
                let progress_bar_height = 8.px();
                let progress_bar_x = self.screen_wh.width / 2.0 - progress_bar_width / 2.0;
                let progress_bar_y = self.screen_wh.height / 2.0 + 20.px();

                let fill_width = progress_bar_width * progress;
                if fill_width > 0.px() {
                    ctx.translate(Xy::new(progress_bar_x, progress_bar_y))
                        .add(simple_rect(
                            Wh::new(fill_width, progress_bar_height),
                            Color::TRANSPARENT,
                            0.px(),
                            palette::PRIMARY,
                        ));
                }

                ctx.translate(Xy::new(progress_bar_x, progress_bar_y))
                    .add(simple_rect(
                        Wh::new(progress_bar_width, progress_bar_height),
                        palette::OUTLINE,
                        1.px(),
                        palette::SURFACE_CONTAINER_LOW,
                    ));

                ctx.add(simple_rect(
                    self.screen_wh,
                    Color::TRANSPARENT,
                    0.px(),
                    palette::SURFACE,
                ));
            }
        }
    }
}

fn start_load_assets() -> JoinSet<Result<(), (ResourceLocation, anyhow::Error)>> {
    let mut set = JoinSet::new();
    load(
        &mut set,
        [
            BackgroundKind::Tile0,
            BackgroundKind::Tile1,
            BackgroundKind::Tile2,
            BackgroundKind::Tile3,
        ],
        &BACKGROUND_ASSET_LOADER,
    );
    load(
        &mut set,
        [
            (Rank::Jack, Suit::Spades),
            (Rank::Jack, Suit::Hearts),
            (Rank::Jack, Suit::Diamonds),
            (Rank::Jack, Suit::Clubs),
            (Rank::Queen, Suit::Spades),
            (Rank::Queen, Suit::Hearts),
            (Rank::Queen, Suit::Diamonds),
            (Rank::Queen, Suit::Clubs),
            (Rank::King, Suit::Spades),
            (Rank::King, Suit::Hearts),
            (Rank::King, Suit::Diamonds),
            (Rank::King, Suit::Clubs),
        ],
        &FACE_CARD_ASSET_LOADER,
    );
    load(
        &mut set,
        [
            IconKind::Accept,
            IconKind::AttackDamage,
            IconKind::AttackRange,
            IconKind::AttackSpeed,
            IconKind::Config,
            IconKind::EnemyBoss,
            IconKind::EnemyNamed,
            IconKind::EnemyNormal,
            IconKind::Gold,
            IconKind::Health,
            IconKind::Invincible,
            IconKind::Item,
            IconKind::Level,
            IconKind::Lock,
            IconKind::MoveSpeed,
            IconKind::Quest,
            IconKind::Refresh,
            IconKind::Reject,
            IconKind::Shield,
            IconKind::Shop,
            IconKind::Speaker,
            IconKind::Suit { suit: Suit::Spades },
            IconKind::Suit { suit: Suit::Hearts },
            IconKind::Suit {
                suit: Suit::Diamonds,
            },
            IconKind::Suit { suit: Suit::Clubs },
            IconKind::Up,
            IconKind::Down,
            IconKind::Card,
            IconKind::New,
            IconKind::Add,
            IconKind::Multiply,
            IconKind::Rarity {
                rarity: Rarity::Common,
            },
            IconKind::Rarity {
                rarity: Rarity::Rare,
            },
            IconKind::Rarity {
                rarity: Rarity::Epic,
            },
            IconKind::Rarity {
                rarity: Rarity::Legendary,
            },
        ],
        &ICON_ASSET_LOADER,
    );
    load(
        &mut set,
        [
            MonsterKind::Mob01,
            MonsterKind::Mob02,
            MonsterKind::Mob03,
            MonsterKind::Mob04,
            MonsterKind::Mob05,
            MonsterKind::Mob06,
            MonsterKind::Mob07,
            MonsterKind::Mob08,
            MonsterKind::Mob09,
            MonsterKind::Mob10,
            MonsterKind::Mob11,
            MonsterKind::Mob12,
            MonsterKind::Mob13,
            MonsterKind::Mob14,
            MonsterKind::Mob15,
            MonsterKind::Named01,
            MonsterKind::Named02,
            MonsterKind::Named03,
            MonsterKind::Named04,
            MonsterKind::Named05,
            MonsterKind::Named06,
            MonsterKind::Named07,
            MonsterKind::Named08,
            MonsterKind::Named09,
            MonsterKind::Named10,
            MonsterKind::Named11,
            MonsterKind::Named12,
            MonsterKind::Named13,
            MonsterKind::Named14,
            MonsterKind::Named15,
            MonsterKind::Named16,
            MonsterKind::Boss01,
            MonsterKind::Boss02,
            MonsterKind::Boss03,
            MonsterKind::Boss04,
            MonsterKind::Boss05,
            MonsterKind::Boss06,
            MonsterKind::Boss07,
            MonsterKind::Boss08,
            MonsterKind::Boss09,
            MonsterKind::Boss10,
            MonsterKind::Boss11,
        ],
        &MONSTER_ASSET_LOADER,
    );
    load(
        &mut set,
        [ParticleKind::MonsterSpirit],
        &PARTICLE_ASSET_LOADER,
    );

    const ALL: &[AnimationKind] = &[
        AnimationKind::Idle1,
        AnimationKind::Idle2,
        AnimationKind::Attack,
    ];
    const IDLE_1: &[AnimationKind] = &[AnimationKind::Idle1];

    load(
        &mut set,
        [
            (TowerKind::Barricade, IDLE_1),
            (TowerKind::High, ALL),
            (TowerKind::OnePair, ALL),
            (TowerKind::TwoPair, ALL),
            (TowerKind::ThreeOfAKind, ALL),
            (TowerKind::Straight, ALL),
            (TowerKind::Flush, ALL),
            (TowerKind::FullHouse, ALL),
            (TowerKind::FourOfAKind, ALL),
            (TowerKind::StraightFlush, ALL),
            (TowerKind::RoyalFlush, ALL),
        ]
        .iter()
        .flat_map(|(tower, animations)| {
            animations
                .iter()
                .map(move |&animation_kind| (*tower, animation_kind))
        }),
        &TOWER_ASSET_LOADER,
    );
    set
}

impl ToResourceLocation for BackgroundKind {
    fn to_resource_location(self) -> ResourceLocation {
        ResourceLocation::bundle(format!("asset/image/background/{}.jpg", self.asset_id()))
    }
}
impl ToResourceLocation for (Rank, Suit) {
    fn to_resource_location(self) -> ResourceLocation {
        let (rank, suit) = self;
        let rank_name = match rank {
            Rank::Jack => "jack",
            Rank::Queen => "queen",
            Rank::King => "king",
            _ => panic!("Invalid face card rank: {rank:?}"),
        };

        let suit_name = match suit {
            Suit::Spades => "spades",
            Suit::Hearts => "hearts",
            Suit::Diamonds => "diamonds",
            Suit::Clubs => "clubs",
        };

        ResourceLocation::bundle(format!("asset/image/face/{suit_name}/{rank_name}.png"))
    }
}
impl ToResourceLocation for IconKind {
    fn to_resource_location(self) -> ResourceLocation {
        ResourceLocation::bundle(format!("asset/image/icon/{}.png", self.asset_id()))
    }
}
impl ToResourceLocation for MonsterKind {
    fn to_resource_location(self) -> ResourceLocation {
        ResourceLocation::bundle(format!("asset/image/monster/{}.png", self.asset_id(),))
    }
}
impl ToResourceLocation for ParticleKind {
    fn to_resource_location(self) -> ResourceLocation {
        ResourceLocation::bundle(self.resource_location())
    }
}
impl ToResourceLocation for (TowerKind, AnimationKind) {
    fn to_resource_location(self) -> ResourceLocation {
        let (tower_kind, animation_kind) = self;
        ResourceLocation::bundle(format!(
            "asset/image/tower/{}/{}.png",
            tower_kind.asset_id(),
            animation_kind.asset_id()
        ))
    }
}

fn load<Key: ToResourceLocation + Copy + Send + Sync + 'static>(
    set: &mut JoinSet<Result<(), (ResourceLocation, anyhow::Error)>>,
    keys: impl IntoIterator<Item = Key>,
    loader: &'static OnceLock<AssetLoader<Key>>,
) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    for key in keys {
        let location = key.to_resource_location();
        let tx = tx.clone();
        set.spawn(async move {
            match load_image_from_resource_location(location.clone()).await {
                Ok(image) => match tx.send((key, image)) {
                    Ok(_) => Ok(()),
                    Err(err) => Err((location, anyhow!(err))),
                },
                Err(err) => Err((location, err)),
            }
        });
    }

    set.spawn(async move {
        let mut map = BTreeMap::new();
        while let Some((key, image)) = rx.recv().await {
            map.insert(key.to_resource_location(), image);
        }
        loader
            .set(AssetLoader {
                inner: map,
                _key: Default::default(),
            })
            .unwrap_or_else(|_| unreachable!("AssetLoader already initialized"));
        Ok(())
    });
}
trait ToResourceLocation {
    fn to_resource_location(self) -> ResourceLocation;
}

struct AssetLoader<Key> {
    inner: BTreeMap<ResourceLocation, namui::Image>,
    _key: PhantomData<Key>,
}

impl<Key: ToResourceLocation> AssetLoader<Key> {
    pub fn get(&self, key: Key) -> Option<Image> {
        self.inner.get(&key.to_resource_location()).cloned()
    }
}
