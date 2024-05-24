use sea_orm::entity::prelude::*;

#[derive(EnumIter, DeriveActiveEnum, Debug, Clone, PartialEq, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "notification_type")]
pub enum NotificationType {
    #[sea_orm(string_value = "NewFriend")]
    NewFriend,
    #[sea_orm(string_value = "NewList")]
    NewList,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notification")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: String,
    pub has_been_read: bool,
    pub notification_type: NotificationType
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_notification::Entity")]
    Customernotification,
}

impl Related<super::customer_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customernotification.def()
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

impl ActiveModelBehavior for ActiveModel {}

impl AsRef<str> for Entity {
    fn as_ref(&self) -> &str {
        "notification"
    }
}