use entity::{project, task};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use crate::settings::Settings;


pub async fn get_db_connection(settings: &Settings) -> Result<DatabaseConnection, DbErr> {
    let db_opts = settings.database.database_connect();

    let db = Database::connect(db_opts)
        .await
        .expect("Database connection failed");
    Ok(db)
}

pub async fn create_tables(db: DatabaseConnection) -> Result<(), DbErr> {

    let postgres = db.get_database_backend();
    let schema = Schema::new(postgres);

    //create tables from entities
    //db.execute(
    //    postgres.build(&schema.create_enum_from_active_enum::<category::active_enum::Category>()),
    //)
    //.await?;
    db.execute(postgres.build(&schema.create_table_from_entity(project::Entity)))
        .await?;
    db.execute(postgres.build(&schema.create_table_from_entity(task::Entity)))
        .await?;

    Ok(())
}
