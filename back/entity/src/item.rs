use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub is_checked: bool,
    pub id_item_type: i32,
    pub id_list: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::item_type::Entity",
        from = "Column::IdItemType",
        to = "super::item_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Itemtype,
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::IdList",
        to = "super::list::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
}

impl Related<super::item_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Itemtype.def()
    }
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