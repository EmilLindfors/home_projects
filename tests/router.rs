#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use entity::{project, task};
    use home_projects::router::api_router;
    use home_projects::{server::Server, settings::Settings};
    use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema, ActiveModelTrait};
    use sea_orm::ActiveValue::Set;
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tower::ServiceBuilder;
    use tower::ServiceExt;
    use tower_http::add_extension::AddExtensionLayer;

    async fn setup_tests() -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Database connection failed");
        let sqlite = db.get_database_backend();
        let schema = Schema::new(sqlite);

        //create tables from entities
        db.execute(sqlite.build(&schema.create_table_from_entity(project::Entity)))
            .await?;
        db.execute(sqlite.build(&schema.create_table_from_entity(task::Entity)))
            .await?;

        Ok(db)
    }

    #[tokio::test]
    async fn create_project() -> anyhow::Result<()> {
        let settings = Settings::new()?;
        let db = setup_tests().await?;

        let app = api_router().layer(ServiceBuilder::new().layer(AddExtensionLayer::new(Server {
            settings: Arc::new(settings),
            db,
        })));
        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/projects/")
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        json!({
                            "title": "test",
                            "text": "test",
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await?;

        println!("{:?}", response);

        assert_eq!(response.status(), StatusCode::CREATED);

        Ok(())
    }

    #[tokio::test]
    async fn get_projects() -> anyhow::Result<()> {
        let settings = Settings::new()?;
        let db = setup_tests().await?;

        let app = api_router().layer(ServiceBuilder::new().layer(AddExtensionLayer::new(Server {
            settings: Arc::new(settings),
            db,
        })));

        let db = setup_tests().await?;

        let project = project::ActiveModel {
            title: Set("Project Title".to_owned()),
            text: Set("Project Description".to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/projects/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!(project));

        Ok(())
    }
}
