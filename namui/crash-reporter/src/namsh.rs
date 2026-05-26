//! namsh wire protocol — `intake_crash` HMAC envelope + presigned R2 PUT.
//!
//! Request body uses snake_case keys (forte_json reads them as-is). Response is
//! internally-tagged with `"t"` discriminator and camelCase fields (forte_json's
//! Serializer convention — see `fn0/forte/json/src/lib.rs`).
//!
//! HMAC payload is `build_id || stack_hash || sha256(serde_json::to_vec(context))`
//! per namsh `actions/intake_crash.rs::verify_hmac`. Using plain `serde_json` is
//! intentional — `verify_hmac` itself uses `serde_json::to_vec(context)`, not the
//! forte_json camelCase variant.

use crate::{Config, Error, context::CrashContext, queue};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{path::Path, time::Duration};

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize)]
struct IntakeRequest<'a> {
    build_id: &'a str,
    stack_hash: &'a str,
    context: &'a CrashContext,
}

#[derive(Deserialize)]
#[serde(tag = "t")]
enum IntakeResponse {
    Ok {
        upload: Option<UploadGrant>,
    },
    UnknownBuild,
    InvalidSignature,
    RateLimited,
    PayloadTooLarge,
    Error {
        message: String,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadGrant {
    #[allow(dead_code)]
    dump_id: String,
    presigned_put_url: String,
}

fn http_client() -> Result<reqwest::blocking::Client, Error> {
    Ok(reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?)
}

fn sign(
    hmac_key_hex: &str,
    build_id: &str,
    stack_hash: &str,
    context_json: &[u8],
) -> Result<String, Error> {
    let key = hex::decode(hmac_key_hex)?;
    let mut mac = HmacSha256::new_from_slice(&key).map_err(|_| Error::HmacKey)?;
    let context_digest = Sha256::digest(context_json);
    mac.update(build_id.as_bytes());
    mac.update(stack_hash.as_bytes());
    mac.update(&context_digest);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn submit(
    config: &Config,
    stack_hash: &str,
    context: &CrashContext,
    dump_bytes: Vec<u8>,
) -> Result<(), Error> {
    let context_json = serde_json::to_vec(context)?;
    let signature = sign(&config.hmac_key_hex, &config.build_id, stack_hash, &context_json)?;

    let body = serde_json::to_vec(&IntakeRequest {
        build_id: &config.build_id,
        stack_hash,
        context,
    })?;

    let client = http_client()?;
    let url = format!("{}/__forte_action/intake_crash", config.namsh_url);

    let response = client
        .post(&url)
        .header("content-type", "application/json")
        .header("x-namsh-build-id", &config.build_id)
        .header("x-namsh-signature", &signature)
        .body(body)
        .send()?;

    let status = response.status();
    let response_bytes = response.bytes()?;
    if !status.is_success() {
        return Err(Error::NamshRejected(format!(
            "http {} — {}",
            status,
            String::from_utf8_lossy(&response_bytes)
        )));
    }

    let parsed: IntakeResponse = serde_json::from_slice(&response_bytes)?;
    let grant = match parsed {
        IntakeResponse::Ok { upload: Some(g) } => g,
        IntakeResponse::Ok { upload: None } => return Ok(()),
        IntakeResponse::UnknownBuild => {
            return Err(Error::NamshRejected("UnknownBuild".into()));
        }
        IntakeResponse::InvalidSignature => {
            return Err(Error::NamshRejected("InvalidSignature".into()));
        }
        IntakeResponse::RateLimited => {
            return Err(Error::NamshRejected("RateLimited".into()));
        }
        IntakeResponse::PayloadTooLarge => {
            return Err(Error::NamshRejected("PayloadTooLarge".into()));
        }
        IntakeResponse::Error { message } => {
            return Err(Error::NamshRejected(format!("Error: {message}")));
        }
    };

    let put_response = client
        .put(&grant.presigned_put_url)
        .header("content-type", "application/octet-stream")
        .body(dump_bytes)
        .send()?;
    let put_status = put_response.status();
    if !put_status.is_success() {
        let put_body = put_response.bytes().unwrap_or_default();
        return Err(Error::NamshRejected(format!(
            "r2 put http {} — {}",
            put_status,
            String::from_utf8_lossy(&put_body)
        )));
    }
    Ok(())
}

pub fn upload_single(config: &Config, dump_path: &Path) -> Result<(), Error> {
    let entry = queue::load_sidecar(dump_path)?;
    let dump_bytes = std::fs::read(dump_path)?;
    submit(config, &entry.stack_hash, &entry.context, dump_bytes)?;
    queue::delete_entry(dump_path)?;
    Ok(())
}

pub fn flush_queue(config: &Config) -> Result<(), Error> {
    let pending = queue::list_pending(&config.app_name)?;
    for dump_path in pending {
        if let Err(e) = upload_single(config, &dump_path) {
            eprintln!(
                "[crash-reporter] queue upload failed ({}): {e}",
                dump_path.display()
            );
        }
    }
    Ok(())
}
