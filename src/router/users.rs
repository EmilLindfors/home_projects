use crate::{error::{HttpError}, server::Server, Result};
use axum::{
    extract::Extension,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use entity::user;
use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHash, password_hash::SaltString};
use anyhow::Context;

async fn get_user(ref ctx: Extension<Server>, Path(id): Path<Uuid>) -> Result<Json<user::Model>> {
    Ok(Json(
        user::Entity::find_by_id(id)
            .one(&ctx.db)
            .await?
            .ok_or_else(|| HttpError::not_found(None, None))?,
    ))
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GetUsersResponse {
    pub users: Vec<user::Model>,
}

async fn get_users(ref ctx: Extension<Server>) -> Result<Json<GetUsersResponse>> {
    Ok(Json(GetUsersResponse {
        users: user::Entity::find().all(&ctx.db).await?,
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserRes {
    email: String,
    username: String,
    bio: String,
    image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CreateUserRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CreateUserResponse {
    pub success: bool,
}

async fn create_user(
    ref ctx: Extension<Server>,
    Json(req): Json<CreateUserRequest>,
) -> Result<StatusCode> {
    let pass = hash_password(req.password).await?;
    user::ActiveModel {
        username: ActiveValue::Set(req.username.to_owned()),
        email: ActiveValue::Set(req.email.to_owned()),
        //todo fix argon2 hashing
        password_hash: ActiveValue::Set(pass),
        ..Default::default()
    }
    .save(&ctx.db)
    .await
    .map_err(|e| HttpError::bad_request(Some(e.to_string()), None))?;

    Ok(StatusCode::CREATED)
}

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/user/:id", get(get_user))
        .route("/user", post(create_user))
        .route("/users", get(get_users))
}

async fn hash_password(password: String) -> Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")??)
}
