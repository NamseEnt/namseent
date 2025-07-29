use crate::card::{Rank, Suit};
use namui::skia::load_image_from_resource_location;
use namui::*;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

// Static global face card asset loader
static GLOBAL_FACE_CARD_ASSET_LOADER: OnceLock<Arc<FaceCardAssetLoader>> = OnceLock::new();

pub struct FaceCardAssetLoaderInitializer {}
impl Component for FaceCardAssetLoaderInitializer {
    fn render(self, ctx: &RenderCtx) {
        ctx.effect("Load face card assets", || {
            let face_card_kinds = [
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
            ];

            ctx.spawn(async move {
                let mut images = HashMap::new();

                for (rank, suit) in face_card_kinds {
                    let image_path = get_face_card_image_path(rank, suit);
                    let resource_location = ResourceLocation::bundle(image_path.clone());

                    match load_image_from_resource_location(resource_location.clone()).await {
                        Ok(image) => {
                            images.insert((rank, suit), image);
                        }
                        Err(error) => {
                            eprintln!("Failed to load face card image {image_path}: {error:?}");
                        }
                    }
                }

                let face_card_asset_loader = Arc::new(FaceCardAssetLoader { images });

                GLOBAL_FACE_CARD_ASSET_LOADER
                    .set(face_card_asset_loader)
                    .unwrap();
            });
        });
    }
}

#[derive(Debug)]
pub struct FaceCardAssetLoader {
    images: HashMap<(Rank, Suit), Image>,
}

impl FaceCardAssetLoader {
    pub fn get_image(&self, rank: Rank, suit: Suit) -> Option<&Image> {
        self.images.get(&(rank, suit))
    }
}

pub fn get_face_card_asset_loader() -> Option<Arc<FaceCardAssetLoader>> {
    GLOBAL_FACE_CARD_ASSET_LOADER.get().cloned()
}

fn get_face_card_image_path(rank: Rank, suit: Suit) -> String {
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

    format!("asset/image/face/{suit_name}/{rank_name}.png")
}
