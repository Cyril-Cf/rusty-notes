use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_notification")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_user: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id_notification: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::IdUser",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::notifications::Entity",
        from = "Column::IdNotification",
        to = "super::notifications::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Notification,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::notifications::Entity> for Entity {
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
