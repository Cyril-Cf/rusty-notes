use crate::graphql_logic::graphql::{DeleteResult, DeleteStatus};
use crate::models::friendship::FriendshipState;
use crate::models::item::Item;
use crate::models::list::{
    AddFriendToMyListStatus, AddListStatus, CreateList, List, ListGraphQL, NewList,
    RemoveFriendFromMyListStatus,
};
use crate::models::list_tag::ListTag;
use crate::models::user::User;
use crate::models::user_list::{
    AddUserToListStatus, NewUserList, RemoveUserFromListStatus, UserList,
};
use crate::schema::items::dsl::{items, list_id as ItemListId};
use crate::schema::list_tags::dsl::{list_id as ListTagListId, list_tags};
use crate::schema::lists::dsl::{id as ListId, lists};
use crate::schema::user_lists::dsl::{
    list_id as UserList_list_id, user_id as UserList_user_id, user_lists,
};
use crate::schema::users::dsl::users;
use crate::services::friendship_service;
use crate::services::user_service;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use juniper::GraphQLObject;
use uuid::Uuid;

#[derive(Debug, GraphQLObject)]
pub struct AddFriendToMyListResult {
    pub status: AddFriendToMyListStatus,
}

#[derive(Debug, GraphQLObject)]
pub struct RemoveFriendFromMyListResult {
    pub status: RemoveFriendFromMyListStatus,
}

#[derive(Debug, GraphQLObject)]
pub struct AddListResult {
    pub status: AddListStatus,
}

fn find_one(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
) -> Result<Option<List>, diesel::result::Error> {
    match lists.filter(ListId.eq(list_id)).first::<List>(conn) {
        Ok(list) => Ok(Some(list)),
        Err(_) => Ok(None),
    }
}
fn find_all_list_for_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<List>, diesel::result::Error> {
    let user = user_service::find_user(conn, user_id)?;
    if user.is_none() {
        return Ok(Vec::new());
    }
    let user = user.unwrap();

    let all_lists: Vec<List> = UserList::belonging_to(&user)
        .inner_join(lists)
        .select(List::as_select())
        .load::<List>(conn)?;
    Ok(all_lists)
}

fn enrich_list_with_relations(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list: &mut ListGraphQL,
    add_tags: bool,
    add_items: bool,
    add_users: bool,
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
    if add_users {
        let list_objet = List {
            id: list.id,
            list_type: list.list_type.clone(),
            name: list.name.clone(),
        };
        let all_users: Vec<User> = UserList::belonging_to(&list_objet)
            .inner_join(users)
            .select(User::as_select())
            .load::<User>(conn)?;
        list.users = all_users;
    }
    Ok(())
}

pub fn find_all_list_for_user_with_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<ListGraphQL>, diesel::result::Error> {
    let all_user_lists = find_all_list_for_user(conn, user_id)?;

    // Prepare the final result
    let mut list_graphqls = Vec::new();

    // Fetch tags and items for each list
    for list in all_user_lists.iter() {
        let mut current_list = ListGraphQL {
            id: list.id,
            name: list.name.clone(),
            list_type: list.list_type.clone(),
            tags: Vec::new(),
            items: Vec::new(),
            users: Vec::new(),
        };
        enrich_list_with_relations(conn, &mut current_list, true, false, true)?;
        list_graphqls.push(current_list);
    }
    Ok(list_graphqls)
}

pub fn find_one_with_items_and_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
) -> Result<Option<ListGraphQL>, diesel::result::Error> {
    let list = find_one(conn, list_id)?;
    if list.is_none() {
        return Ok(None);
    };
    let list = list.unwrap();
    let mut res = ListGraphQL {
        id: list.id,
        name: list.name,
        list_type: list.list_type,
        tags: Vec::new(),
        items: Vec::new(),
        users: Vec::new(),
    };
    enrich_list_with_relations(conn, &mut res, true, true, true)?;
    Ok(Some(res))
}

fn link_list_to_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<AddUserToListStatus, diesel::result::Error> {
    let new_link = NewUserList { user_id, list_id };
    diesel::insert_into(user_lists)
        .values(&new_link)
        .execute(conn)?;
    Ok(AddUserToListStatus::AddSuccessful)
}

