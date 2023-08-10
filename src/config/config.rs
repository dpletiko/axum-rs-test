use once_cell::sync::Lazy;
use super::{
    jwt::JwtConfig,
    db::DbConfig
};

#[derive(Clone)]
pub struct Config {
    db: DbConfig,
    jwt: JwtConfig
}

impl Config {
    pub fn init() -> Self {
        Self {
            db: DbConfig::init(),
            jwt: JwtConfig::init(),
        }
    }

    pub fn db(&self) -> &DbConfig { &self.db }
    pub fn jwt(&self) -> &JwtConfig { &self.jwt }
}


pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::init()
});
