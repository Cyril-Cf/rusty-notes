use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub id_list_type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_list::Entity")]
    Customerlist,
    #[sea_orm(has_many = "super::item::Entity")]
    Item,
    #[sea_orm(has_many = "super::list_tag_list::Entity")]
    Listtaglist,
    #[sea_orm(
        belongs_to = "super::list_type::Entity",
        from = "Column::IdListType",
        to = "super::list_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Listtype,
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

impl Related<super::list_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtype.def()
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
