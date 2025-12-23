use jsonwebtoken::{
    get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use std::time::Duration;

const DEFAULT_SECRET: &str = r#"2234!QW@#ESDX234GVYBHKJU@234#$WEBHJ@#WSEDRCFrdcftghuyj"#;
static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::default());

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct JwtConfig {
    pub secret: Cow<'static, str>,
    pub expiration: Duration,
    pub audience: String,
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            expiration: Duration::from_secs(60 * 60),
            audience: "audience".to_string(),
            issuer: "issuer".to_string(),
        }
    }
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
    pub fn new(config: JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        let encode_secret = EncodingKey::from_secret(secret);
        let decode_secret = DecodingKey::from_secret(secret);
        let header = Header::new(Algorithm::HS256);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let expiration = config.expiration;
        let audience = config.audience;
        let issuer = config.issuer;

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

    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
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
    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims = jsonwebtoken::decode::<Claims>(token, &self.decode_secret, &self.validation)?;
        let sub = claims.claims.sub;
        let mut parts = sub.splitn(2, ':');
        Ok(Principal {
            id: parts.next().unwrap().to_string(),
            name: parts.next().unwrap().to_string(),
        })
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

pub fn get_default_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
