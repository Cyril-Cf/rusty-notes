use sea_orm::entity::prelude::*;

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "item_type")]
pub enum ItemType {
    #[sea_orm(string_value = "Checkbox")]
    Checkbox,
    #[sea_orm(string_value = "BulletPoint")]
    BulletPoint,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub is_checked: bool,
    pub id_list: i32,
    pub item_type: ItemType
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::IdList",
        to = "super::list::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
}


impl Related<super::list::Entity> for Entity {
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