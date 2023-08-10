#[derive(Clone)]

pub struct DbConfig {
    url: String
}

impl DbConfig {
    pub fn init() -> Self {
        Self {
            url: std::env::var("DATABASE_URL").expect("Database url not set")
        }
    }

    pub fn url(&self) -> &str { &self.url }
}
