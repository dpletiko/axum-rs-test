use std::fmt::Display;
use async_trait::async_trait;
use axum::{
    body::BoxBody,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, Response, StatusCode},
    Json,
    response::IntoResponse,
    TypedHeader, RequestPartsExt,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use serde_json::json;
use crate::{models::user::User, config::CONFIG};


pub struct Jwt {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // subject
    pub sub: i32,
    // issuer
    pub iss: String,
    // expiration time
    pub exp: usize,
    // issued at
    pub iat: i64
}

impl Jwt {
    pub fn generate(user: User) -> Result<AuthPayload, AuthError> {
        let iat = Utc::now().timestamp();
        // TODO: Use config duration stuff

        let claims = Claims {
            iss: "kazzimir".to_owned(),

            sub: user.id,
            // sub: user.email,

            // Mandatory expiry time as UTC timestamp
            exp: 2000000000, // May 2033

            iat
        };


        let token = encode(
            &Header::default(),
            &claims,
            &CONFIG.jwt().encoding()
        ).map_err(|_| AuthError::TokenCreation)?;


        Ok(AuthPayload {
            access_token: token,
            token_type: "Bearer".to_owned(),
            user
        })
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Claims: {}\n", self)
        // write!(f, "Subject: {}\nExpiry: {}", self.sub, self.exp)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;


        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &CONFIG.jwt().decoding(),
            &Validation::default()
        ).map_err(|_| AuthError::InvalidToken)?;


        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthPayload {
    access_token: String,
    token_type: String,
    user: User
}


#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<BoxBody> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
