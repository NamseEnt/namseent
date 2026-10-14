//! Variable length effect rows, separate from the frozen legacy inputs.
use super::encoding::{EntityRow, EntitySet};
use super::semantic_candidates::PolicyCandidate;
use crate::environment::{AgentAction, HandItemObservation, Observation, TowerTemplateObservation};
use td_core::{ContentEffectKind as K, EffectSource as S};

fn row(tag: u32, owner: u32, kind: u32, key: u32, values: [f32; 6]) -> EntityRow {
    assert!([tag, owner, kind, key].iter().all(|id| *id < 4096));
    EntityRow::new([tag, owner, kind, key], values.to_vec())
}
fn squash(value: i64, unit: f32) -> f32 {
    let x = value as f32 / unit;
    x.signum() * x.abs().ln_1p()
}
fn content(effect: &td_core::ContentEffectObservation) -> EntityRow {
    let unit = match effect.kind {
        K::Heal | K::Shield | K::MaxHp => 1000.0,
        K::RatioParameter => 1_000_000.0,
        _ => 1.0,
    };
    row(
        3500 + effect.source as u32,
        effect.key_id as u32,
        3520 + effect.kind as u32,
        0,
        [
            squash(effect.value_raw, unit),
            effect.enabled as u8 as f32,
            effect.source_index as f32 / 32.0,
            effect.parameter_index as f32 / 16.0,
            0.0,
            0.0,
        ],
    )
}
fn splashes(
    rows: &mut Vec<EntityRow>,
    template: &TowerTemplateObservation,
    owner: usize,
    xy: [f32; 2],
    hit: &[crate::environment::DamageSplashObservation],
    attack: &[crate::environment::DamageSplashObservation],
) {
    for (trigger, effects) in [(0, hit), (1, attack)] {
        for effect in effects {
            rows.push(row(
                3560,
                template.kind_id as u32,
                3561 + trigger,
                0,
                [
                    squash(effect.radius_raw, 1_000_000.0),
                    squash(effect.damage_pct_raw, 1_000_000.0),
                    owner as f32 / 64.0,
                    xy[0],
                    xy[1],
                    squash(template.effective_damage_raw, 1000.0),
                ],
            ));
        }
    }
}
pub fn state_effects(observation: &Observation) -> EntitySet {
    let mut rows = observation
        .content_effects
        .iter()
        .map(content)
        .collect::<Vec<_>>();
    let mut towers = observation.towers.iter().collect::<Vec<_>>();
    towers.sort_by_key(|tower| tower.id);
    for (owner, tower) in towers.into_iter().enumerate() {
        let xy = [
            tower.left as f32 / observation.map_width.max(1) as f32,
            tower.top as f32 / observation.map_height.max(1) as f32,
        ];
        splashes(
            &mut rows,
            &tower.template,
            owner,
            xy,
            &tower.on_hit_splashes,
            &tower.on_attack_splashes,
        );
        for effect in &tower.status_effects {
            let (kind, value, unit) = match effect.kind {
                crate::environment::TowerStatusEffectObservationKind::DamageMul { mul_raw } => {
                    (3571, mul_raw, 1_000_000.0)
                }
                crate::environment::TowerStatusEffectObservationKind::DamageAdd { add_raw } => {
                    (3572, add_raw, 1000.0)
                }
            };
            rows.push(row(
                3570,
                tower.template.kind_id as u32,
                kind,
                0,
                [
                    squash(value, unit),
                    effect
                        .remaining_ticks
                        .map_or(0.0, |ticks| (ticks as f32 / 600.0).ln_1p()),
                    effect.remaining_ticks.is_none() as u8 as f32,
                    owner as f32 / 64.0,
                    xy[0],
                    xy[1],
                ],
            ));
        }
    }
    for item in &observation.hand {
        if let HandItemObservation::Tower(template) = &item.item {
            splashes(
                &mut rows,
                template,
                item.index,
                [0.0, 0.0],
                &template.on_hit_splashes,
                &template.on_attack_splashes,
            );
        }
    }
    for (slot, template) in observation.extra_tower_card_templates.iter().enumerate() {
        splashes(
            &mut rows,
            template,
            observation.hand.len() + slot,
            [0.0, 0.0],
            &template.on_hit_splashes,
            &template.on_attack_splashes,
        );
    }
    EntitySet::new(rows)
}
pub fn candidate_effects(observation: &Observation, candidate: &PolicyCandidate) -> EntitySet {
    let mut rows = Vec::new();
    let source = match candidate.action {
        AgentAction::SelectTreasure { option_index } => Some((S::TreasureOption, option_index)),
        AgentAction::PurchaseShopItem { slot_index } => Some((S::Shop, slot_index)),
        AgentAction::UseInventoryItem { item_index } => Some((S::Inventory, item_index)),
        _ => None,
    };
    if let Some((source, index)) = source {
        rows.extend(
            observation
                .content_effects
                .iter()
                .filter(|e| e.source == source && e.source_index == index)
                .map(content),
        );
    }
    let template = match &candidate.action {
        AgentAction::BuildTower {
            card_ids,
            hand_slot_index,
            ..
        } => {
            if *hand_slot_index == 0 {
                observation
                    .build_tower_candidates
                    .iter()
                    .find(|c| {
                        let mut a = c.card_ids.clone();
                        let mut b = card_ids.clone();
                        a.sort_unstable();
                        b.sort_unstable();
                        a == b
                    })
                    .map(|c| &c.template)
            } else {
                observation
                    .extra_tower_card_templates
                    .get(hand_slot_index - 1)
            }
        }
        AgentAction::PlaceTower {
            hand_slot_index, ..
        } => observation.hand.iter().find_map(|i| {
            if i.index == *hand_slot_index {
                if let HandItemObservation::Tower(t) = &i.item {
                    Some(t)
                } else {
                    None
                }
            } else {
                None
            }
        }),
        _ => None,
    };
    if let Some(t) = template {
        splashes(
            &mut rows,
            t,
            0,
            [0.0, 0.0],
            &t.on_hit_splashes,
            &t.on_attack_splashes,
        );
    }
    EntitySet::new(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::{
        feature_contract::InputContract,
        model::{ModelConfig, TrainBackend, default_policy_device},
        policy_v2::{KindMode, factorized_log_probs},
        semantic_candidates::encode_decision,
    };
    use burn::module::AutodiffModule;
    use burn::optim::{AdamConfig, GradientsParams, Optimizer};
    fn decision(enabled: bool) -> crate::ml::semantic_candidates::EncodedDecision {
        let mut observation = crate::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            17,
        )
        .snapshot();
        observation.decision_point = crate::environment::DecisionPoint::TreasureSelection;
        observation.treasure_options = vec!["black_white".into(), "apple".into()];
        observation.content_effects = vec![td_core::ContentEffectObservation {
            source: S::TreasureOption,
            source_index: 0,
            key_id: 3,
            kind: K::SameSuit,
            parameter_index: 0,
            value_raw: 1,
            enabled,
        }];
        let candidates = (0..2)
            .map(|index| PolicyCandidate {
                action: AgentAction::SelectTreasure {
                    option_index: index,
                },
                action_id: format!("treasure-{index}"),
                family_rank: None,
                spatial: None,
            })
            .collect::<Vec<_>>();
        encode_decision(&observation, &candidates, vec![true, true])
    }
    #[test]
    fn build_result_tower_effects_align_with_candidate_slots() {
        let config = std::sync::Arc::new(crate::config::GameConfig::default_config());
        let mut environment = crate::environment::GameEnvironment::new(config, 0);
        environment.test_only_seed_extra_tower_cards(1).unwrap();
        let mut observation = environment.snapshot();
        let splash = crate::environment::DamageSplashObservation {
            radius_raw: 2_500_000,
            damage_pct_raw: 250_000,
        };
        observation.build_tower_candidates[0]
            .template
            .on_hit_splashes
            .push(splash.clone());
        observation.extra_tower_card_templates[0]
            .on_attack_splashes
            .push(splash);
        let actions = [
            AgentAction::BuildTower {
                card_ids: observation.build_tower_candidates[0].card_ids.clone(),
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: observation.build_tower_candidates[0].card_ids.clone(),
                hand_slot_index: 1,
                left: 0,
                top: 0,
            },
        ];
        let candidates = actions
            .into_iter()
            .enumerate()
            .map(|(index, action)| PolicyCandidate {
                action_id: format!("build-{index}"),
                action,
                family_rank: None,
                spatial: None,
            })
            .collect::<Vec<_>>();
        let encoded = encode_decision(&observation, &candidates, vec![true, true]);
        assert_eq!(encoded.candidate_effects.len(), 2);
        assert_eq!(encoded.state_extensions.rows.len(), 1);
        assert_eq!(encoded.state_extensions.rows[0].categorical[3], 1024);
        assert!(
            encoded
                .state_effects
                .rows
                .iter()
                .any(|row| { row.categorical[0] == 3560 && row.categorical[2] == 3562 })
        );
        assert!(
            encoded.candidate_effects[0]
                .rows
                .iter()
                .any(|row| row.categorical[0] == 3560 && row.categorical[2] == 3561)
        );
        assert!(
            encoded.candidate_effects[1]
                .rows
                .iter()
                .any(|row| row.categorical[0] == 3560 && row.categorical[2] == 3562)
        );
    }

    #[test]
    fn effect_migration_preserves_outputs_and_actor_and_critic_learn_effect_changes() {
        let device = default_policy_device();
        let config = ModelConfig { hidden_size: 8 };
        let before = decision(true);
        let changed = decision(false);
        assert_eq!(before.typed, changed.typed);
        assert_eq!(before.candidates, changed.candidates);
        assert_ne!(before.state_effects, changed.state_effects);
        assert_ne!(before.candidate_effects, changed.candidate_effects);
        let legacy = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            config,
            KindMode::Learned,
            91,
            &device,
        )
        .unwrap();
        let mut actor = legacy.clone();
        actor.scorer = actor.scorer.with_effect_adapters(config, &device);
        actor.inputs = InputContract::RawEffects;
        let probabilities =
            |actor: &crate::ml::policy_v2::PolicyNet<TrainBackend>,
             d: &crate::ml::semantic_candidates::EncodedDecision| {
                factorized_log_probs(actor, &[d], &device)
                    .joint
                    .into_data()
                    .to_vec::<f32>()
                    .unwrap()
            };
        assert_eq!(
            probabilities(&legacy, &before),
            probabilities(&actor, &changed)
        );
        let critic = legacy.scorer.clone();
        let mut effects_critic = critic.clone().with_effect_adapters(config, &device);
        use crate::ml::semantic_ppo::{CriticInputs, critic_values};
        assert_eq!(
            critic_values(&critic, &[&before], CriticInputs::SquashAll, &device)
                .into_data()
                .to_vec::<f32>()
                .unwrap(),
            critic_values(
                &effects_critic,
                &[&changed],
                CriticInputs::SquashAllEffects,
                &device
            )
            .into_data()
            .to_vec::<f32>()
            .unwrap()
        );
        let mut optimizer = AdamConfig::new().init();
        let mut critic_optimizer = AdamConfig::new().init();
        for _ in 0..3 {
            let loss = -factorized_log_probs(&actor, &[&changed], &device)
                .joint
                .slice([0..1, 1..2])
                .mean();
            let gradients = GradientsParams::from_grads(loss.backward(), &actor);
            actor = optimizer.step(0.01, actor, gradients);
            let loss = (critic_values(
                &effects_critic,
                &[&changed],
                CriticInputs::SquashAllEffects,
                &device,
            ) - 1.0)
                .powf_scalar(2.0)
                .mean();
            let gradients = GradientsParams::from_grads(loss.backward(), &effects_critic);
            effects_critic = critic_optimizer.step(0.01, effects_critic, gradients);
        }
        assert_ne!(
            probabilities(&actor, &before),
            probabilities(&actor, &changed)
        );
        let values = critic_values(
            &effects_critic,
            &[&before, &changed],
            CriticInputs::SquashAllEffects,
            &device,
        )
        .into_data()
        .to_vec::<f32>()
        .unwrap();
        assert_ne!(values[0], values[1]);
        let bytes = crate::ml::policy_v2::module_to_bytes(actor.clone().valid()).unwrap();
        let restored = crate::ml::policy_v2::policy_from_bytes::<TrainBackend>(
            config,
            KindMode::Learned,
            bytes,
            &device,
        )
        .unwrap()
        .with_inputs(InputContract::RawEffects);
        assert_eq!(
            probabilities(&actor, &changed),
            probabilities(&restored, &changed)
        );
    }

    #[test]
    fn schema_extensions_preserve_then_change_actor_and_critic_outputs_after_learning() {
        use crate::ml::encoding::{EntityRow, EntitySet};
        use crate::ml::semantic_ppo::{CriticInputs, critic_values};

        let device = default_policy_device();
        let config = ModelConfig { hidden_size: 8 };
        let base = decision(true);
        let mut changed = base.clone();
        changed.state_extensions = EntitySet::new(vec![EntityRow::new(
            [24, 3, 11, 1024],
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 1.0],
        )]);
        changed.candidate_extensions = vec![
            EntitySet::default(),
            EntitySet::new(vec![EntityRow::new(
                [2, 9, 4, 1],
                vec![0.7, 1.0, 0.0, 0.0, 0.0, 0.0],
            )]),
        ];
        assert_eq!(base.typed, changed.typed);
        assert_eq!(base.candidates, changed.candidates);
        assert_ne!(base.state_extensions, changed.state_extensions);
        assert_ne!(base.candidate_extensions, changed.candidate_extensions);

        let mut actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            config,
            KindMode::Learned,
            711,
            &device,
        )
        .unwrap();
        actor.scorer = actor.scorer.with_migration_adapters(config, &device);
        let probabilities = |model: &crate::ml::policy_v2::PolicyNet<TrainBackend>, d: &_| {
            factorized_log_probs(model, &[d], &device)
                .joint
                .into_data()
                .to_vec::<f32>()
                .unwrap()
        };
        let initial_base = probabilities(&actor, &base);
        let initial_changed = probabilities(&actor, &changed);
        assert_eq!(initial_base, initial_changed);

        let critic_source =
            crate::ml::semantic_bc::seeded_materialized_model(config, 712, &device).unwrap();
        let mut critic = critic_source.with_migration_adapters(config, &device);
        assert_eq!(
            critic_values(&critic, &[&base], CriticInputs::SquashAll, &device)
                .into_data()
                .to_vec::<f32>()
                .unwrap(),
            critic_values(&critic, &[&changed], CriticInputs::SquashAll, &device)
                .into_data()
                .to_vec::<f32>()
                .unwrap()
        );

        let mut actor_optimizer = AdamConfig::new().init();
        let mut critic_optimizer = AdamConfig::new().init();
        for _ in 0..4 {
            let loss = -factorized_log_probs(&actor, &[&changed], &device)
                .joint
                .slice([0..1, 1..2])
                .mean();
            let gradients = GradientsParams::from_grads(loss.backward(), &actor);
            assert!(
                !gradients.is_empty(),
                "actor extension path receives gradients"
            );
            actor = actor_optimizer.step(0.01, actor, gradients);

            let loss = (critic_values(&critic, &[&changed], CriticInputs::SquashAll, &device)
                - 1.0)
                .powf_scalar(2.0)
                .mean();
            let gradients = GradientsParams::from_grads(loss.backward(), &critic);
            assert!(
                !gradients.is_empty(),
                "critic extension path receives gradients"
            );
            critic = critic_optimizer.step(0.01, critic, gradients);
        }
        assert_ne!(
            probabilities(&actor, &base),
            probabilities(&actor, &changed)
        );
        let values = critic_values(
            &critic,
            &[&base, &changed],
            CriticInputs::SquashAll,
            &device,
        )
        .into_data()
        .to_vec::<f32>()
        .unwrap();
        assert_ne!(values[0], values[1]);
    }

    #[test]
    fn actual_learned_effect_checkpoint_preserves_outputs_with_schema_extensions() {
        use crate::ml::{
            model::InferenceBackend,
            semantic_bc::load_model_file,
            semantic_candidates::{CandidateMode, policy_candidates_with},
            semantic_ppo::{PpoRunMetadata, critic_values, load_transfer_actor_as},
        };

        let checkpoint = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "artifacts/retraining/effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004",
        );
        if !checkpoint.exists() {
            return;
        }
        let device = default_policy_device();
        let mut environment = crate::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            73_091,
        );
        environment.test_only_seed_extra_tower_cards(2).unwrap();
        environment
            .test_only_seed_owned_upgrade(td_core::UpgradeKind::BlackWhite)
            .unwrap();
        let set = policy_candidates_with(&environment, CandidateMode::Top8).unwrap();
        let mut observation = set.observation.clone();
        let mut polish = 1_000;
        for hand_item in &mut observation.hand {
            if let crate::environment::HandItemObservation::Card(card) = &mut hand_item.item {
                card.polish_pct_raw = polish;
                polish += 7_000;
            }
        }
        let encoded = encode_decision(
            &observation,
            &set.candidates,
            vec![true; set.candidates.len()],
        );
        assert!(!encoded.state_extensions.is_empty());
        assert!(
            encoded
                .candidate_extensions
                .iter()
                .any(|set| !set.is_empty())
        );
        assert!(
            !encoded.state_effects.is_empty(),
            "fixture exercises the learned effect adapter"
        );

        let loaded = load_transfer_actor_as::<InferenceBackend>(&checkpoint, &device).unwrap();
        assert!(loaded.inputs.has_effects());
        let migrated = loaded.clone();
        let mut legacy = loaded.clone();
        legacy.scorer = legacy.scorer.clone().without_migration_adapters();
        let before = factorized_log_probs(&legacy, &[&encoded], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let after = factorized_log_probs(&migrated, &[&encoded], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let actor_max_abs_error = before
            .iter()
            .zip(&after)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);
        assert!(
            actor_max_abs_error <= 1.0e-6,
            "actor max error {actor_max_abs_error}"
        );

        let run: PpoRunMetadata = serde_json::from_slice(
            &std::fs::read(checkpoint.parent().unwrap().join("ppo.json")).unwrap(),
        )
        .unwrap();
        let critic = load_model_file::<InferenceBackend>(
            run.model_config,
            &checkpoint.join("critic.bin"),
            &device,
        )
        .unwrap();
        let legacy_critic = critic.clone().without_migration_adapters();
        let inputs = run.critic_inputs;
        let before = critic_values(&legacy_critic, &[&encoded], inputs, &device)
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let after = critic_values(&critic, &[&encoded], inputs, &device)
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let critic_max_abs_error = (before[0] - after[0]).abs();
        assert!(
            critic_max_abs_error <= 1.0e-6,
            "critic max error {critic_max_abs_error}"
        );
        eprintln!(
            "schema6/candidate1 trained-effect transfer: same Top8 candidates, extra_towers=2, varied_card_polish=true, actor_max_abs={actor_max_abs_error:.9}, critic_max_abs={critic_max_abs_error:.9}"
        );
    }

    #[test]
    fn actual_source_converted_checkpoint_preserves_outputs_with_schema_extensions() {
        use crate::ml::{
            model::InferenceBackend,
            semantic_bc::load_model_file,
            semantic_candidates::{CandidateMode, policy_candidates_with},
            semantic_ppo::{PpoRunMetadata, critic_values, load_transfer_actor_as},
        };

        let checkpoint = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "artifacts/retraining/effect-adaptation-isolated-20261011/source-converted/iter-0000",
        );
        if !checkpoint.exists() {
            return;
        }
        let device = default_policy_device();
        let mut environment = crate::environment::GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            73_092,
        );
        environment.test_only_seed_extra_tower_cards(2).unwrap();
        let set = policy_candidates_with(&environment, CandidateMode::Top8).unwrap();
        let mut observation = set.observation.clone();
        for (index, hand_item) in observation.hand.iter_mut().enumerate() {
            if let crate::environment::HandItemObservation::Card(card) = &mut hand_item.item {
                card.polish_pct_raw = 2_000 + index as i64 * 5_000;
            }
        }
        let encoded = encode_decision(
            &observation,
            &set.candidates,
            vec![true; set.candidates.len()],
        );
        assert!(!encoded.state_extensions.is_empty());
        assert!(
            encoded
                .candidate_extensions
                .iter()
                .any(|set| !set.is_empty())
        );

        let loaded = load_transfer_actor_as::<InferenceBackend>(&checkpoint, &device).unwrap();
        let mut legacy = loaded.clone();
        legacy.scorer = legacy.scorer.clone().without_migration_adapters();
        let old_logits = factorized_log_probs(&legacy, &[&encoded], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let new_logits = factorized_log_probs(&loaded, &[&encoded], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let actor_max_abs_error = old_logits
            .iter()
            .zip(&new_logits)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);

        let run: PpoRunMetadata = serde_json::from_slice(
            &std::fs::read(checkpoint.parent().unwrap().join("ppo.json")).unwrap(),
        )
        .unwrap();
        let critic = load_model_file::<InferenceBackend>(
            run.model_config,
            &checkpoint.join("critic.bin"),
            &device,
        )
        .unwrap();
        let old_critic = critic.clone().without_migration_adapters();
        let old_value = critic_values(&old_critic, &[&encoded], run.critic_inputs, &device)
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let new_value = critic_values(&critic, &[&encoded], run.critic_inputs, &device)
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        let critic_max_abs_error = (old_value[0] - new_value[0]).abs();
        assert!(actor_max_abs_error <= 1.0e-6);
        assert!(critic_max_abs_error <= 1.0e-6);
        eprintln!(
            "schema6/candidate1 source-converted checkpoint: same Top8 candidates, extra_towers=2, varied_card_polish=true, actor_max_abs={actor_max_abs_error:.9}, critic_max_abs={critic_max_abs_error:.9}"
        );
    }
    #[test]
    fn treasure_preparation_keeps_other_action_families_unchanged() {
        let device = default_policy_device();
        let config = ModelConfig { hidden_size: 8 };
        let mut actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
            config,
            KindMode::Learned,
            193,
            &device,
        )
        .unwrap();
        actor.inputs = InputContract::RawEffects;
        actor.scorer = actor.scorer.with_effect_adapters(config, &device);
        let treasure = decision(true);
        let probabilities =
            |actor: &crate::ml::policy_v2::PolicyNet<TrainBackend>,
             d: &crate::ml::semantic_candidates::EncodedDecision| {
                factorized_log_probs(actor, &[d], &device)
                    .joint
                    .into_data()
                    .to_vec::<f32>()
                    .unwrap()
            };
        let other_decisions = [
            crate::environment::ActionKind::UseInventoryItem,
            crate::environment::ActionKind::PurchaseShopItem,
            crate::environment::ActionKind::BuildTower,
        ]
        .map(|kind| {
            let mut d = treasure.clone();
            d.families.fill(kind.index() as u8);
            d
        });
        let before_other = other_decisions.each_ref().map(|d| probabilities(&actor, d));
        let before_treasure = probabilities(&actor, &treasure);
        let mut layer = actor.scorer.candidate_effect_adapter().unwrap();
        let mut optimizer = AdamConfig::new().init();
        for _ in 0..3 {
            actor.scorer = actor
                .scorer
                .clone()
                .with_candidate_effect_adapter(layer.clone());
            let loss = factorized_log_probs(&actor, &[&treasure], &device)
                .joint
                .slice([0..1, 0..1])
                .mean()
                .neg();
            let gradients = GradientsParams::from_grads(loss.backward(), &layer);
            layer = optimizer.step(0.01, layer, gradients);
        }
        actor.scorer = actor.scorer.with_candidate_effect_adapter(layer);
        assert_ne!(before_treasure, probabilities(&actor, &treasure));
        assert_eq!(
            before_other,
            other_decisions.each_ref().map(|d| probabilities(&actor, d))
        );
    }
}
