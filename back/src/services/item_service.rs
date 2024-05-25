use crate::models::item::{CreateItem, Item, NewItem};
use crate::schema::items::dsl::items;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn create_item(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateItem,
) -> Result<Item, diesel::result::Error> {
    let new_item = NewItem {
        id: Uuid::new_v4(),
        name: input.name,
        is_checked: false,
        item_type: input.item_type,
        list_id: input.list_id,
    };
    diesel::insert_into(items)
        .values(&new_item)
        .get_result::<Item>(conn)
}
