// SPDX-License-Identifier: MIT
pub use lambda_runtime::Error as LambdaError;

#[cfg(test)]
#[cfg(any(feature = "actix4", feature = "rocket05", feature = "hyper"))]
mod test_consts;

#[cfg(any(feature = "actix4", feature = "rocket05", feature = "hyper"))]
pub(crate) mod brotli;
#[cfg(any(feature = "actix4", feature = "rocket05", feature = "hyper"))]
mod request;

#[cfg(feature = "hyper")]
mod hyper014;
#[cfg(feature = "hyper")]
pub use hyper014::run_hyper_on_lambda;

/// Returns true if it is running on AWS Lambda
pub fn is_running_on_lambda() -> bool {
    std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok()
}
