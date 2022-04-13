pub mod active_enum {
    use sea_orm::entity::prelude::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(schema_name = "home_projects", table_name = "active_enum")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
        pub tea: Option<Category>,
    }
    
    #[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
    #[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
    pub enum Category {
        #[sea_orm(string_value = "Work")]
        Work,
        #[sea_orm(string_value = "Home")]
        Home,
    }
    
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}
    
    impl ActiveModelBehavior for ActiveModel {}
}