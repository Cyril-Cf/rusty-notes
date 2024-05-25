use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::schema::notifications;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = notifications)]
#[graphql(description = "A notification")]
pub struct Notification {
    pub id: Uuid,
    pub content: String,
    pub has_been_read: bool,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
}

#[derive(AsChangeset)]
#[diesel(table_name = notifications)]
pub struct NotificationChangeset {
    pub content: Option<String>,
    pub has_been_read: Option<bool>,
    pub notification_type: Option<NotificationType>,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum)]
#[ExistingTypePath = "crate::schema::sql_types::NotificationType"]
pub enum NotificationType {
    #[db_rename = "NEWFRIEND"]
    NewFriend,
    #[db_rename = "NEWLIST"]
    NewList,
}
