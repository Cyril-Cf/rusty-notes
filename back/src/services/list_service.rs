use crate::models::item::Item;
use crate::models::list::{CreateList, List, ListGraphQL, NewList};
use crate::models::list_tag::ListTag;
use crate::schema::items::dsl::{items, list_id as ItemListId};
use crate::schema::list_tags::dsl::{list_id as ListTagListId, list_tags};
use crate::schema::lists::dsl::{id as ListId, lists, user_id as list_user_id};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
) -> Result<List, diesel::result::Error> {
    lists.filter(ListId.eq(list_id)).first::<List>(conn)
}

fn find_all_list_for_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<List>, diesel::result::Error> {
    lists.filter(list_user_id.eq(user_id)).load::<List>(conn)
}

fn enrich_list_with_relations(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list: &mut ListGraphQL,
    add_tags: bool,
    add_items: bool,
) -> Result<(), diesel::result::Error> {
    if add_tags {
        let tags: Vec<ListTag> = list_tags
            .filter(ListTagListId.eq(list.id))
            .load::<ListTag>(conn)?;
        list.tags = tags;
    }
    if add_items {
        let list_items: Vec<Item> = items.filter(ItemListId.eq(list.id)).load::<Item>(conn)?;
        list.items = list_items;
    }
    Ok(())
}

pub fn find_all_list_for_user_with_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<ListGraphQL>, diesel::result::Error> {
    let user_lists = find_all_list_for_user(conn, user_id)?;

    // Prepare the final result
    let mut list_graphqls = Vec::new();

    // Fetch tags and items for each list
    for list in user_lists.iter() {
        let mut current_list = ListGraphQL {
            id: list.id,
            name: list.name.clone(),
            list_type: list.list_type.clone(),
            tags: Vec::new(),
            items: Vec::new(),
        };
        enrich_list_with_relations(conn, &mut current_list, true, false)?;
        list_graphqls.push(current_list);
    }
    Ok(list_graphqls)
}

pub fn find_one_with_items_and_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
) -> Result<ListGraphQL, diesel::result::Error> {
    let list = find_one(conn, list_id)?;
    let mut res = ListGraphQL {
        id: list.id,
        name: list.name,
        list_type: list.list_type,
        tags: Vec::new(),
        items: Vec::new(),
    };
    enrich_list_with_relations(conn, &mut res, true, true);
    Ok(res)
}

pub fn create_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateList,
) -> Result<List, diesel::result::Error> {
    let new_list = NewList {
        id: Uuid::new_v4(),
        name: input.name,
        list_type: input.list_type,
    };
    diesel::insert_into(lists)
        .values(&new_list)
        .get_result::<List>(conn)
}
