use crate::{error::HttpError, server::Server, Result};
use axum::{
    extract::Extension,
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use entity::{project, task};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn router() -> Router {
    Router::new()
        .route("/project/:id", get(get_project))
        .route("/projects/", get(get_projects))
        .route("/projects/", post(create_project))
}

#[derive(Serialize, Debug)]
#[serde(default)]
pub struct GetProjectResponse {
    pub project: project::Model,
    pub tasks: Vec<task::Model>,
}

async fn get_project(
    ref ctx: Extension<Server>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetProjectResponse>> {
    let (project, task) = project::Entity::find_by_id(id)
        .find_also_related(task::Entity)
        .one(&ctx.db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    Ok(Json(if task.is_some() {
        GetProjectResponse {
            project: project,
            tasks: vec![task.unwrap()],
        }
    } else {
        GetProjectResponse {
            project: project,
            tasks: Vec::new(),
        }
    }))
}

async fn get_projects(ref ctx: Extension<Server>) -> Result<Json<Vec<GetProjectResponse>>> {
    let res = project::Entity::find()
        .find_also_related(task::Entity)
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|(project, tasks)| {
            if tasks.is_some() {
                GetProjectResponse {
                    project: project,
                    tasks: vec![tasks.unwrap()],
                }
            } else {
                GetProjectResponse {
                    project: project,
                    tasks: Vec::new(),
                }
            }
        })
        .collect();

    Ok(Json(res))
}

pub trait ModelIn {
    type ActiveModel;

    fn update_model(self, model: &mut Self::ActiveModel);
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProjectRequest {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub text: String,
}

#[tracing::instrument(
    name = "Creating a new project",
    skip(ctx),
    fields(
        title = %data.title,
        text = %data.text
    )
)]
async fn create_project(
    ref ctx: Extension<Server>,
    data: Json<ProjectRequest>,
) -> Result<StatusCode> {
    project::ActiveModel {
        title: ActiveValue::Set(data.title.to_owned()),
        text: ActiveValue::Set(data.text.to_owned()),
        ..Default::default()
    }
    .insert(&ctx.db)
    .await
    .map_err(|e| HttpError::bad_request(Some(e.to_string()), None))?;

    Ok(StatusCode::CREATED)
}
