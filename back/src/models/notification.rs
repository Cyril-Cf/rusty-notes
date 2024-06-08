use crate::schema::notifications;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = notifications)]
#[graphql(description = "A notification")]
pub struct Notification {
    pub id: Uuid,
    pub has_been_read: bool,
    pub notif_type: NotifType,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::NotifType"]
pub enum NotifType {
    #[db_rename = "NEW_FRIENDSHIP_DEMAND"]
    NewFriendshipDemand,
    #[db_rename = "NEW_FRIENDSHIP_ACCEPTED"]
    NewFriendshipAccepted,
    #[db_rename = "SHARED_LIST_MODIFIED"]
    SharedListModified,
}

#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub has_been_read: bool,
    pub notif_type: NotifType,
    pub user_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct CreateNotification {
    pub notif_type: NotifType,
    pub user_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct UpdateNotificationGQL {
    pub id: Uuid,
    pub has_been_read: bool,
    pub notif_type: NotifType,
    pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = notifications)]
pub struct UpdateNotification {
    pub id: Option<Uuid>,
    pub has_been_read: Option<bool>,
    pub notif_type: Option<NotifType>,
    pub user_id: Option<Uuid>,
}
