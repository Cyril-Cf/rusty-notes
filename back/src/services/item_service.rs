use crate::graphql_logic::graphql::{DeleteResult, DeleteStatus, UpdateResult, UpdateStatus};
use crate::models::item::{CreateItem, Item, NewItem, UpdateItem};
use crate::schema::items::dsl::{id, items};
use crate::services::list_service;
use crate::{MessageType, NotificationServer, SendFriendshipNotification};
use actix::Addr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item_id: Uuid,
) -> Result<Option<Item>, diesel::result::Error> {
    match items.filter(id.eq(item_id)).first::<Item>(conn) {
        Ok(item) => Ok(Some(item)),
        Err(_) => Ok(None),
    }
}

pub fn create_item(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateItem,
    notification_server: &Addr<NotificationServer>,
) -> Result<Item, diesel::result::Error> {
    let new_item = NewItem {
        id: Uuid::new_v4(),
        name: input.name,
        is_checked: false,
        item_type: input.item_type,
        list_id: input.list_id,
    };
    let item = diesel::insert_into(items)
        .values(&new_item)
        .get_result::<Item>(conn)?;
    if let Some(list) = list_service::find_one(conn, input.list_id)? {
        let users = list_service::get_users_for_list(conn, list)?;
        for user in users {
            notification_server.do_send(SendFriendshipNotification {
                user_id: user.id,
                message: MessageType::RefreshSelectedList,
            });
        }
    };
    Ok(item)
}

pub fn delete_item(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    item_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<DeleteResult, diesel::result::Error> {
    match find_one(conn, item_id)? {
        Some(item) => {
            let rows_deleted = diesel::delete(items.filter(id.eq(item_id))).execute(conn)? as usize;
            if rows_deleted > 0 {
                if let Some(list) = list_service::find_one(conn, item.list_id)? {
                    let users = list_service::get_users_for_list(conn, list)?;
                    for user in users {
                        notification_server.do_send(SendFriendshipNotification {
                            user_id: user.id,
                            message: MessageType::RefreshSelectedList,
                        });
                    }
                };
                Ok(DeleteResult {
                    status: DeleteStatus::ResourceDeleted,
                })
            } else {
                Ok(DeleteResult {
                    status: DeleteStatus::NoDeletion,
                })
            }
        }
        None => Ok(DeleteResult {
            status: DeleteStatus::NoDeletion,
        }),
    }
}

pub fn update_item(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: UpdateItem,
    notification_server: &Addr<NotificationServer>,
) -> Result<UpdateResult, diesel::result::Error> {
    let existing_item = find_one(conn, input.id)?;
    if existing_item.is_none() {
        return Ok(UpdateResult {
            status: UpdateStatus::NoUpdate,
        });
    }
    let existing_item = existing_item.unwrap();
    if existing_item.name != input.name
        || existing_item.is_checked != input.is_checked
        || existing_item.item_type != input.item_type
    {
        let new_item = NewItem {
            id: input.id,
            name: input.name,
            is_checked: input.is_checked,
            list_id: input.list_id,
            item_type: input.item_type,
        };
        diesel::update(items.filter(id.eq(input.id)))
            .set(&new_item)
            .execute(conn)?;
        if let Some(list) = list_service::find_one(conn, input.list_id)? {
            let users = list_service::get_users_for_list(conn, list)?;
            for user in users {
                notification_server.do_send(SendFriendshipNotification {
                    user_id: user.id,
                    message: MessageType::RefreshSelectedList,
                });
            }
        };
        Ok(UpdateResult {
            status: UpdateStatus::ResourceUpdated,
        })
    } else {
        Ok(UpdateResult {
            status: UpdateStatus::NoUpdate,
        })
    }
}
