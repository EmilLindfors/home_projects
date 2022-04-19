use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub user_id: Option<String>
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Task,
    User
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Task => Entity::has_many(super::task::Entity).into(),
            Self::User => Entity::belongs_to(super::user::Entity)
            .from(Column::UserId)
            .to(super::user::Column::UserId)
            .into(),
        }
    }
}

impl Related<super::task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}



impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let timestamp = Utc::now();
        Self {
            id: Set(Uuid::new_v4()),
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
    pub fn find_by_title(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(name))
    }
}
