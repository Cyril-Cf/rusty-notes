use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub has_been_read: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_notification::Entity")]
    Customernotification,
    #[sea_orm(has_many = "super::notification_notification_type::Entity")]
    Notificationnotificationtype,
}

impl Related<super::customer_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customernotification.def()
    }
}

impl Related<super::notification_notification_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notificationnotificationtype.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        super::customer_notification::Relation::Customer.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::customer_notification::Relation::Notification
                .def()
                .rev(),
        )
    }
}

impl Related<super::notification_type::Entity> for Entity {
    fn to() -> RelationDef {
        super::notification_notification_type::Relation::Notificationtype.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::notification_notification_type::Relation::Notification
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
