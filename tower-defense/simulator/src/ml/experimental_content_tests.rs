use crate::environment::AgentAction;
use crate::ml::{
    model::{ModelConfig, TrainBackend, default_policy_device},
    policy_v2::{KindMode, factorized_log_probs, module_to_bytes},
    semantic_candidates::{PolicyCandidate, encode_decision},
};
use burn::{
    module::{Module, ModuleVisitor, Param, ParamId},
    optim::{AdamConfig, GradientsParams, Optimizer},
    tensor::Tensor,
};
use td_core::{ContentEffectKind as K, EffectSource as S};

fn entity_embedding_snapshot<B: burn::tensor::backend::Backend>(
    model: &crate::ml::policy_v2::PolicyNet<B>,
) -> (ParamId, Vec<f32>) {
    struct EmbeddingVisitor<B: burn::tensor::backend::Backend> {
        found: Option<(ParamId, Vec<f32>)>,
        backend: std::marker::PhantomData<B>,
    }

    impl<B: burn::tensor::backend::Backend> ModuleVisitor<B> for EmbeddingVisitor<B> {
        fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<B, D>>) {
            if parameter.val().shape().dims::<D>().as_slice() == [4096, 8] {
                self.found = Some((
                    parameter.id,
                    parameter
                        .val()
                        .clone()
                        .into_data()
                        .to_vec::<f32>()
                        .expect("embedding parameter is f32"),
                ));
            }
        }
    }

    let mut visitor = EmbeddingVisitor {
        found: None,
        backend: std::marker::PhantomData,
    };
    model.visit(&mut visitor);
    visitor
        .found
        .expect("actor has one 4096x8 entity embedding")
}

