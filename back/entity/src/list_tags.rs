use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub id_user: Uuid,
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
    #[sea_orm(has_many = "super::list_tag_list::Entity")]
    Listtaglist,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::list_tag_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtaglist.def()
    }
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        super::list_tag_list::Relation::List.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::list_tag_list::Relation::Listtag.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "list_tag"
    }
}
