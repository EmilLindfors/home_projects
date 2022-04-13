use axum::Router;
mod projects;
mod users;



pub fn api_router() -> Router {
    // This is the order that the modules were authored in.
    projects::router()
       .merge(users::router())
}

