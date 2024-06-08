use crate::models::allowed_items::AllowedItem;
use crate::models::item_type::ItemType;
use crate::models::list_type::{ListType, ListTypeGraphQL};
use crate::schema::item_types::dsl::item_types;
use crate::schema::list_types::dsl::{id, list_types};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_type_id: Uuid,
) -> Result<Option<ListType>, diesel::result::Error> {
    match list_types
        .filter(id.eq(list_type_id))
        .first::<ListType>(conn)
    {
        Ok(list_type) => Ok(Some(list_type)),
        Err(_) => Ok(None),
    }
}

pub fn get_allowed_items_for_one_list_type(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_type_id: Uuid,
) -> Result<Vec<ItemType>, diesel::result::Error> {
    if let Some(list_type) = find_one(conn, list_type_id)? {
        let vec: Vec<ItemType> = AllowedItem::belonging_to(&list_type)
            .inner_join(item_types)
            .select(ItemType::as_select())
            .load::<ItemType>(conn)?;
        Ok(vec)
    } else {
        Ok(Vec::new())
    }
}

pub fn find_all_with_item_types(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<ListTypeGraphQL>, diesel::result::Error> {
    let list_types_data: Vec<ListType> = list_types.load::<ListType>(conn)?;
    let mut res: Vec<ListTypeGraphQL> = Vec::new();
    for list in list_types_data {
        let allowed_item_types = get_allowed_items_for_one_list_type(conn, list.id)?;
        res.push(ListTypeGraphQL {
            allowed_item_types,
            description: list.description,
            id: list.id,
            name: list.name,
        })
    }
    Ok(res)
}
