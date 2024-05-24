use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "customer_notification")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_customer: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_notification: Uuid,
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
        belongs_to = "super::notification::Entity",
        from = "Column::IdNotification",
        to = "super::notification::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Notification,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notification.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "customer_notification"
    }
}
