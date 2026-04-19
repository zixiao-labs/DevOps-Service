use anyhow::{Context, Result, bail};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Config {
    pub bind: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_ttl_seconds: i64,
    pub live_kit_url: String,
    pub github_client_id: Option<String>,
    pub github_client_secret: Option<String>,
    pub cors_allowed_origins: Option<String>,
}

// Custom Debug so `tracing::debug!(?config)` or `format!("{:?}", state)` never
// leaks secrets into logs. Fields with credential-shaped content (jwt_secret,
// database_url which may embed a password, github_client_secret) are elided.
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("bind", &self.bind)
            .field("database_url", &"<redacted>")
            .field("jwt_secret", &"<redacted>")
            .field("jwt_ttl_seconds", &self.jwt_ttl_seconds)
            .field("live_kit_url", &self.live_kit_url)
            .field("github_client_id", &self.github_client_id)
            .field(
                "github_client_secret",
                &self.github_client_secret.as_ref().map(|_| "<redacted>"),
            )
            .field("cors_allowed_origins", &self.cors_allowed_origins)
            .finish()
    }
}

/// Read an env var, returning `Some(trimmed)` only when the value contains at
/// least one non-whitespace character. A whitespace-only value is treated as
/// unset so operators don't get silent misbehaviour from stray spaces in
/// `.env` files.
fn env_nonempty_trimmed(key: &str) -> Option<String> {
    std::env::var(key).ok().and_then(|s| {
        let t = s.trim();
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    })
}

impl Config {
    /// Load configuration from environment variables. Fails fast on missing or
    /// malformed values rather than falling back to unsafe defaults (e.g. a
    /// shared dev JWT secret) that would produce silent misbehaviour in
    /// production.
    pub fn from_env() -> Result<Self> {
        let bind_raw = std::env::var("YUXU_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
        let bind: SocketAddr = bind_raw
            .parse()
            .with_context(|| format!("YUXU_BIND is not a valid SocketAddr: {bind_raw}"))?;

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            if cfg!(feature = "postgres") {
                "postgres://postgres:postgres@localhost/yuxu".into()
            } else {
                "sqlite://yuxu.db?mode=rwc".into()
            }
        });

        // JWT secret is required; we never want to accept a built-in fallback,
        // which would let every deployment forge tokens with the same key.
        // `YUXU_DEV_MODE=1` opts in to an ephemeral random secret for local dev.
        let jwt_secret = match std::env::var("YUXU_JWT_SECRET") {
            Ok(s) if s.len() >= 32 => s,
            Ok(_) => bail!("YUXU_JWT_SECRET must be at least 32 bytes"),
            Err(_) => {
                if std::env::var("YUXU_DEV_MODE").ok().as_deref() == Some("1") {
                    use rand::RngCore;
                    let mut buf = [0u8; 48];
                    rand::thread_rng().fill_bytes(&mut buf);
                    use base64::{Engine as _, engine::general_purpose};
                    tracing::warn!(
                        "YUXU_DEV_MODE=1: generated ephemeral JWT secret; tokens won't survive restart"
                    );
                    general_purpose::STANDARD_NO_PAD.encode(buf)
                } else {
                    bail!(
                        "YUXU_JWT_SECRET is required (>=32 bytes); set YUXU_DEV_MODE=1 to auto-generate one for local development"
                    );
                }
            }
        };

        let jwt_ttl_seconds: i64 = match std::env::var("YUXU_JWT_TTL_SECS") {
            Ok(v) => v
                .parse()
                .with_context(|| format!("YUXU_JWT_TTL_SECS must be a positive integer: {v}"))?,
            Err(_) => 60 * 60 * 24,
        };
        if jwt_ttl_seconds <= 0 {
            bail!("YUXU_JWT_TTL_SECS must be positive");
        }

        let live_kit_url = std::env::var("YUXU_LIVEKIT_URL").unwrap_or_default();

        // GitHub OAuth is optional; both pieces must be present together for
        // the /api/auth/github/callback endpoint to function.
        let github_client_id = env_nonempty_trimmed("GITHUB_CLIENT_ID");
        let github_client_secret = env_nonempty_trimmed("GITHUB_CLIENT_SECRET");
        if github_client_id.is_some() != github_client_secret.is_some() {
            bail!(
                "GITHUB_CLIENT_ID and GITHUB_CLIENT_SECRET must be set together (or both unset to disable OAuth)"
            );
        }

        Ok(Self {
            bind,
            database_url,
            jwt_secret,
            jwt_ttl_seconds,
            live_kit_url,
            github_client_id,
            github_client_secret,
            cors_allowed_origins: env_nonempty_trimmed("YUXU_CORS_ORIGINS"),
        })
    }
}
