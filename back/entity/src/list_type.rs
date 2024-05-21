use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::list::Entity")]
    List,
    #[sea_orm(has_many = "super::list_type_item_type::Entity")]
    Listtypeitemtype,
}

impl Related<super::list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl Related<super::list_type_item_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtypeitemtype.def()
    }
}

impl Related<super::item_type::Entity> for Entity {
    fn to() -> RelationDef {
        super::list_type_item_type::Relation::Itemtype.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::list_type_item_type::Relation::Listtype.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "list_type"
    }
}