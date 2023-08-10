use jsonwebtoken::{EncodingKey, DecodingKey};

#[derive(Clone)]
pub struct JwtConfig {
    encoding: EncodingKey,
    decoding: DecodingKey,

    pub ttl: u32,
    pub refresh_ttl: u32,

    secret: String,
}

impl JwtConfig {
    fn generate_keys(secret: &String) -> (EncodingKey, DecodingKey) {
        (EncodingKey::from_secret(secret.as_bytes()), DecodingKey::from_secret(secret.as_bytes()))
    }
    pub fn init() -> Self {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let ttl = std::env::var("JWT_TTL").unwrap().parse::<u32>().unwrap_or(0);
        let refresh_ttl = std::env::var("JWT_REFRESH_TTL").unwrap().parse::<u32>().unwrap_or(0);

        let (encoding, decoding) = JwtConfig::generate_keys(&secret);

        Self {
            secret,

            ttl,
            refresh_ttl,

            encoding,
            decoding
        }
    }

    pub fn encoding(&self) -> &EncodingKey { &self.encoding }
    pub fn decoding(&self) -> &DecodingKey { &self.decoding }
}
