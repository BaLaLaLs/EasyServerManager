use axum::{async_trait, extract::{FromRequest, RequestParts}, http::StatusCode, response::{IntoResponse, Response}, routing::{get, post}, Json, Router, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, Validation};
use crate::global;
use crate::global::Claims;
use crate::modules::user::error::AuthError;

#[async_trait]
impl<B> FromRequest<B> for Claims
    where
        B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &global::JWT_KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}