use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub git_root: PathBuf,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/yuxu".into()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "yuxu-dev-secret-change-in-production".into()),
            git_root: PathBuf::from(
                std::env::var("GIT_ROOT").unwrap_or_else(|_| "/tmp/yuxu-repos".into()),
            ),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        }
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
