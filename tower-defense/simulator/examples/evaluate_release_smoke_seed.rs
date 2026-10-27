use anyhow::Result;
use std::{path::Path, sync::Arc};
use td_simulator::{
    config::load_jsonc,
    ml::{
        phase4_eval::{EvalPolicy, evaluate_policies},
        semantic_bc::SemanticPolicy,
    },
};
fn main() -> Result<()> {
    let root = Path::new("artifacts/retraining/release-smoke-20261013");
    let config = Arc::new(load_jsonc(Path::new(
        "artifacts/retraining/effect-adaptation-isolated-20261011/disabled.jsonc",
    ))?);
    let checkpoint = root.join("active-continuous/iter-0002");
    let policies = [
        EvalPolicy::Canonical,
        EvalPolicy::Learned {
            name: "smoke".to_string(),
            policy: Box::new(SemanticPolicy::from_path(&checkpoint)?),
        },
    ];
    let seeds = [8_900_000u64];
    let report = evaluate_policies(
        config,
        "release_smoke_development",
        &seeds,
        &policies,
        &[("smoke".to_string(), "canonical".to_string())],
    )?;
    std::fs::write(
        root.join("fresh-development-eval.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!(
        "fresh development seed {}: games {}",
        seeds[0],
        report.episodes.len()
    );
    for summary in &report.summaries {
        println!(
            "{} progress {:.2} illegal {} fallback {} post_sampling_mutations {}",
            summary.policy,
            summary.mean_terminal_clear_rate,
            summary.illegal_actions,
            summary.fallback_actions,
            summary.post_sampling_mutations
        );
    }
    Ok(())
}
