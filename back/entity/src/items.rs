use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(EnumIter, DeriveActiveEnum, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "item_type")]
pub enum ItemType {
    #[sea_orm(string_value = "Checkbox")]
    Checkbox,
    #[sea_orm(string_value = "BulletPoint")]
    BulletPoint,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "items")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub is_checked: bool,
    pub id_list: Uuid,
    pub item_type: ItemType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::lists::Entity",
        from = "Column::IdList",
        to = "super::lists::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::List.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "item"
    }
}
