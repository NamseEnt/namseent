pub mod batch;
pub mod combat;
pub mod dense_build;
pub mod entity;
pub mod normalize;
pub mod observation;
pub mod spatial;
pub mod tensor;

pub use batch::{
    PaddedEntityBatch, PolicyBatch, candidate_entity_rows, candidate_rows_for_legal_actions,
};
pub use combat::{
    SplashTriggerFeatureKind, TowerCombatFeatureBundle, TowerCombatFeatureRow,
    TowerSplashFeatureRow, TowerStatusFeatureKind, TowerStatusFeatureRow,
};
pub use dense_build::{
    BUILD_TEMPLATE_CATEGORICAL_FIELDS, BUILD_TEMPLATE_NUMERIC_WIDTH, DenseBuildFeatureBundle,
    DenseBuildFeatureShape, POSITION_FEATURE_WIDTH, PlaceTowerFeatureBundle, PositionFeatureTable,
    RangeCoverageTable, SplashTriggerKind, TemplateSplashFeatureRow,
};
pub use entity::{
    CATEGORICAL_FIELDS, ENTITY_NUMERIC_WIDTH, EntityRow, EntitySet, PooledSet, masked_mean_max,
};
pub use observation::{ENTITY_SET_COUNT, TypedObservation};
pub use spatial::manhattan_route_distance;
