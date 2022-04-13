use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveValue::Set;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
    pub project_id: Option<Uuid>,
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
            Self::Project => Entity::belongs_to(super::project::Entity)
                .from(Column::ProjectId)
                .to(super::project::Column::Id)
                .into(),
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
