use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use entity::user;
use serde::{Deserialize, Serialize};
use session::Session;
use time::Duration;
use tracing::error;

use crate::ctx::Context;

#[derive(Deserialize)]
pub(super) struct RegisterRequest {
  name: String,
  email: String,
  display_name: String,
  password: String,
}

#[derive(Serialize)]
pub(super) struct RegisterResponse {
  user_id: String,
}

#[derive(Deserialize)]
pub(super) struct PasswordLoginRequest {
  name: String,
  password: String,
}

#[derive(Serialize)]
pub(super) struct PasswordLoginResponse {
  user_id: String,
}

pub(super) async fn me(
  State(ctx): State<Context>,
  session: Session,
) -> Result<Json<user::Model>, StatusCode> {
  ctx
    .user_service
    .by_id(session.user_id)
    .await
    .map(|user| Json(user.expect("User can not be null")))
    .map_err(|err| {
      error!("Unable to fetch user: {}", err);
      StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub(super) async fn register(
  State(ctx): State<Context>,
  Json(req): Json<RegisterRequest>,
) -> Result<Json<user::Model>, StatusCode> {
  let user = ctx
    .user_service
    .register(
      req.name,
      req.email,
      req.display_name,
      req.password.as_bytes(),
    )
    .await
    .map_err(|err| {
      error!("Unable to register user: {}", err);
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

  Ok(Json(user))
}

pub(super) async fn password_login(
  State(ctx): State<Context>,
  jar: CookieJar,
  Json(req): Json<PasswordLoginRequest>,
) -> Result<(CookieJar, Json<user::Model>), StatusCode> {
  let user = ctx
    .user_service
    .password_auth(&req.name, req.password.as_bytes())
    .await
    .map_err(|err| {
      error!("Unable to verify password: {}", err);
      StatusCode::INTERNAL_SERVER_ERROR
    })?;

  match user {
    None => Err(StatusCode::UNAUTHORIZED),
    Some(user) => {
      let session_id = ctx.session_store.push(user.id).await;

      let cookie = Cookie::build(("session_id", session_id.to_string()))
        .domain("localhost")
        .same_site(SameSite::Strict)
        .path("/api/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(1));

      Ok((jar.add(cookie), Json(user)))
    }
  }
}
