use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list_type_item_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_list_type: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_item_type: i32,
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
        belongs_to = "super::list_type::Entity",
        from = "Column::IdListType",
        to = "super::list_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Listtype,
}

impl Related<super::item_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Itemtype.def()
    }
}

impl Related<super::list_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtype.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "list_type_item_type"
    }
}