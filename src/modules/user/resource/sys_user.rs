use std::borrow::BorrowMut;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::Response;
use jsonwebtoken::{encode, Header};
use rbatis::{field_key, field_name};
use crate::{AppState, global, IntoResponse, pool, SysUser};
use crate::modules::user::{dto, service};
use serde::{Deserialize, Serialize};
use crate::global::Claims;
use chrono::{Duration, Utc};
use axum_macros::debug_handler;
use chrono::Local;
use serde_json::json;
use crate::modules::user::error::AuthError;

pub async fn list(claims: Claims) -> impl IntoResponse {
    // Json(SysUser::select_all(pool!()).await.unwrap())
    Json(claims)
}

pub async fn login(Json(payload): Json<dto::sys_user_dto::AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    let result = SysUser::select_by_username_password(pool!(), &payload.username, &payload.password).await.unwrap();
    if result.is_none() {
        return Err(AuthError::MissingCredentials)
    }
    let exp_time = Local::now() + Duration::days(1);
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: exp_time.timestamp() as usize,
        username: result.unwrap().username
    };
    let token = encode(&Header::default(), &claims, &global::JWT_KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;
    Ok(Json(AuthBody { access_token: token }))
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    pub access_token: String,
}