use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "notification_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::notification_notification_type::Entity")]
    Notificationnotificationtype,
}

impl Related<super::notification_notification_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Notificationnotificationtype.def()
    }
}

impl Related<super::notification::Entity> for Entity {
    fn to() -> RelationDef {
        super::notification_notification_type::Relation::Notification.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::notification_notification_type::Relation::Notificationtype
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
