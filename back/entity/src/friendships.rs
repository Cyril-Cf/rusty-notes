use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "friendships")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub id_user1: Uuid,
    pub id_user2: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::IdUser1",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User1,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::IdUser2",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User2,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User1.def()
    }

    fn via() -> Option<RelationDef> {
        Some(Relation::User2.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "friendships"
    }
}
