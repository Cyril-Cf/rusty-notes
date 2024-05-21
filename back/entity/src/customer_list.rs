use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "customer_list")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_list: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_customer: i32,
    pub is_owner: bool,
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
    #[sea_orm(
        belongs_to = "super::list::Entity",
        from = "Column::IdList",
        to = "super::list::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    List,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
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
        "customer_list"
    }
}