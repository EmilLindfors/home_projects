use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub user_id: Uuid,
    #[sea_orm(unique, case_insensitive)]
    username: String,
    #[sea_orm(unique, case_insensitive)]
    email: String,
    #[sea_orm(default = "")]
    bio: String,
    image: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Project,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Project => Entity::has_many(super::project::Entity).into(),
        }
    }
}

impl Related<super::project::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Project.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let timestamp = Utc::now();
        Self {
            user_id: Set(Uuid::new_v4()),
            created_at: Set(timestamp.into()),
            updated_at: Set(timestamp.into()),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, _insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(Utc::now().into());
        Ok(self)
    }
}

impl Entity {
    pub fn find_by_name(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Username.eq(name))
    }
}
