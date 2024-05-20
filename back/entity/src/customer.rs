use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub keycloak_id: String,
    pub id_customer: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::IdCustomer",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::customer_list::Entity")]
    Customerlist,
    #[sea_orm(has_many = "super::customer_notification::Entity")]
    Customernotification,
    #[sea_orm(has_many = "super::list_tag::Entity")]
    Listtag,
}

impl Related<super::customer_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customerlist.def()
    }
}

impl Related<super::customer_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customernotification.def()
    }
}

impl Related<super::list_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Listtag.def()
    }
}

impl Related<super::list::Entity> for Entity {
    fn to() -> RelationDef {
        super::customer_list::Relation::List.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::customer_list::Relation::Customer.def().rev())
    }
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        super::customer_notification::Relation::Notification.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::customer_notification::Relation::Customer.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
