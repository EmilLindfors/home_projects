#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use entity::{project, task};
    use sea_orm::{
        ActiveModelTrait, ColumnTrait, ConnectionTrait, Database, DatabaseConnection, DbErr,
        EntityTrait, QueryFilter, Schema, Set, Statement,
    };
    use tokio_stream::{ StreamExt};

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
    async fn create_a_sea_orm_table() -> Result<(), DbErr> {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Database connection failed");
        let sqlite = db.get_database_backend();
        let schema = Schema::new(sqlite);

        // Create a table from the entity
        assert_eq!(
            sqlite.build(&schema.create_table_from_entity(project::Entity)),
            Statement::from_string(
                sqlite,
                vec![
                    r#"CREATE TABLE "project" ("#,
                    r#""id" text(36) NOT NULL PRIMARY KEY,"#,
                    r#""title" text NOT NULL,"#,
                    r#""text" text NOT NULL,"#,
                    r#""created_at" text NOT NULL,"#,
                    r#""updated_at" text NOT NULL"#,
                    r#")"#,
                ]
                .join(" ")
            )
        );

        Ok(())
    }

    #[tokio::test]
    async fn insert_into_tables() -> Result<(), DbErr> {
        let db = setup_tests().await?;

        let project_insert_res = project::ActiveModel {
            title: Set("Hello World".to_owned()),
            text: Set("Hello World".to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .expect("could not insert baker");

        let task_insert_res = task::ActiveModel {
            title: Set("Hello World".to_owned()),
            text: Set("Task Text".to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await
        .expect("could not insert baker");

        //assert that the id was auto-set
        assert!(!(task_insert_res.id).is_nil());

        //assert that the created_at was auto-set
        assert_eq!(
            (task_insert_res.created_at).minute(),
            chrono::Utc::now().minute()
        );

        assert_eq!(project_insert_res.title, "Hello World");
        assert_eq!(task_insert_res.text, "Task Text");

        Ok(())
    }
    #[tokio::test]
    async fn select_from_tables() -> Result<(), DbErr> {
        let db = setup_tests().await?;

        let project = project::ActiveModel {
            title: Set("Project Title".to_owned()),
            text: Set("Project Description".to_owned()),
            ..Default::default()
        }
        .insert(&db)
        .await?;

        task::ActiveModel {
            title: Set("Task Title".to_owned()),
            text: Set("Task Description".to_owned()),
            project_id: Set(Some(project.id)),
            ..Default::default()
        }
        .insert(&db)
        .await?;

        let select_project = project::Entity::find_by_title("Project Title")
            .find_also_related(task::Entity)
            .one(&db)
            .await?;
        let select_task = project::Entity::find_by_title("Project Title")
            .find_also_related(task::Entity)
            .one(&db)
            .await?;

        assert_eq!(&select_project.unwrap().0.text, "Project Description");
        assert_eq!(select_task.unwrap().1.unwrap().title, "Task Title");

        Ok(())
    }
    #[tokio::test]
    async fn stream_from_tables() -> Result<(), DbErr> {
        let db = setup_tests().await?;


        let mut stream = project::Entity::find()
            .filter(project::Column::Title.contains("Project Title"))
            .stream(&db)
            .await?;
        while let Some(value) = stream.next().await {
            assert_eq!(value.unwrap().text, "Project Description");
        }

        Ok(())
    }
}