fn remove_link_list_to_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<RemoveUserFromListStatus, diesel::result::Error> {
    diesel::delete(
        user_lists
            .filter(UserList_user_id.eq(user_id))
            .filter(UserList_list_id.eq(list_id)),
    )
    .execute(conn)?;
    Ok(RemoveUserFromListStatus::RemoveSuccessful)
}

pub fn create_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateList,
) -> Result<AddListResult, diesel::result::Error> {
    let new_list = NewList {
        id: Uuid::new_v4(),
        name: input.name,
        list_type: input.list_type,
    };
    let list = diesel::insert_into(lists)
        .values(&new_list)
        .get_result::<List>(conn)?;
    match link_list_to_user(conn, list.id, input.user_id)? {
        AddUserToListStatus::ErrCannotAdd => Ok(AddListResult {
            status: AddListStatus::ErrNoUserFound,
        }),
        AddUserToListStatus::AddSuccessful => Ok(AddListResult {
            status: AddListStatus::AddSuccessful,
        }),
    }
}

pub fn delete_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
) -> Result<DeleteResult, diesel::result::Error> {
    diesel::delete(list_tags.filter(ListTagListId.eq(list_id))).execute(conn)?;
    diesel::delete(items.filter(ItemListId.eq(list_id))).execute(conn)?;
    diesel::delete(user_lists.filter(UserList_list_id.eq(list_id))).execute(conn)?;
    let rows_deleted = diesel::delete(lists.filter(ListId.eq(list_id))).execute(conn)? as usize;
    if rows_deleted > 0 {
        Ok(DeleteResult {
            status: DeleteStatus::ResourceDeleted,
        })
    } else {
        Ok(DeleteResult {
            status: DeleteStatus::NoDeletion,
        })
    }
}

pub fn invite_user_to_your_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    friend_id: Uuid,
) -> Result<AddFriendToMyListResult, diesel::result::Error> {
    let main_user = user_service::find_user(conn, user_id)?;
    let friend_user = user_service::find_user(conn, friend_id)?;
    if main_user.is_none() {
        return Ok(AddFriendToMyListResult {
            status: AddFriendToMyListStatus::ErrNoUserFound,
        });
    }
    if friend_user.is_none() {
        return Ok(AddFriendToMyListResult {
            status: AddFriendToMyListStatus::ErrNoUserFriendFound,
        });
    }
    match friendship_service::does_friendship_exists(conn, user_id, friend_id)? {
        FriendshipState::DoesNotExist | FriendshipState::ExistsButNotValidate => {
            Ok(AddFriendToMyListResult {
                status: AddFriendToMyListStatus::ErrNoFriendshipFound,
            })
        }
        FriendshipState::ExistsAndValidated => {
            let list = find_one(conn, list_id)?;
            if list.is_none() {
                return Ok(AddFriendToMyListResult {
                    status: AddFriendToMyListStatus::ErrNoListFound,
                });
            }
            match link_list_to_user(conn, list_id, friend_id)? {
                AddUserToListStatus::ErrCannotAdd => Ok(AddFriendToMyListResult {
                    status: AddFriendToMyListStatus::ErrServerIssue,
                }),
                AddUserToListStatus::AddSuccessful => Ok(AddFriendToMyListResult {
                    status: AddFriendToMyListStatus::AddSuccessful,
                }),
            }
        }
    }
}

pub fn remove_user_from_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    friend_id: Uuid,
) -> Result<RemoveFriendFromMyListResult, diesel::result::Error> {
    let friend_user = user_service::find_user(conn, friend_id)?;
    if friend_user.is_none() {
        return Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::ErrNoUserFriendFound,
        });
    }
    let list = find_one(conn, list_id)?;
    if list.is_none() {
        return Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::ErrNoListFound,
        });
    }
    match remove_link_list_to_user(conn, list_id, friend_id)? {
        RemoveUserFromListStatus::ErrCannotRemove => Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::ErrServerIssue,
        }),
        RemoveUserFromListStatus::RemoveSuccessful => Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::RemoveSuccessful,
        }),
    }
}
