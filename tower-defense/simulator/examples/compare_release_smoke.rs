use burn::{
    module::{Module, ModuleVisitor, Param},
    optim::Optimizer,
    record::{BinBytesRecorder, BinFileRecorder, FullPrecisionSettings, Recorder},
    tensor::{Tensor, backend::Backend},
};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use td_simulator::ml::{
    model::{
        DeepSetsActorCritic, InferenceBackend, TrainBackend, default_policy_device,
        model_from_full_precision_bytes,
    },
    semantic_bc::{load_model_file, load_policy_file},
    semantic_ppo::{CriticInputs, PpoActorFile, PpoConfig, PpoLearner, load_ppo_actor_as},
};

#[derive(Deserialize)]
struct Meta {
    model_config: td_simulator::ml::model::ModelConfig,
    config: PpoConfig,
    critic_inputs: CriticInputs,
}
fn read_meta(path: &Path) -> Meta {
    serde_json::from_slice(&std::fs::read(path.join("ppo.json")).unwrap()).unwrap()
}
fn load_actor_opt(path: &Path, meta: &Meta) -> Vec<(u64, Vec<u8>)> {
    let d = path.join("iter-0002");
    let device = default_policy_device();
    let actor = load_ppo_actor_as::<TrainBackend>(&d, &device).unwrap();
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    let critic = model_from_full_precision_bytes::<TrainBackend>(
        meta.model_config,
        std::fs::read(d.join("critic.bin")).unwrap(),
        &device,
    )
    .unwrap();
    let mut learner = PpoLearner::new(actor, critic, meta.critic_inputs, &meta.config);
    learner.actor_optimizer = learner.actor_optimizer.load_record(
        recorder
            .load(d.join("actor-optimizer.bin"), &device)
            .unwrap(),
    );
    let bytes = BinBytesRecorder::<FullPrecisionSettings>::default();
    let mut entries = learner
        .actor_optimizer
        .to_record()
        .into_iter()
        .map(|(id, s)| (id.val(), bytes.record(s, ()).unwrap()))
        .collect::<Vec<_>>();
    entries.sort_by_key(|v| v.0);
    entries
}
fn load_critic_opt(path: &Path, meta: &Meta) -> Vec<(u64, Vec<u8>)> {
    let d = path.join("iter-0002");
    let device = default_policy_device();
    let actor = load_ppo_actor_as::<TrainBackend>(&d, &device).unwrap();
    let recorder = BinFileRecorder::<FullPrecisionSettings>::default();
    let critic = model_from_full_precision_bytes::<TrainBackend>(
        meta.model_config,
        std::fs::read(d.join("critic.bin")).unwrap(),
        &device,
    )
    .unwrap();
    let mut learner = PpoLearner::new(actor, critic, meta.critic_inputs, &meta.config);
    learner.critic_optimizer = learner.critic_optimizer.load_record(
        recorder
            .load(d.join("critic-optimizer.bin"), &device)
            .unwrap(),
    );
    let bytes = BinBytesRecorder::<FullPrecisionSettings>::default();
    let mut entries = learner
        .critic_optimizer
        .to_record()
        .into_iter()
        .map(|(id, s)| (id.val(), bytes.record(s, ()).unwrap()))
        .collect::<Vec<_>>();
    entries.sort_by_key(|v| v.0);
    entries
}
fn collect_adapters<B: Backend>(scorer: &DeepSetsActorCritic<B>) -> BTreeMap<u64, Vec<f32>> {
    struct V<B: Backend>(BTreeMap<u64, Vec<f32>>, std::marker::PhantomData<B>);
    impl<B: Backend> ModuleVisitor<B> for V<B> {
        fn visit_float<const D: usize>(&mut self, p: &Param<Tensor<B, D>>) {
            let id = p.id.val();
            if (0x4546465354410000..0x4546465354410002).contains(&id)
                || (0x45464643414e0000..0x45464643414e0002).contains(&id)
                || (0x4546465452450000..0x4546465452450002).contains(&id)
                || (0x4d49475253544100..0x4d49475253544102).contains(&id)
                || (0x4d49475243414e00..0x4d49475243414e02).contains(&id)
            {
                self.0
                    .insert(id, p.val().clone().into_data().to_vec::<f32>().unwrap());
            }
        }
    }
    let mut v = V(BTreeMap::new(), std::marker::PhantomData);
    scorer.visit(&mut v);
    v.0
}
fn adapter_values(path: &Path) -> BTreeMap<u64, Vec<f32>> {
    let actor_dir = if path.join("actor.bin").exists() {
        path.to_path_buf()
    } else {
        path.join("iter-0002")
    };
    let actor =
        td_simulator::ml::semantic_ppo::load_ppo_actor(&actor_dir, &default_policy_device())
            .unwrap();
    collect_adapters(&actor.scorer)
}
fn critic_adapter_values(
    path: &Path,
    config: td_simulator::ml::model::ModelConfig,
) -> BTreeMap<u64, Vec<f32>> {
    let dir = if path.join("critic.bin").exists() {
        path.to_path_buf()
    } else {
        path.join("iter-0002")
    };
    let critic = load_model_file::<InferenceBackend>(
        config,
        &dir.join("critic.bin"),
        &default_policy_device(),
    )
    .unwrap();
    collect_adapters(&critic)
}

