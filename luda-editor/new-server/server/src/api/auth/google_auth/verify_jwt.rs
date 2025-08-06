use anyhow::Result;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use std::{
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

#[derive(Debug, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
}

pub struct GoogleJwksClient {
    client_id: String,
    client: reqwest::Client,
}

impl GoogleJwksClient {
    pub fn new(client_id: String) -> Self {
        Self {
            client_id,
            client: reqwest::Client::new(),
        }
    }

    pub async fn verify(&self, jwt: impl AsRef<str>) -> Result<Claims> {
        let jwt = jwt.as_ref();

        struct Data {
            jwks: Arc<GoogleJwks>,
            expires: Instant,
        }
        static LOCK: OnceLock<Mutex<Option<Data>>> = OnceLock::new();

        let jwks = {
            let mut lock = LOCK.get_or_init(Default::default).lock().await;

            let need_refresh = match lock.as_ref() {
                Some(data) => data.expires < Instant::now(),
                None => true,
            };

            if need_refresh {
                let response = self
                    .client
                    .get("https://www.googleapis.com/oauth2/v3/certs")
                    .send()
                    .await?;

                let cache_control = response.headers().get("cache-control").unwrap();
                let max_age = cache_control
                    .to_str()?
                    .split(", ")
                    .find(|s| s.starts_with("max-age="))
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .parse::<u64>()?;
                let expires = Instant::now() + Duration::from_secs(max_age);

                let jwks = response.json::<GoogleJwks>().await?;

                *lock = Some(Data {
                    jwks: jwks.into(),
                    expires,
                });
            }

            lock.as_ref().unwrap().jwks.clone()
        };

        let header = decode_header(jwt)?;

        for jwk in &jwks.keys {
            if Some(&jwk.kid) != header.kid.as_ref() {
                continue;
            }

            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[&self.client_id]);
            validation.validate_aud = true;
            validation.validate_exp = true;
            validation.validate_nbf = true;

            let token = decode::<Claims>(
                jwt,
                &DecodingKey::from_rsa_components(&jwk.n, &jwk.e)?,
                &validation,
            )?;

            return Ok(token.claims);
        }

        Err(anyhow::anyhow!("No valid key found"))
    }
}

#[derive(serde::Deserialize)]
struct GoogleJwks {
    keys: Vec<GoogleJwk>,
}

#[derive(serde::Deserialize)]
struct GoogleJwk {
    kid: String,
    // alg: String,
    e: String,
    n: String,
}
