use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notification_notification_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_notification: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_notification_type: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::notification::Entity",
        from = "Column::IdNotification",
        to = "super::notification::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Notification,
    #[sea_orm(
        belongs_to = "super::notification_type::Entity",
        from = "Column::IdNotificationType",
        to = "super::notification_type::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Notificationtype,
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notification.def()
    }
}

impl Related<super::notification_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notificationtype.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
