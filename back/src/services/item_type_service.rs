use crate::models::item_type::ItemType;
use crate::schema::item_types::dsl::{id, item_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_type_id: Uuid,
) -> Result<Option<ItemType>, diesel::result::Error> {
    match item_types
        .filter(id.eq(list_type_id))
        .first::<ItemType>(conn)
    {
        Ok(item_type) => Ok(Some(item_type)),
        Err(_) => Ok(None),
    }
}
