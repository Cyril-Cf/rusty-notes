use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "list_tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub id_customer: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customer::Entity",
        from = "Column::IdCustomer",
        to = "super::customer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customer,
    #[sea_orm(has_many = "super::list_tag_list::Entity")]
    Listtaglist,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::list_tag_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtaglist.def()
    }
}

impl Related<super::list::Entity> for Entity {
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