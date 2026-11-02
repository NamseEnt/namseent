use anyhow::{Context, Result, bail};
use burn::module::{Module, ModuleVisitor, Param};
use burn::optim::GradientsParams;
use burn::tensor::Tensor;
use sha2::{Digest, Sha256};

pub fn current_git_revision() -> Result<String> {
    let output = std::process::Command::new("git")
        .args(["-C", env!("CARGO_MANIFEST_DIR"), "rev-parse", "HEAD"])
        .output()
        .context("failed to invoke git for checkpoint metadata")?;
    if !output.status.success() {
        bail!("git rev-parse HEAD failed");
    }
    let revision = String::from_utf8(output.stdout).context("git revision was not UTF-8")?;
    let revision = revision.trim().to_string();
    if revision.is_empty() {
        bail!("git revision is empty");
    }
    Ok(revision)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(crate) fn gradient_l2_norm<B: burn::tensor::backend::AutodiffBackend, M: Module<B>>(
    model: &M,
    gradients: &GradientsParams,
) -> f32 {
    struct GradientVisitor<'a> {
        gradients: &'a GradientsParams,
        squared_norm: f64,
        finite: bool,
    }

    impl<B: burn::tensor::backend::AutodiffBackend> ModuleVisitor<B> for GradientVisitor<'_> {
        fn visit_float<const D: usize>(&mut self, parameter: &Param<Tensor<B, D>>) {
            let Some(gradient) = self.gradients.get::<B::InnerBackend, D>(parameter.id) else {
                return;
            };
            let squared_norm = gradient
                .powf_scalar(2.0)
                .sum()
                .into_data()
                .to_vec::<f32>()
                .expect("gradient tensor must be f32");
            let value = squared_norm[0];
            self.finite &= value.is_finite();
            self.squared_norm += f64::from(value);
        }
    }

    let mut visitor = GradientVisitor {
        gradients,
        squared_norm: 0.0,
        finite: true,
    };
    model.visit(&mut visitor);
    if visitor.finite {
        visitor.squared_norm.sqrt() as f32
    } else {
        f32::NAN
    }
}
