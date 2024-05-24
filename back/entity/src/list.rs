use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "list_type")]
pub enum ListType {
    #[sea_orm(string_value = "ToDo")]
    ToDo,
    #[sea_orm(string_value = "ToBuy")]
    ToBuy,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Eq)]
#[sea_orm(table_name = "list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_list::Entity")]
    Customerlist,
    #[sea_orm(has_many = "super::item::Entity")]
    Item,
    #[sea_orm(has_many = "super::list_tag_list::Entity")]
    Listtaglist,
}

impl Related<super::customer_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customerlist.def()
    }
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<super::list_tag_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtaglist.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        super::customer_list::Relation::Customer.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::customer_list::Relation::List.def().rev())
    }
}

impl Related<super::list_tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::list_tag_list::Relation::Listtag.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::list_tag_list::Relation::List.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "list"
    }
}
