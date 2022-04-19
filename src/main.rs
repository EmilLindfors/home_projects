use home_projects::{
    database::{get_db_connection},
    settings::Settings,
    server::serve,
    telemetry::{get_subscriber, init_subscriber},
};
use std::env;
use home_projects::database::create_tables;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("home_project".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    let settings = Settings::new()?;
    let db = get_db_connection(&settings).await?;
    //create_tables(&db).await?;


    //let project = project::ActiveModel {
    //    title: Set("Project Title".to_owned()),
    //    text: Set("Project Description".to_owned()),
    //    ..Default::default()
    //}
    //.insert(&db)
    //.await?;
//
    //task::ActiveModel {
    //    title: Set("Task Title".to_owned()),
    //    text: Set("Task Description".to_owned()),
    //    project_id: Set(Some(project.id)),
    //    ..Default::default()
    //}
    //.insert(&db)
    //.await?;
    serve(settings, db).await?;
    Ok(())
}
