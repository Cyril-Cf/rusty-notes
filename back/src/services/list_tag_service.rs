use crate::models::list_tag::{CreateListTag, ListTag, NewListTag};
use crate::schema::list_tags::dsl::list_tags;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn create_list_tag(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateListTag,
) -> Result<ListTag, diesel::result::Error> {
    let new_list_tag = NewListTag {
        id: Uuid::new_v4(),
        name: input.name,
        list_id: input.list_id,
    };
    diesel::insert_into(list_tags)
        .values(&new_list_tag)
        .get_result::<ListTag>(conn)
}
