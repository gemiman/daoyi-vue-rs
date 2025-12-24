use crate::configs::jwt_config::JwtConfig;
use crate::configs::AppConfig;
use jsonwebtoken::{
    get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::OnceCell;

static DEFAULT_JWT: OnceCell<JWT> = OnceCell::const_new();

#[derive(Debug, Clone, Serialize, Default)]
pub struct Principal {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String,
    sub: String,
    aud: String,
    iss: String,
    iat: u64,
    exp: u64,
}

pub struct JWT {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn new(config: &JwtConfig) -> Self {
        let secret = config.secret().as_bytes();
        let encode_secret = EncodingKey::from_secret(secret);
        let decode_secret = DecodingKey::from_secret(secret);
        let header = Header::new(Algorithm::HS256);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience()]);
        validation.set_issuer(&[&config.issuer()]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let expiration = config.expiration();
        let audience = String::from(config.audience());
        let issuer = String::from(config.issuer());

        Self {
            encode_secret,
            decode_secret,
            header,
            validation,
            expiration,
            audience,
            issuer,
        }
    }

    pub async fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!("{}:{}", principal.id, principal.name),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }
    pub async fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims = jsonwebtoken::decode::<Claims>(token, &self.decode_secret, &self.validation)?;
        let sub = claims.claims.sub;
        let mut parts = sub.splitn(2, ':');
        Ok(Principal {
            id: parts.next().unwrap().to_string(),
            name: parts.next().unwrap().to_string(),
        })
    }
}

pub async fn get_default_jwt() -> &'static JWT {
    DEFAULT_JWT
        .get_or_init(async || JWT::new(AppConfig::get().await.jwt()))
        .await
}
