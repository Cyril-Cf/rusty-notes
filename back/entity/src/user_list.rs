use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_list: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_user: Uuid,
    pub is_owner: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::IdUser",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::lists::Entity",
        from = "Column::IdList",
        to = "super::lists::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "customer_list"
    }
}
