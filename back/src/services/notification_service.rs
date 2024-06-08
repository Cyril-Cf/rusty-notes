use crate::graphql_logic::graphql::{UpdateResult, UpdateStatus};
use crate::models::notification::{
    CreateNotification, NewNotification, Notification, UpdateNotification, UpdateNotificationGQL,
};
use crate::schema::notifications::dsl::{id, notifications, user_id as notifications_user_id};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    notification_id: Uuid,
) -> Result<Option<Notification>, diesel::result::Error> {
    match notifications
        .filter(id.eq(notification_id))
        .first::<Notification>(conn)
    {
        Ok(notif) => Ok(Some(notif)),
        Err(_) => Ok(None),
    }
}

pub fn find_all_notifications_for_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<Notification>, diesel::result::Error> {
    notifications
        .filter(notifications_user_id.eq(user_id))
        .load::<Notification>(conn)
}

pub fn create_notification(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateNotification,
) -> Result<Notification, diesel::result::Error> {
    let new_notification = NewNotification {
        has_been_read: false,
        notif_type: input.notif_type,
        user_id: input.user_id,
    };
    diesel::insert_into(notifications)
        .values(&new_notification)
        .get_result::<Notification>(conn)
}

pub fn update_notification(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: UpdateNotificationGQL,
) -> Result<UpdateResult, diesel::result::Error> {
    match find_one(conn, input.id)? {
        Some(_) => {
            let target = notifications.filter(id.eq(input.id));
            diesel::update(target)
                .set(&UpdateNotification {
                    id: None,
                    has_been_read: Some(input.has_been_read),
                    notif_type: None,
                    user_id: None,
                })
                .execute(conn)?;
            Ok(UpdateResult {
                status: UpdateStatus::ResourceUpdated,
            })
        }
        None => Ok(UpdateResult {
            status: UpdateStatus::NoUpdate,
        }),
    }
}