fn main() {
    let root = PathBuf::from("artifacts/retraining/release-smoke-20261013");
    let cont = root.join("active-continuous");
    let resumed = root.join("active-resumed");
    let meta = read_meta(&cont);
    for (label, path) in [("continuous", &cont), ("resumed", &resumed)] {
        println!(
            "{label} actor optimizer entries={} critic optimizer entries={}",
            load_actor_opt(path, &meta).len(),
            load_critic_opt(path, &meta).len()
        );
    }
    println!(
        "actor optimizer structural equality={}",
        load_actor_opt(&cont, &meta) == load_actor_opt(&resumed, &meta)
    );
    println!(
        "critic optimizer structural equality={}",
        load_critic_opt(&cont, &meta) == load_critic_opt(&resumed, &meta)
    );
    let source =
        root.join("../effect-adaptation-isolated-20261011/effect-reuse_weights-r1/iter-0004");
    let source_file: PpoActorFile =
        serde_json::from_slice(&std::fs::read(source.join("ppo-actor.json")).unwrap()).unwrap();
    let source_actor = load_policy_file::<InferenceBackend>(
        source_file.policy_representation_version,
        source_file.model_config,
        source_file.kind_mode,
        &source.join("actor.bin"),
        &default_policy_device(),
    )
    .unwrap();
    let source_values = collect_adapters(&source_actor.scorer);
    let init = adapter_values(&cont.join("iter-0000"));
    let final_values = adapter_values(&cont.join("iter-0002"));
    let effect_ids = [
        0x4546465354410000,
        0x4546465354410001,
        0x45464643414e0000,
        0x45464643414e0001,
        0x4546465452450000,
        0x4546465452450001,
    ];
    println!(
        "existing effect adapters exactly preserved at iter-0000={}",
        effect_ids
            .iter()
            .all(|id| source_values.get(id) == init.get(id))
    );
    for (label, values) in [
        ("source", &source_values),
        ("continuous-init", &init),
        ("continuous-final", &final_values),
        ("resumed-final", &adapter_values(&resumed.join("iter-0002"))),
    ] {
        let norms = values
            .iter()
            .map(|(id, xs)| {
                (
                    *id,
                    xs.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>().sqrt(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        println!("{label} actor_adapter_norms={norms:?}");
    }
    let source_meta = read_meta(source.parent().unwrap());
    let source_critic = critic_adapter_values(&source, source_meta.model_config);
    let init_critic = critic_adapter_values(&cont.join("iter-0000"), meta.model_config);
    let final_critic = critic_adapter_values(&cont.join("iter-0002"), meta.model_config);
    let critic_ids = [
        0x4546465354410000,
        0x4546465354410001,
        0x4d49475253544100,
        0x4d49475253544101,
    ];
    println!(
        "critic existing effect adapters exactly preserved at iter-0000={}",
        critic_ids[..2]
            .iter()
            .all(|id| source_critic.get(id) == init_critic.get(id))
    );
    println!(
        "critic migration state zero at iter-0000={}",
        critic_ids[2..]
            .iter()
            .all(|id| init_critic.get(id).unwrap().iter().all(|v| *v == 0.0))
    );
    println!(
        "critic migration adapter norms={:?}",
        final_critic
            .iter()
            .map(|(id, xs)| (
                *id,
                xs.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>().sqrt()
            ))
            .collect::<BTreeMap<_, _>>()
    );
}
