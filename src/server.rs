use crate::settings::Settings;
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr};
use std::sync::Arc;
use anyhow::Context;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use tower_http::cors::{Any, CorsLayer, Origin};
use axum::http::Method;
use crate::router::api_router;

#[derive(Clone)]
pub struct Server {
    pub settings: Arc<Settings>,
    pub db: DatabaseConnection,
}

pub async fn serve(settings: Settings, db: DatabaseConnection) -> anyhow::Result<()> {
    let address: SocketAddr = format!(
        "{}:{}",
        settings.server.host, settings.server.port
    ).parse().context("could not parse address")?;

    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(Server {
                settings: Arc::new(settings),
                db
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
                    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any),
            ),
    );
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")

}