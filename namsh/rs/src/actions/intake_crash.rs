use crate::docs::*;
use forte_sdk::*;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::Duration;

type HmacSha256 = Hmac<Sha256>;

const MAX_DUMPS_PER_GROUP: usize = 3;
const RATE_LIMIT_PER_MINUTE: usize = 1;
const RATE_LIMIT_PER_24H: usize = 5;
const LOG_TAIL_MAX_BYTES: usize = 64 * 1024;
const PRESIGNED_PUT_EXPIRES_SECS: u64 = 600;

#[derive(Deserialize)]
pub struct Input {
    pub build_id: String,
    pub stack_hash: String,
    pub context: CrashContext,
}

#[derive(Serialize)]
pub struct UploadGrant {
    pub dump_id: String,
    pub presigned_put_url: String,
}

#[derive(Serialize)]
pub enum Output {
    Ok { upload: Option<UploadGrant> },
    UnknownBuild,
    InvalidSignature,
    RateLimited,
    PayloadTooLarge,
    Error { message: String },
}

pub async fn handler(req: ForteRequest<'_, Input>) -> Output {
    let Some(header_build_id) = req.headers.get("x-namsh-build-id") else {
        return Output::InvalidSignature;
    };
    let Ok(header_build_id) = header_build_id.to_str() else {
        return Output::InvalidSignature;
    };
    if header_build_id != req.body.build_id {
        return Output::InvalidSignature;
    }

    let Some(signature_hex) = req.headers.get("x-namsh-signature") else {
        return Output::InvalidSignature;
    };
    let Ok(signature_hex) = signature_hex.to_str() else {
        return Output::InvalidSignature;
    };
    let Ok(signature) = hex::decode(signature_hex.trim()) else {
        return Output::InvalidSignature;
    };

    if let Some(tail) = &req.body.context.log_tail
        && tail.len() > LOG_TAIL_MAX_BYTES
    {
        return Output::PayloadTooLarge;
    }

    let db = doc_db::turso();
    let build = match (BuildDocGet {
        build_id: req.body.build_id.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(Some(b)) => b,
        Ok(None) => return Output::UnknownBuild,
        Err(e) => {
            tracing::error!(?e, "intake_crash BuildDocGet");
            return Output::Error {
                message: format!("BuildDocGet: {e}"),
            };
        }
    };

    if !verify_hmac(
        &build.hmac_key_hex,
        &req.body.build_id,
        &req.body.stack_hash,
        &req.body.context,
        &signature,
    ) {
        return Output::InvalidSignature;
    }

    let Some(client_ip) = extract_client_ip(req.headers) else {
        return Output::InvalidSignature;
    };

    let now = forte_sdk::now();
    let rate_doc = match (IpRateLimitDocGet { ip: client_ip.clone() })
        .send_with(&db)
        .await
    {
        Ok(d) => d,
        Err(e) => {
            tracing::error!(?e, "intake_crash IpRateLimitDocGet");
            return Output::Error {
                message: format!("IpRateLimitDocGet: {e}"),
            };
        }
    };
    let mut recent = rate_doc.map(|d| d.recent_requests).unwrap_or_default();
    prune_old(&mut recent, now);
    if !within_rate_limit(&recent, now) {
        return Output::RateLimited;
    }
    recent.push(now);
    if let Err(e) = (IpRateLimitDocPut(IpRateLimitDoc {
        ip: client_ip.clone(),
        recent_requests: recent,
    }))
    .send_with(&db)
    .await
    {
        tracing::error!(?e, "intake_crash IpRateLimitDocPut");
        return Output::Error {
            message: format!("IpRateLimitDocPut: {e}"),
        };
    }

    let existing = match (StackGroupDocGet {
        stack_hash: req.body.stack_hash.clone(),
    })
    .send_with(&db)
    .await
    {
        Ok(g) => g,
        Err(e) => {
            tracing::error!(?e, "intake_crash StackGroupDocGet");
            return Output::Error {
                message: format!("StackGroupDocGet: {e}"),
            };
        }
    };

    let (first_seen, prev_count, mut dump_ids) = match existing {
        Some(g) => (g.first_seen, g.count, g.dump_ids),
        None => (now, 0, Vec::new()),
    };

    let upload = if dump_ids.len() < MAX_DUMPS_PER_GROUP {
        let bytes = rand::get_random_bytes(16).await;
        let Ok(uuid_bytes): Result<[u8; 16], _> = bytes.as_slice().try_into() else {
            return Output::Error {
                message: "rng returned wrong length".to_string(),
            };
        };
        let dump_id = Uuid::from_bytes(uuid_bytes).to_string();
        let r2_key = format!("dump/{}/{}.dmp", req.body.stack_hash, dump_id);

        let bucket = object_storage::bucket();
        let presigned_put_url = match bucket
            .presigned_put_url(&r2_key, Duration::from_secs(PRESIGNED_PUT_EXPIRES_SECS))
            .await
        {
            Ok(u) => u,
            Err(e) => {
                tracing::error!(?e, "intake_crash presigned_put_url");
                return Output::Error {
                    message: format!("presigned_put_url: {e}"),
                };
            }
        };

        if let Err(e) = (DumpDocPut(DumpDoc {
            dump_id: dump_id.clone(),
            stack_hash: req.body.stack_hash.clone(),
            build_id: req.body.build_id.clone(),
            uploaded_at: now,
            r2_key: r2_key.clone(),
            context: req.body.context.clone(),
            client_ip: client_ip.clone(),
        }))
        .send_with(&db)
        .await
        {
            tracing::error!(?e, "intake_crash DumpDocPut");
            return Output::Error {
                message: format!("DumpDocPut: {e}"),
            };
        }

        dump_ids.push(dump_id.clone());
        Some(UploadGrant {
            dump_id,
            presigned_put_url,
        })
    } else {
        None
    };

    let group = StackGroupDoc {
        stack_hash: req.body.stack_hash.clone(),
        first_seen,
        last_seen: now,
        count: prev_count + 1,
        dump_ids,
        latest_context: req.body.context,
    };
    if let Err(e) = StackGroupDocPut(group).send_with(&db).await {
        tracing::error!(?e, "intake_crash StackGroupDocPut");
        return Output::Error {
            message: format!("StackGroupDocPut: {e}"),
        };
    }

    Output::Ok { upload }
}

fn verify_hmac(
    key_hex: &str,
    build_id: &str,
    stack_hash: &str,
    context: &CrashContext,
    signature: &[u8],
) -> bool {
    let Ok(key) = hex::decode(key_hex) else {
        return false;
    };
    let Ok(context_json) = serde_json::to_vec(context) else {
        return false;
    };
    let mut digest = Sha256::new();
    digest.update(&context_json);
    let context_digest = digest.finalize();

    let mut mac = match HmacSha256::new_from_slice(&key) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(build_id.as_bytes());
    mac.update(stack_hash.as_bytes());
    mac.update(&context_digest);
    mac.verify_slice(signature).is_ok()
}

fn extract_client_ip(headers: &::http::HeaderMap) -> Option<String> {
    for name in ["cf-connecting-ip", "x-real-ip", "x-forwarded-for"] {
        let Some(v) = headers.get(name) else { continue };
        let Ok(s) = v.to_str() else { continue };
        let first = s.split(',').next().unwrap_or("").trim();
        if !first.is_empty() {
            return Some(first.to_string());
        }
    }
    None
}

fn prune_old(recent: &mut Vec<DateTime>, now: DateTime) {
    let cutoff = now - chrono::Duration::hours(24);
    recent.retain(|t| *t > cutoff);
}

fn within_rate_limit(recent: &[DateTime], now: DateTime) -> bool {
    let minute_ago = now - chrono::Duration::minutes(1);
    let recent_minute = recent.iter().filter(|t| **t > minute_ago).count();
    if recent_minute >= RATE_LIMIT_PER_MINUTE {
        return false;
    }
    if recent.len() >= RATE_LIMIT_PER_24H {
        return false;
    }
    true
}