#[test]
fn card_polisher_shop_candidate_id_and_gradient_reach_actor_inputs() {
    use crate::environment::{DecisionPoint, ShopSlotObservation};

    let device = default_policy_device();
    let mut observation = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        16_041,
    )
    .snapshot();
    observation.decision_point = DecisionPoint::Shop;
    observation.shop = vec![
        ShopSlotObservation {
            index: 0,
            purchased: false,
            kind: "card_service".into(),
            kind_id: 3,
            key: "card_polisher".into(),
            key_id: 17,
            cost: 45,
        },
        ShopSlotObservation {
            index: 1,
            purchased: false,
            kind: "card_service".into(),
            kind_id: 3,
            key: "long_sword".into(),
            key_id: 1,
            cost: 45,
        },
    ];
    let candidates = (0..2)
        .map(|slot_index| PolicyCandidate {
            action: AgentAction::PurchaseShopItem { slot_index },
            action_id: format!("purchase:{slot_index}"),
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let encoded = encode_decision(&observation, &candidates, vec![true, true]);
    assert_eq!(td_core::CardServiceKind::CardPolisher.raw(), 16);
    assert_eq!(
        crate::ml::vocabulary::card_service_key_id("card_polisher"),
        17
    );
    assert!(
        encoded.candidates[0]
            .rows
            .iter()
            .any(|row| row.categorical == [3005, 4, 18, 0])
    );

    let mut actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
        ModelConfig { hidden_size: 8 },
        KindMode::Learned,
        16_041,
        &device,
    )
    .unwrap();
    let (embedding_id, before) = entity_embedding_snapshot(&actor);
    let log_probs = factorized_log_probs(&actor, &[&encoded], &device).joint;
    let loss = (log_probs.clone().exp() * log_probs).sum();
    let gradients = GradientsParams::from_grads(loss.backward(), &actor);
    let gradient = gradients
        .get::<<TrainBackend as burn::tensor::backend::AutodiffBackend>::InnerBackend, 2>(
            embedding_id,
        )
        .unwrap()
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let row_gradient_l2 = gradient[18 * 8..19 * 8]
        .iter()
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();
    assert!(row_gradient_l2 > 0.0);
    let mut optimizer = AdamConfig::new().init();
    actor = optimizer.step(1e-3, actor, gradients);
    let (_, after) = entity_embedding_snapshot(&actor);
    assert_ne!(before[18 * 8..19 * 8], after[18 * 8..19 * 8]);
    println!(
        "CARD_POLISHER_ACTOR_INPUT candidate_id=17 candidate_row=18 embedding_gradient_l2={row_gradient_l2} row_changed=true"
    );
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn final_card_polisher_checkpoint_infers_on_shop_and_target_selection() {
    use crate::environment::{CardServiceObservation, DecisionPoint, ShopSlotObservation};

    let checkpoint =
        std::env::var("CARD_POLISHER_CHECKPOINT").expect("set CARD_POLISHER_CHECKPOINT");
    let checkpoint = std::path::Path::new(&checkpoint);
    let device = default_policy_device();
    let (actor, mode) = crate::ml::semantic_ppo::load_ppo_policy(checkpoint, &device).unwrap();
    let run_metadata: crate::ml::semantic_ppo::PpoRunMetadata = serde_json::from_slice(
        &std::fs::read(checkpoint.parent().unwrap().join("ppo.json")).unwrap(),
    )
    .unwrap();
    let contract = run_metadata.environment.as_ref().unwrap();
    contract
        .check_run(&contract.game_config.clone())
        .expect("strict current-source checkpoint contract");
    let source_checkpoint = std::env::var("CARD_POLISHER_SOURCE_CHECKPOINT")
        .expect("set CARD_POLISHER_SOURCE_CHECKPOINT");
    let source_actor = crate::ml::semantic_ppo::load_transfer_actor_as::<
        crate::ml::model::InferenceBackend,
    >(std::path::Path::new(&source_checkpoint), &device)
    .unwrap();
    let (_, source_embedding) = entity_embedding_snapshot(&source_actor);
    let (_, final_embedding) = entity_embedding_snapshot(&actor);
    assert_ne!(
        source_embedding[18 * 8..19 * 8],
        final_embedding[18 * 8..19 * 8],
        "saved/reloaded CardPolisher entity row changed during PPO"
    );
    let row18_delta_l2 = source_embedding[18 * 8..19 * 8]
        .iter()
        .zip(&final_embedding[18 * 8..19 * 8])
        .map(|(before, after)| (after - before) * (after - before))
        .sum::<f32>()
        .sqrt();
    let metadata: crate::ml::semantic_ppo::PpoActorFile =
        serde_json::from_slice(&std::fs::read(checkpoint.join("ppo-actor.json")).unwrap()).unwrap();
    let catalog = &metadata.environment.as_ref().unwrap().content_catalog;
    assert_eq!(metadata.iteration, 2);
    assert_eq!(catalog["card_services"].len(), 17);
    assert_eq!(
        catalog["card_services"][..16],
        td_core::CardServiceKind::ALL[..16]
            .iter()
            .map(|kind| kind.key().to_string())
            .collect::<Vec<_>>()
    );
    assert_eq!(catalog["card_services"][16], "card_polisher");
    assert_eq!(catalog["items"][11], "ShieldBerry");
    assert_eq!(catalog["treasures"][37], "TrainingMelon");

    let mut shop = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        91,
    )
    .snapshot();
    shop.decision_point = DecisionPoint::Shop;
    shop.shop = vec![
        ShopSlotObservation {
            index: 0,
            purchased: false,
            kind: "card_service".into(),
            kind_id: 3,
            key: "card_polisher".into(),
            key_id: 17,
            cost: 45,
        },
        ShopSlotObservation {
            index: 1,
            purchased: false,
            kind: "card_service".into(),
            kind_id: 3,
            key: "long_sword".into(),
            key_id: 1,
            cost: 45,
        },
    ];
    let shop_candidates = (0..2)
        .map(|slot_index| PolicyCandidate {
            action: AgentAction::PurchaseShopItem { slot_index },
            action_id: format!("purchase:{slot_index}"),
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let shop_encoded = encode_decision(&shop, &shop_candidates, vec![true, true]);
    assert!(
        shop_encoded.candidates[0]
            .rows
            .iter()
            .any(|row| row.categorical == [3005, 4, 18, 0])
    );
    let shop_logits = factorized_log_probs(&actor, &[&shop_encoded], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    assert_eq!(shop_logits.len(), 2);
    assert!(shop_logits.iter().all(|value| value.is_finite()));

    let mut selection = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        92,
    )
    .snapshot();
    selection.decision_point = DecisionPoint::CardServiceSelection;
    selection.card_service = Some(CardServiceObservation {
        key: "card_polisher".into(),
        key_id: 17,
        polish_delta_raw: Some(td_core::CardServiceKind::CardPolisher.polish_delta_raw()),
        suit_change_raw: None,
        target_filter: Some(td_core::CardSelectionFilterState::Any),
        current_step: 0,
        step_count: 1,
        required_count: 1,
        selected_card_indices: vec![],
        candidate_card_indices: vec![0, 1],
    });
    let selection_candidates = (0..2)
        .map(|card_index| PolicyCandidate {
            action: AgentAction::SelectCardServiceCard { card_index },
            action_id: format!("select:{card_index}"),
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let selection_encoded = encode_decision(&selection, &selection_candidates, vec![true, true]);
    let selection_logits = factorized_log_probs(&actor, &[&selection_encoded], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    assert_eq!(selection_logits.len(), 2);
    assert!(selection_logits.iter().all(|value| value.is_finite()));
    println!(
        "CARD_POLISHER_CHECKPOINT_INFERENCE iteration=2 mode={mode:?} shop_logits={shop_logits:?} selection_logits={selection_logits:?} embedding_row18_source_delta_l2={row18_delta_l2}"
    );
}

#[test]
fn core_card_service_context_is_present_for_selection_and_confirmation() {
    use crate::environment::{AgentAction as A, GameEnvironment};
    use crate::ml::semantic_candidates::{PolicyCandidate, encode_decision};

    fn purchased(kind: td_core::CardServiceKind) -> GameEnvironment {
        let mut env = GameEnvironment::new(
            std::sync::Arc::new(crate::config::GameConfig::default_config()),
            781,
        );
        let slot = env.add_card_service_shop_slot_for_test(kind.raw());
        env.step(A::PurchaseShopItem { slot_index: slot }).unwrap();
        env
    }
    fn encoded(env: &GameEnvironment) -> crate::ml::semantic_candidates::EncodedDecision {
        let candidates = env
            .legal_actions()
            .into_iter()
            .map(|legal| PolicyCandidate {
                action_id: legal.action.action_id(),
                action: legal.action,
                family_rank: None,
                spatial: None,
            })
            .collect::<Vec<_>>();
        encode_decision(&env.snapshot(), &candidates, vec![true; candidates.len()])
    }

    let mut polish = purchased(td_core::CardServiceKind::CardPolisher);
    let mut sword = purchased(td_core::CardServiceKind::LongSword);
    assert_eq!(
        polish.snapshot().deck.all_cards,
        sword.snapshot().deck.all_cards
    );
    let polish_select = encoded(&polish);
    let sword_select = encoded(&sword);
    // Every input consumed by the legacy actor/critic path is identical for
    // these two Core states. Only the isolated new service residual differs.
    assert_eq!(polish_select.global_features, sword_select.global_features);
    for (index, (polish_rows, sword_rows)) in polish_select
        .typed
        .sets
        .iter()
        .zip(&sword_select.typed.sets)
        .enumerate()
    {
        if index != crate::ml::encoding::observation::SHOP {
            assert_eq!(polish_rows, sword_rows, "legacy typed set {index}");
        }
    }
    assert_ne!(
        polish_select.typed.sets[crate::ml::encoding::observation::SHOP],
        sword_select.typed.sets[crate::ml::encoding::observation::SHOP]
    );
    assert_eq!(polish_select.state_effects, sword_select.state_effects);
    assert_eq!(
        polish_select.candidate_effects,
        sword_select.candidate_effects
    );
    assert_eq!(
        polish_select.state_extensions,
        sword_select.state_extensions
    );
    assert_eq!(
        polish_select.candidate_extensions,
        sword_select.candidate_extensions
    );
    assert_eq!(polish_select.candidates, sword_select.candidates);
    assert_eq!(polish_select.legal_mask, sword_select.legal_mask);
    assert_eq!(polish_select.families, sword_select.families);
    assert_eq!(polish_select.projection, sword_select.projection);
    assert_ne!(
        polish_select.card_service_context,
        sword_select.card_service_context
    );
    assert_ne!(
        polish_select.candidate_card_service_effects,
        sword_select.candidate_card_service_effects
    );
    assert_eq!(
        polish_select.card_service_context.rows[0].categorical[1],
        td_core::CardServiceKind::CardPolisher.id() as u32
    );
    assert_eq!(polish_select.card_service_context.rows[0].numeric[4], 3.0);
    assert_eq!(sword_select.card_service_context.rows[0].numeric[4], 2.0);
    assert_eq!(polish_select.card_service_context.rows[1].categorical[2], 1);
    assert_eq!(sword_select.card_service_context.rows[1].categorical[2], 2);
    assert_eq!(polish_select.card_service_context.rows[1].numeric[0], 3.0);
    assert_eq!(
        polish_select.candidate_card_service_effects[0].rows[0].numeric[5], 1.0,
        "candidate row explicitly marks supported effect semantics"
    );
    let eraser = purchased(td_core::CardServiceKind::Eraser);
    let eraser_select = encoded(&eraser);
    assert_eq!(eraser_select.card_service_context.rows[1].categorical[2], 0);
    assert_eq!(
        eraser_select.candidate_card_service_effects[0].rows[0].numeric[5], 0.0,
        "unsupported service semantics stay explicitly unsupported"
    );
    assert_eq!(
        polish_select.card_service_context.rows[0].numeric[5],
        sword_select.card_service_context.rows[0].numeric[5],
        "both services have the same Any target condition"
    );
    let polished_card_id = polish.snapshot().deck.all_cards[0].id;
    let sword_card_id = sword.snapshot().deck.all_cards[0].id;
    let predicted_polish = polish_select.candidate_card_service_effects[0].rows[0].numeric[1];
    let polish_before = polish_select.candidate_card_service_effects[0].rows[0].numeric[0];
    assert_eq!(predicted_polish - polish_before, 3.0);
    let predicted_sword = sword_select.candidate_card_service_effects[0].rows[0].numeric[1];

    for env in [&mut polish, &mut sword] {
        env.step(A::SelectCardServiceCard { card_index: 0 })
            .unwrap();
    }
    let polish_confirm = encoded(&polish);
    let sword_confirm = encoded(&sword);
    assert!(
        polish
            .legal_actions()
            .iter()
            .any(|legal| matches!(legal.action, A::ConfirmCardServiceSelection))
    );
    polish.step(A::ConfirmCardServiceSelection).unwrap();
    sword.step(A::ConfirmCardServiceSelection).unwrap();
    assert_eq!(
        polish
            .snapshot()
            .deck
            .all_cards
            .iter()
            .find(|card| card.id == polished_card_id)
            .unwrap()
            .polish_pct_raw as f32
            / 1_000_000.0,
        predicted_polish,
        "candidate projection must equal authoritative Core result"
    );
    let sword_after_confirm = sword.snapshot();
    let sword_result = sword_after_confirm
        .deck
        .all_cards
        .iter()
        .find(|card| card.id == sword_card_id)
        .unwrap();
    let (sword_suit, sword_delta) = td_core::CardServiceKind::LongSword
        .card_enhancement()
        .unwrap();
    assert_eq!(
        sword_result.polish_pct_raw as f32 / 1_000_000.0,
        predicted_sword,
        "existing enhancement candidate result matches Core polish"
    );
    assert_eq!(
        crate::ml::vocabulary::suit_id(&sword_result.suit) as u8,
        sword_suit.unwrap().raw() + 1
    );
    assert_eq!(sword_delta, 2_000_000);
    assert_ne!(
        polish_confirm.card_service_context,
        sword_confirm.card_service_context
    );
    assert_eq!(
        polish_confirm.global_features,
        sword_confirm.global_features
    );
    for (index, (polish_rows, sword_rows)) in polish_confirm
        .typed
        .sets
        .iter()
        .zip(&sword_confirm.typed.sets)
        .enumerate()
    {
        if index != crate::ml::encoding::observation::SHOP {
            assert_eq!(polish_rows, sword_rows, "confirm legacy typed set {index}");
        }
    }
    assert_eq!(polish_confirm.candidates, sword_confirm.candidates);
    assert_eq!(polish_confirm.legal_mask, sword_confirm.legal_mask);
    assert_ne!(
        polish_confirm.typed.sets[crate::ml::encoding::observation::SHOP],
        sword_confirm.typed.sets[crate::ml::encoding::observation::SHOP]
    );

    let device = default_policy_device();
    let model_config = crate::ml::model::ModelConfig::default();
    let actor = crate::ml::policy_v2::PolicyNet::<crate::ml::model::InferenceBackend>::new(
        model_config,
        crate::ml::policy_v2::KindMode::Learned,
        &device,
    );
    let mut actor = actor;
    actor.scorer = actor
        .scorer
        .with_card_service_adapters(model_config, &device);
    for decision in [
        &polish_select,
        &sword_select,
        &polish_confirm,
        &sword_confirm,
    ] {
        let output = crate::ml::policy_v2::factorized_log_probs(&actor, &[decision], &device);
        assert!(
            output
                .joint
                .into_data()
                .to_vec::<f32>()
                .unwrap()
                .iter()
                .all(|x| x.is_finite())
        );
    }
    let critic = crate::ml::model::DeepSetsActorCritic::<crate::ml::model::InferenceBackend>::new(
        model_config,
        &device,
    )
    .with_card_service_adapters(model_config, &device);
    for decision in [
        &polish_select,
        &sword_select,
        &polish_confirm,
        &sword_confirm,
    ] {
        let value = crate::ml::semantic_ppo::critic_values(
            &critic,
            &[decision],
            crate::ml::semantic_ppo::CriticInputs::SquashAll,
            &device,
        );
        assert!(value.into_data().to_vec::<f32>().unwrap()[0].is_finite());
    }
    println!(
        "CARD_SERVICE_CONTEXT_SELECTION_AND_CONFIRMATION key_ids={:?},{:?} polisher_delta=3000000 raw_actor_input_diff=true",
        polish_select.card_service_context.rows[0].categorical[1],
        sword_select.card_service_context.rows[0].categorical[1]
    );
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn retained_card_polisher_iteration_two_contract_is_explicitly_migratable() {
    let checkpoint = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("artifacts/retraining/card-polisher-20261014/run/iter-0002");
    let actor_file: crate::ml::semantic_ppo::PpoActorFile =
        serde_json::from_slice(&std::fs::read(checkpoint.join("ppo-actor.json")).unwrap()).unwrap();
    let source = actor_file.environment.as_ref().unwrap();
    assert_eq!(source.ml.observation_schema_version, 10);
    assert_eq!(source.entity_encoder_version, 7);
    assert_eq!(source.candidate_encoder_version, 2);
    source
        .check_transfer_encoding()
        .expect("exact source hashes are allowlisted for explicit side-path migration");
    let device = default_policy_device();
    let actor =
        crate::ml::semantic_ppo::load_transfer_actor_as::<crate::ml::model::InferenceBackend>(
            &checkpoint,
            &device,
        )
        .expect("load iteration 2 model and materialize zero card-service residual");
    assert!(actor.scorer.has_effect_adapters());
    assert!(actor.scorer.has_migration_adapters());
    assert!(actor.scorer.has_card_service_adapters());
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn zero_service_residual_preserves_source_actor_and_critic_outputs() {
    use crate::environment::{AgentAction as A, GameEnvironment};
    use crate::ml::semantic_candidates::{PolicyCandidate, encode_decision};

    let checkpoint = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("artifacts/retraining/card-polisher-20261014/run/iter-0002");
    let device = default_policy_device();
    let with_adapters = crate::ml::semantic_ppo::load_transfer_actor_as::<
        crate::ml::model::InferenceBackend,
    >(&checkpoint, &device)
    .unwrap();
    let mut source_path = with_adapters.clone();
    source_path.scorer = source_path.scorer.without_card_service_adapters();
    let critic_path = checkpoint.join("critic.bin");
    let with_critic =
        crate::ml::semantic_bc::load_model_file::<crate::ml::model::InferenceBackend>(
            crate::ml::model::ModelConfig::default(),
            &critic_path,
            &device,
        )
        .unwrap();
    let source_critic = with_critic.clone().without_card_service_adapters();
    let mut env = GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        981,
    );
    let slot =
        env.add_card_service_shop_slot_for_test(td_core::CardServiceKind::CardPolisher.raw());
    let purchase_candidates = env
        .legal_actions()
        .into_iter()
        .map(|legal| PolicyCandidate {
            action_id: legal.action.action_id(),
            action: legal.action,
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let purchase = encode_decision(
        &env.snapshot(),
        &purchase_candidates,
        vec![true; purchase_candidates.len()],
    );
    let before_actor =
        crate::ml::policy_v2::factorized_log_probs(&source_path, &[&purchase], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
    let after_actor =
        crate::ml::policy_v2::factorized_log_probs(&with_adapters, &[&purchase], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
    let actor_error = before_actor
        .iter()
        .zip(&after_actor)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    let critic_error = (crate::ml::semantic_ppo::critic_values(
        &source_critic,
        &[&purchase],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    ) - crate::ml::semantic_ppo::critic_values(
        &with_critic,
        &[&purchase],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    ))
    .abs()
    .max()
    .into_data()
    .to_vec::<f32>()
    .unwrap()[0];
    assert!(actor_error <= 1.0e-6, "actor output error={actor_error}");
    assert!(critic_error <= 1.0e-6, "critic output error={critic_error}");
    env.step(A::PurchaseShopItem { slot_index: slot }).unwrap();
    let candidates = env
        .legal_actions()
        .into_iter()
        .map(|legal| PolicyCandidate {
            action_id: legal.action.action_id(),
            action: legal.action,
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let target = encode_decision(&env.snapshot(), &candidates, vec![true; candidates.len()]);
    let before = crate::ml::policy_v2::factorized_log_probs(&source_path, &[&target], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let after = crate::ml::policy_v2::factorized_log_probs(&with_adapters, &[&target], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let target_error = before
        .iter()
        .zip(&after)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        target_error <= 1.0e-6,
        "target-selection actor output error={target_error}"
    );
    let value_before = crate::ml::semantic_ppo::critic_values(
        &source_critic,
        &[&target],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    )
    .into_data()
    .to_vec::<f32>()
    .unwrap()[0];
    let value_after = crate::ml::semantic_ppo::critic_values(
        &with_critic,
        &[&target],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    )
    .into_data()
    .to_vec::<f32>()
    .unwrap()[0];
    assert!((value_before - value_after).abs() <= 1.0e-6);
    let general = GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        982,
    );
    let construction_policy = crate::ml::semantic_candidates::policy_candidates(&general).unwrap();
    let construction_candidates = construction_policy.candidates;
    assert!(
        construction_candidates
            .iter()
            .any(|candidate| matches!(candidate.action, A::BuildTower { .. }))
    );
    let construction = encode_decision(
        &construction_policy.observation,
        &construction_candidates,
        vec![true; construction_candidates.len()],
    );
    let source_build =
        crate::ml::policy_v2::factorized_log_probs(&source_path, &[&construction], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
    let target_build =
        crate::ml::policy_v2::factorized_log_probs(&with_adapters, &[&construction], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
    let build_error = source_build
        .iter()
        .zip(&target_build)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(
        build_error <= 1.0e-6,
        "general-construction actor error={build_error}"
    );
    let critic_build_before = crate::ml::semantic_ppo::critic_values(
        &source_critic,
        &[&construction],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    )
    .into_data()
    .to_vec::<f32>()
    .unwrap()[0];
    let critic_build_after = crate::ml::semantic_ppo::critic_values(
        &with_critic,
        &[&construction],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    )
    .into_data()
    .to_vec::<f32>()
    .unwrap()[0];
    assert!((critic_build_before - critic_build_after).abs() <= 1.0e-6);
    println!(
        "CARD_SERVICE_ZERO_RESIDUAL actor_max_abs={actor_error:.9} critic_max_abs={critic_error:.9} target_actor_max_abs={target_error:.9} target_critic_abs={:.9} construction_actor_max_abs={build_error:.9} construction_critic_abs={:.9}",
        (value_before - value_after).abs(),
        (critic_build_before - critic_build_after).abs()
    );
}

#[test]
fn card_service_residual_has_gradient_updates_and_roundtrips() {
    use crate::environment::{AgentAction as A, GameEnvironment};
    use crate::ml::model::TrainBackend;

    let config = ModelConfig { hidden_size: 8 };
    let device = default_policy_device();
    let mut env = GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        991,
    );
    let slot =
        env.add_card_service_shop_slot_for_test(td_core::CardServiceKind::CardPolisher.raw());
    env.step(A::PurchaseShopItem { slot_index: slot }).unwrap();
    let candidates = env
        .legal_actions()
        .into_iter()
        .map(|legal| PolicyCandidate {
            action_id: legal.action.action_id(),
            action: legal.action,
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let decision = encode_decision(&env.snapshot(), &candidates, vec![true; candidates.len()]);
    assert!(!decision.card_service_context.is_empty());
    assert!(
        decision
            .candidate_card_service_effects
            .iter()
            .any(|rows| !rows.is_empty())
    );
    let mut actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
        config,
        KindMode::Learned,
        991,
        &device,
    )
    .unwrap()
    .with_inputs(crate::ml::feature_contract::InputContract::NormalizedEffects);
    actor.scorer = actor.scorer.with_card_service_adapters(config, &device);
    let before = card_service_adapter_snapshot(&actor.scorer);
    let logits = factorized_log_probs(&actor, &[&decision], &device).joint;
    // Connectivity target: prefer candidate index 0 for this one fixture.
    // This is a synthetic gradient probe, not a service-value label.
    let loss = -logits.slice([0..1, 0..1]).mean();
    let gradients = GradientsParams::from_grads(loss.clone().backward(), &actor);
    let grad_norm = crate::ml::ppo::gradient_l2_norm(&actor, &gradients);
    assert!(grad_norm.is_finite() && grad_norm > 0.0);
    let mut optimizer = AdamConfig::new().init();
    actor = optimizer.step(1e-3, actor, gradients);
    let after = card_service_adapter_snapshot(&actor.scorer);
    assert_ne!(
        before, after,
        "zero residual parameters must receive the synthetic loss gradient"
    );
    let bytes = module_to_bytes(actor.clone()).unwrap();
    let mut restored = crate::ml::policy_v2::policy_from_bytes::<TrainBackend>(
        config,
        KindMode::Learned,
        bytes,
        &device,
    )
    .unwrap();
    restored.inputs = actor.inputs;
    restored.temperature = actor.temperature;
    assert_eq!(after, card_service_adapter_snapshot(&restored.scorer));
    let restored_logits = factorized_log_probs(&restored, &[&decision], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let updated_logits = factorized_log_probs(&actor, &[&decision], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    assert_eq!(restored_logits, updated_logits);
    let mut critic =
        crate::ml::semantic_bc::seeded_materialized_model(config, 992, &device).unwrap();
    critic = critic.with_card_service_adapters(config, &device);
    let critic_before = card_service_adapter_snapshot(&critic);
    let mut critic_optimizer = AdamConfig::new().init();
    let critic_loss = (crate::ml::semantic_ppo::critic_values(
        &critic,
        &[&decision],
        crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
        &device,
    ) - 1.0)
        .powf_scalar(2.0)
        .mean();
    let critic_gradients = GradientsParams::from_grads(critic_loss.clone().backward(), &critic);
    let critic_grad_norm = crate::ml::ppo::gradient_l2_norm(&critic, &critic_gradients);
    assert!(critic_grad_norm.is_finite() && critic_grad_norm > 0.0);
    critic = critic_optimizer.step(1e-3, critic, critic_gradients);
    let critic_after = card_service_adapter_snapshot(&critic);
    assert_ne!(critic_before, critic_after);
    let critic_bytes = crate::ml::model::model_to_full_precision_bytes(critic).unwrap();
    let restored_critic = crate::ml::model::model_from_full_precision_bytes::<TrainBackend>(
        config,
        critic_bytes,
        &device,
    )
    .unwrap();
    assert_eq!(
        critic_after,
        card_service_adapter_snapshot(&restored_critic)
    );
    println!(
        "CARD_SERVICE_SYNTHETIC_CONNECTIVITY actor_target=candidate_0 actor_grad_l2={grad_norm:.9} critic_target=value_1 critic_grad_l2={critic_grad_norm:.9} adapter_params_changed=true save_load_exact=true service_value_learning=false"
    );
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn resumed_ppo_restores_optimizer_steps_for_stable_service_parameter_ids() {
    use crate::ml::semantic_ppo::{PpoRunMetadata, adam_step_counters, load_learner};

    let run = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("artifacts/retraining/card-service-context-20261014/attempt-005/run");
    let metadata: PpoRunMetadata =
        serde_json::from_slice(&std::fs::read(run.join("ppo.json")).unwrap()).unwrap();
    assert_eq!(metadata.completed_iterations, 2);
    let step_at_one = load_learner(
        &run,
        1,
        &metadata.config,
        metadata.model_config,
        metadata.critic_inputs,
    )
    .unwrap();
    let step_at_two = load_learner(
        &run,
        2,
        &metadata.config,
        metadata.model_config,
        metadata.critic_inputs,
    )
    .unwrap();
    let service_ids = [0x4353525653540000_u64, 0x4353525653540001];
    let actor_one = adam_step_counters(&step_at_one.actor_optimizer, &step_at_one.actor).unwrap();
    let actor_two = adam_step_counters(&step_at_two.actor_optimizer, &step_at_two.actor).unwrap();
    let critic_one =
        adam_step_counters(&step_at_one.critic_optimizer, &step_at_one.critic).unwrap();
    let critic_two =
        adam_step_counters(&step_at_two.critic_optimizer, &step_at_two.critic).unwrap();
    for id in service_ids {
        let a1 = actor_one.iter().find(|(param, _)| *param == id).unwrap().1;
        let a2 = actor_two.iter().find(|(param, _)| *param == id).unwrap().1;
        let c1 = critic_one.iter().find(|(param, _)| *param == id).unwrap().1;
        let c2 = critic_two.iter().find(|(param, _)| *param == id).unwrap().1;
        assert!(
            a1 > 0 && a2 > a1,
            "actor optimizer parameter {id:#x}: {a1} -> {a2}"
        );
        assert!(
            c1 > 0 && c2 > c1,
            "critic optimizer parameter {id:#x}: {c1} -> {c2}"
        );
        println!("service_optimizer_param={id:#x} actor_step={a1}->{a2} critic_step={c1}->{c2}");
    }
    for id in [0x4353525643410000_u64, 0x4353525643410001] {
        let actor_step_one = actor_one
            .iter()
            .find(|(param, _)| *param == id)
            .map(|(_, step)| *step);
        let actor_step_two = actor_two
            .iter()
            .find(|(param, _)| *param == id)
            .map(|(_, step)| *step);
        let critic_has_candidate_state = critic_two.iter().any(|(param, _)| *param == id);
        println!(
            "service_candidate_optimizer_param={id:#x} actor_step={actor_step_one:?}->{actor_step_two:?} critic_has_step={critic_has_candidate_state}"
        );
    }
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn resumed_final_checkpoint_infers_on_shop_service_and_build_states() {
    use crate::environment::{AgentAction as A, GameEnvironment};
    use crate::ml::semantic_candidates::{PolicyCandidate, encode_decision};

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("artifacts/retraining/card-service-context-20261014");
    let source = root.join("../card-polisher-20261014/run/iter-0002");
    let final_checkpoint = root.join("attempt-005/run/iter-0002");
    let device = default_policy_device();
    let (final_actor, _) =
        crate::ml::semantic_ppo::load_ppo_policy(&final_checkpoint, &device).unwrap();
    let source_actor = crate::ml::semantic_ppo::load_transfer_actor_as::<
        crate::ml::model::InferenceBackend,
    >(&source, &device)
    .unwrap();
    assert_ne!(
        card_service_adapter_snapshot(&final_actor.scorer),
        card_service_adapter_snapshot(&source_actor.scorer)
    );
    let final_critic =
        crate::ml::semantic_bc::load_model_file::<crate::ml::model::InferenceBackend>(
            ModelConfig::default(),
            &final_checkpoint.join("critic.bin"),
            &device,
        )
        .unwrap();
    let source_critic =
        crate::ml::semantic_bc::load_model_file::<crate::ml::model::InferenceBackend>(
            ModelConfig::default(),
            &source.join("critic.bin"),
            &device,
        )
        .unwrap();
    assert_ne!(
        card_service_adapter_snapshot(&final_critic),
        card_service_adapter_snapshot(&source_critic)
    );

    let mut env = GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        1001,
    );
    let slot =
        env.add_card_service_shop_slot_for_test(td_core::CardServiceKind::CardPolisher.raw());
    let shop_candidates = env
        .legal_actions()
        .into_iter()
        .map(|legal| PolicyCandidate {
            action_id: legal.action.action_id(),
            action: legal.action,
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let shop = encode_decision(
        &env.snapshot(),
        &shop_candidates,
        vec![true; shop_candidates.len()],
    );
    env.step(A::PurchaseShopItem { slot_index: slot }).unwrap();
    let target_candidates = env
        .legal_actions()
        .into_iter()
        .map(|legal| PolicyCandidate {
            action_id: legal.action.action_id(),
            action: legal.action,
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let target = encode_decision(
        &env.snapshot(),
        &target_candidates,
        vec![true; target_candidates.len()],
    );
    let general_env = GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        1002,
    );
    let build = crate::ml::semantic_candidates::policy_candidates(&general_env).unwrap();
    let construction = encode_decision(
        &build.observation,
        &build.candidates,
        vec![true; build.candidates.len()],
    );
    for decision in [&shop, &target, &construction] {
        let logits = crate::ml::policy_v2::factorized_log_probs(&final_actor, &[decision], &device)
            .joint
            .into_data()
            .to_vec::<f32>()
            .unwrap();
        assert!(!logits.is_empty() && logits.iter().all(|value| value.is_finite()));
        let value = crate::ml::semantic_ppo::critic_values(
            &final_critic,
            &[decision],
            crate::ml::semantic_ppo::CriticInputs::SquashAllEffects,
            &device,
        )
        .into_data()
        .to_vec::<f32>()
        .unwrap()[0];
        assert!(value.is_finite());
    }
}

fn card_service_adapter_snapshot<B: burn::tensor::backend::Backend>(
    model: &crate::ml::model::DeepSetsActorCritic<B>,
) -> Vec<(ParamId, Vec<f32>)> {
    struct Snapshot<B: burn::tensor::backend::Backend> {
        values: Vec<(ParamId, Vec<f32>)>,
        marker: std::marker::PhantomData<B>,
    }
    impl<B: burn::tensor::backend::Backend> ModuleVisitor<B> for Snapshot<B> {
        fn visit_float<const D: usize>(&mut self, param: &Param<Tensor<B, D>>) {
            let ids = [
                ParamId::from(0x4353525653540000),
                ParamId::from(0x4353525653540001),
                ParamId::from(0x4353525643410000),
                ParamId::from(0x4353525643410001),
            ];
            if ids.contains(&param.id) {
                self.values.push((
                    param.id,
                    param.val().clone().into_data().to_vec::<f32>().unwrap(),
                ));
            }
        }
    }
    let mut snapshot = Snapshot {
        values: Vec::new(),
        marker: std::marker::PhantomData,
    };
    model.visit(&mut snapshot);
    snapshot.values.sort_by_key(|(id, _)| *id);
    snapshot.values
}

#[test]
fn training_melon_candidate_id_effect_and_policy_gradient_are_connected() {
    let device = default_policy_device();
    let mut observation = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        23,
    )
    .snapshot();
    observation.decision_point = crate::environment::DecisionPoint::TreasureSelection;
    observation.treasure_options = vec!["training_melon".into(), "apple".into()];
    observation.content_effects = vec![td_core::ContentEffectObservation {
        source: S::TreasureOption,
        source_index: 0,
        key_id: td_core::UpgradeKind::TrainingMelon.id(),
        kind: K::MaxHp,
        parameter_index: 0,
        value_raw: 8_000,
        enabled: true,
    }];
    let candidates = (0..2)
        .map(|option_index| PolicyCandidate {
            action: AgentAction::SelectTreasure { option_index },
            action_id: format!("treasure-{option_index}"),
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let encoded = encode_decision(&observation, &candidates, vec![true, true]);
    assert_eq!(crate::ml::vocabulary::upgrade_key_id("training_melon"), 38);
    assert!(
        encoded.candidates[0]
            .rows
            .iter()
            .any(|row| row.categorical[0] == 3007 && row.categorical[1] == 39)
    );
    assert!(
        encoded.candidate_effects[0]
            .rows
            .iter()
            .any(
                |row| row.categorical[1] == td_core::UpgradeKind::TrainingMelon.id() as u32
                    && row.categorical[2] == 3520 + K::MaxHp as u32
                    && (row.numeric[0] - 9.0_f32.ln()).abs() < 1e-6
            )
    );

    let config = ModelConfig { hidden_size: 8 };
    let mut actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
        config,
        KindMode::Learned,
        123,
        &device,
    )
    .unwrap();
    let before_bytes = module_to_bytes(actor.clone()).unwrap();
    let (embedding_id, embedding_before) = entity_embedding_snapshot(&actor);
    let before = factorized_log_probs(&actor, &[&encoded], &device).joint;
    let probabilities = before.clone().exp();
    // Entropy regularization is part of PPO and gives a non-label-based
    // gradient over the complete legal candidate distribution.
    let loss = (probabilities * before.clone()).sum();
    let before = before.into_data().to_vec::<f32>().unwrap();
    let mut optimizer = AdamConfig::new().init();
    let gradients = GradientsParams::from_grads(loss.backward(), &actor);
    let embedding_gradient = gradients
        .get::<<TrainBackend as burn::tensor::backend::AutodiffBackend>::InnerBackend, 2>(
            embedding_id,
        )
        .expect("TrainingMelon candidate must backpropagate to the entity embedding table")
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let melon_embedding_gradient_l2 = embedding_gradient[39 * 8..40 * 8]
        .iter()
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();
    assert!(melon_embedding_gradient_l2 > 0.0);
    actor = optimizer.step(1e-3, actor, gradients);
    let (_, embedding_after) = entity_embedding_snapshot(&actor);
    let melon_embedding_row_changed =
        embedding_before[39 * 8..40 * 8] != embedding_after[39 * 8..40 * 8];
    assert!(melon_embedding_row_changed);
    let after = factorized_log_probs(&actor, &[&encoded], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let after_bytes = module_to_bytes(actor).unwrap();
    assert_ne!(after_bytes, before_bytes);
    assert_ne!(after, before);
    println!(
        "TRAINING_MELON_PPO_PATH={{\"before_log_probs\":{before:?},\"after_log_probs\":{after:?},\"parameters_changed\":true,\"melon_embedding_gradient_l2\":{melon_embedding_gradient_l2},\"melon_embedding_row_changed\":{melon_embedding_row_changed}}}"
    );
}

#[test]
fn shield_berry_inventory_candidate_and_effect_reach_actor_inputs() {
    use crate::environment::InventoryObservation;
    use burn::optim::{AdamConfig, GradientsParams, Optimizer};

    let device = default_policy_device();
    let mut observation = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        812_041,
    )
    .snapshot();
    observation.hp_raw = 30_000;
    observation.inventory.extend([
        InventoryObservation {
            index: 3,
            key: "shield_berry".into(),
            key_id: 12,
        },
        InventoryObservation {
            index: 4,
            key: "milk".into(),
            key_id: td_core::ItemKind::Milk.id(),
        },
    ]);
    for (source_index, key_id) in [
        (3, td_core::ItemKind::ShieldBerry.id()),
        (4, td_core::ItemKind::Milk.id()),
    ] {
        observation
            .content_effects
            .push(td_core::ContentEffectObservation {
                source: S::Inventory,
                source_index,
                key_id,
                kind: K::Shield,
                parameter_index: 0,
                value_raw: 12_000,
                enabled: true,
            });
    }
    observation.decision_point = crate::environment::DecisionPoint::PreDefenseItem;
    let candidates = [3, 4].map(|item_index| PolicyCandidate {
        action: AgentAction::UseInventoryItem { item_index },
        action_id: format!("use_inventory_item:{item_index}"),
        family_rank: None,
        spatial: None,
    });
    let encoded = encode_decision(&observation, &candidates, vec![true, true]);
    assert_eq!(encoded.candidates.len(), 2);
    assert!(
        encoded
            .typed
            .sets
            .iter()
            .flat_map(|set| &set.rows)
            .any(|row| row.categorical[0] == 12)
    );
    assert!(
        encoded.candidates[0]
            .rows
            .iter()
            .any(|row| row.categorical[0] == 3006 && row.categorical[1] == 13)
    );
    assert!(encoded.state_effects.rows.iter().any(|row| {
        row.categorical == [3500, 12, 3520 + K::Shield as u32, 0]
            && (row.numeric[0] - 12.0_f32.ln_1p()).abs() < 1e-6
    }));
    assert!(encoded.candidate_effects[0].rows.iter().any(|row| {
        row.categorical == [3500, 12, 3520 + K::Shield as u32, 0]
            && (row.numeric[0] - 12.0_f32.ln_1p()).abs() < 1e-6
    }));

    let actor = crate::ml::semantic_bc::seeded_policy_net::<TrainBackend>(
        ModelConfig { hidden_size: 8 },
        KindMode::Learned,
        812_041,
        &device,
    )
    .unwrap()
    .with_inputs(crate::ml::feature_contract::InputContract::RawEffects);
    let log_probs = factorized_log_probs(&actor, &[&encoded], &device).joint;
    let output = log_probs.clone().into_data().to_vec::<f32>().unwrap();
    assert_eq!(output.len(), 2);
    assert!(output.iter().all(|value| value.is_finite()));
    let loss = (log_probs.clone().exp() * log_probs).sum();
    let (embedding_id, before) = entity_embedding_snapshot(&actor);
    let gradients = GradientsParams::from_grads(loss.backward(), &actor);
    let gradient = gradients
        .get::<<TrainBackend as burn::tensor::backend::AutodiffBackend>::InnerBackend, 2>(
            embedding_id,
        )
        .unwrap()
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    let shield_gradient_l2 = gradient[12 * 8..13 * 8]
        .iter()
        .chain(&gradient[13 * 8..14 * 8])
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();
    assert!(shield_gradient_l2 > 0.0);
    let mut optimizer = AdamConfig::new().init();
    let actor = optimizer.step(1e-3, actor, gradients);
    let (_, after) = entity_embedding_snapshot(&actor);
    assert_ne!(before[12 * 8..14 * 8], after[12 * 8..14 * 8]);
    println!(
        "SHIELD_BERRY_ACTOR_SIGNAL={{\"joint_log_probs\":{output:?},\"embedding_rows_12_13_gradient_l2\":{shield_gradient_l2}}}"
    );
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn final_shield_berry_checkpoint_infers_on_inventory_fixture() {
    use crate::environment::InventoryObservation;

    let checkpoint = std::env::var("SHIELD_BERRY_CHECKPOINT")
        .expect("set SHIELD_BERRY_CHECKPOINT to the final iteration directory");
    let checkpoint = std::path::Path::new(&checkpoint);
    let device = default_policy_device();
    let (actor, candidate_mode) = crate::ml::semantic_ppo::load_ppo_policy(checkpoint, &device)
        .expect("load final ShieldBerry actor checkpoint");
    let metadata: crate::ml::semantic_ppo::PpoActorFile =
        serde_json::from_slice(&std::fs::read(checkpoint.join("ppo-actor.json")).unwrap()).unwrap();
    assert_eq!(metadata.iteration, 2);
    assert_eq!(
        candidate_mode,
        crate::ml::semantic_candidates::CandidateMode::AllBuildOptions
    );
    let run_dir = checkpoint.parent().unwrap();
    let run_metadata: crate::ml::semantic_ppo::PpoRunMetadata =
        serde_json::from_slice(&std::fs::read(run_dir.join("ppo.json")).unwrap()).unwrap();
    let learner_one = crate::ml::semantic_ppo::load_learner(
        run_dir,
        1,
        &run_metadata.config,
        run_metadata.model_config,
        run_metadata.critic_inputs,
    )
    .unwrap();
    let actor_steps_one = crate::ml::semantic_ppo::adam_step_counters(
        &learner_one.actor_optimizer,
        &learner_one.actor,
    )
    .unwrap();
    let critic_steps_one = crate::ml::semantic_ppo::adam_step_counters(
        &learner_one.critic_optimizer,
        &learner_one.critic,
    )
    .unwrap();
    let learner_two = crate::ml::semantic_ppo::load_learner(
        run_dir,
        2,
        &run_metadata.config,
        run_metadata.model_config,
        run_metadata.critic_inputs,
    )
    .unwrap();
    let actor_steps_two = crate::ml::semantic_ppo::adam_step_counters(
        &learner_two.actor_optimizer,
        &learner_two.actor,
    )
    .unwrap();
    let critic_steps_two = crate::ml::semantic_ppo::adam_step_counters(
        &learner_two.critic_optimizer,
        &learner_two.critic,
    )
    .unwrap();
    for (before, after) in [
        (&actor_steps_one, &actor_steps_two),
        (&critic_steps_one, &critic_steps_two),
    ] {
        assert!(!before.is_empty() && before.iter().all(|(_, step)| *step > 0));
        assert_eq!(before.len(), after.len());
        assert!(
            before
                .iter()
                .zip(after)
                .all(|((id_a, step_a), (id_b, step_b))| id_a == id_b && step_b > step_a)
        );
    }
    println!(
        "SHIELD_BERRY_OPTIMIZER_RESTORE={{\"iteration_1_actor_steps\":{:?},\"iteration_2_actor_steps\":{:?},\"iteration_1_critic_steps\":{:?},\"iteration_2_critic_steps\":{:?}}}",
        actor_steps_one
            .iter()
            .map(|(_, step)| step)
            .collect::<Vec<_>>(),
        actor_steps_two
            .iter()
            .map(|(_, step)| step)
            .collect::<Vec<_>>(),
        critic_steps_one
            .iter()
            .map(|(_, step)| step)
            .collect::<Vec<_>>(),
        critic_steps_two
            .iter()
            .map(|(_, step)| step)
            .collect::<Vec<_>>()
    );
    let catalog = &metadata.environment.as_ref().unwrap().content_catalog;
    assert_eq!(catalog["items"].len(), 12);
    assert_eq!(catalog["items"][11], "ShieldBerry");
    assert_eq!(catalog["treasures"].len(), 38);

    let mut observation = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        812_041,
    )
    .snapshot();
    observation.inventory.extend([
        InventoryObservation {
            index: 3,
            key: "shield_berry".into(),
            key_id: 12,
        },
        InventoryObservation {
            index: 4,
            key: "milk".into(),
            key_id: td_core::ItemKind::Milk.id(),
        },
    ]);
    for (source_index, key_id) in [
        (3, td_core::ItemKind::ShieldBerry.id()),
        (4, td_core::ItemKind::Milk.id()),
    ] {
        observation
            .content_effects
            .push(td_core::ContentEffectObservation {
                source: S::Inventory,
                source_index,
                key_id,
                kind: K::Shield,
                parameter_index: 0,
                value_raw: 12_000,
                enabled: true,
            });
    }
    observation.decision_point = crate::environment::DecisionPoint::PreDefenseItem;
    let candidates = [3, 4].map(|item_index| PolicyCandidate {
        action: AgentAction::UseInventoryItem { item_index },
        action_id: format!("use_inventory_item:{item_index}"),
        family_rank: None,
        spatial: None,
    });
    let encoded = encode_decision(&observation, &candidates, vec![true, true]);
    let scores = factorized_log_probs(&actor, &[&encoded], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    assert_eq!(scores.len(), 2);
    assert!(scores.iter().all(|score| score.is_finite()));
    println!(
        "SHIELD_BERRY_CHECKPOINT_INFERENCE={{\"iteration\":{},\"candidate_mode\":\"{:?}\",\"item_catalog_count\":12,\"treasure_catalog_count\":38,\"candidate_log_probs\":{:?}}}",
        metadata.iteration, candidate_mode, scores
    );
}

#[test]
#[ignore = "requires retained local experiment checkpoints; run explicitly with their environment variables"]
fn final_training_melon_checkpoint_infers_on_training_melon_fixture() {
    let checkpoint = std::env::var("TRAINING_MELON_CHECKPOINT")
        .expect("set TRAINING_MELON_CHECKPOINT to the final iteration directory");
    let checkpoint = std::path::Path::new(&checkpoint);
    let device = default_policy_device();
    let (actor, candidate_mode) = crate::ml::semantic_ppo::load_ppo_policy(checkpoint, &device)
        .expect("load final actor checkpoint");
    let metadata: crate::ml::semantic_ppo::PpoActorFile =
        serde_json::from_slice(&std::fs::read(checkpoint.join("ppo-actor.json")).unwrap()).unwrap();
    assert_eq!(metadata.iteration, 2);
    assert_eq!(
        candidate_mode,
        crate::ml::semantic_candidates::CandidateMode::AllBuildOptions
    );
    assert_eq!(
        metadata.environment.as_ref().unwrap().content_catalog["treasures"].len(),
        38
    );

    let mut observation = crate::environment::GameEnvironment::new(
        std::sync::Arc::new(crate::config::GameConfig::default_config()),
        23,
    )
    .snapshot();
    observation.decision_point = crate::environment::DecisionPoint::TreasureSelection;
    observation.treasure_options = vec!["training_melon".into(), "apple".into()];
    observation.content_effects = vec![td_core::ContentEffectObservation {
        source: S::TreasureOption,
        source_index: 0,
        key_id: td_core::UpgradeKind::TrainingMelon.id(),
        kind: K::MaxHp,
        parameter_index: 0,
        value_raw: 8_000,
        enabled: true,
    }];
    let candidates = (0..2)
        .map(|option_index| PolicyCandidate {
            action: AgentAction::SelectTreasure { option_index },
            action_id: format!("treasure-{option_index}"),
            family_rank: None,
            spatial: None,
        })
        .collect::<Vec<_>>();
    let encoded = encode_decision(&observation, &candidates, vec![true, true]);
    let scores = factorized_log_probs(&actor, &[&encoded], &device)
        .joint
        .into_data()
        .to_vec::<f32>()
        .unwrap();
    assert_eq!(scores.len(), 2);
    assert!(scores.iter().all(|score| score.is_finite()));
    let report = serde_json::json!({
        "checkpoint": checkpoint,
        "iteration": metadata.iteration,
        "candidate_mode": candidate_mode,
        "content_catalog_treasures": metadata.environment.unwrap().content_catalog["treasures"].len(),
        "fixture_candidates": ["training_melon", "apple"],
        "training_melon_vocab_id": crate::ml::vocabulary::upgrade_key_id("training_melon"),
        "effect": {"kind":"MaxHp", "value_raw":8000, "normalized":9.0_f32.ln()},
        "actor_joint_log_probabilities": scores,
    });
    println!("TRAINING_MELON_CHECKPOINT_INFERENCE={}", report);
}
