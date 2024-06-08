use crate::graphql_logic::graphql::{DeleteResult, DeleteStatus};
use crate::models::friendship::FriendshipState;
use crate::models::item::Item;
use crate::models::list::{
    AcceptListInvitationStatus, AddFriendToMyListStatus, AddListStatus, CreateList, List,
    ListGraphQL, NewList, RefuseListInvitationStatus, RemoveFriendFromMyListStatus,
    UserListGraphQL,
};
use crate::models::list_tag::ListTag;
use crate::models::user::User;
use crate::models::user_list::{
    AddUserToListStatus, ListPermission, NewUserList, RemoveUserFromListStatus, UserList,
};
use crate::schema::items::dsl::{items, list_id as ItemListId};
use crate::schema::list_tags::dsl::{list_id as ListTagListId, list_tags};
use crate::schema::lists::dsl::{id as ListId, lists};
use crate::schema::user_lists::dsl::{
    is_validated, list_id as UserList_list_id, user_id as UserList_user_id, user_lists,
};
use crate::schema::users::dsl::users;
use crate::services::friendship_service;
use crate::services::user_service;
use crate::web_socket_logic::web_socket::{
    MessageType, NotificationServer, SendFriendshipNotification,
};
use actix::Addr;
use chrono::{DateTime, Utc};
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

#[derive(Debug, GraphQLObject)]
pub struct AcceptListInvitationResult {
    pub status: AcceptListInvitationStatus,
}

#[derive(Debug, GraphQLObject)]
pub struct RefuseListInvitationResult {
    pub status: RefuseListInvitationStatus,
}

pub fn find_one(
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
        let now: DateTime<Utc> = Utc::now();
        let list_objet = List {
            id: list.id,
            list_type: list.list_type.clone(),
            name: list.name.clone(),
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        };
        let mut user_validated: Vec<UserListGraphQL> = Vec::new();
        let mut users_awaiting_validation: Vec<UserListGraphQL> = Vec::new();
        for user in get_users_for_list(conn, list_objet.clone(), true)? {
            if let Some(user_list) = get_list_user_association(conn, list.id, user.id)? {
                user_validated.push(UserListGraphQL {
                    list_permission: user_list.list_permission,
                    email: user.email,
                    firstname: user.firstname,
                    id: user.id,
                    keycloak_uuid: user.keycloak_uuid,
                    lastname: user.lastname,
                    created_at: now.naive_utc(),
                    updated_at: now.naive_utc(),
                });
            }
        }
        list.users_validated = user_validated;
        for user in get_users_for_list(conn, list_objet, false)? {
            if let Some(user_list) = get_list_user_association(conn, list.id, user.id)? {
                users_awaiting_validation.push(UserListGraphQL {
                    list_permission: user_list.list_permission,
                    email: user.email,
                    firstname: user.firstname,
                    id: user.id,
                    keycloak_uuid: user.keycloak_uuid,
                    lastname: user.lastname,
                    created_at: now.naive_utc(),
                    updated_at: now.naive_utc(),
                });
            }
        }
        list.users_awaiting_validation = users_awaiting_validation;
    }
    Ok(())
}

pub fn get_users_for_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list: List,
    validated: bool,
) -> Result<Vec<User>, diesel::result::Error> {
    UserList::belonging_to(&list)
        .filter(is_validated.eq(validated))
        .inner_join(users)
        .select(User::as_select())
        .load::<User>(conn)
}

pub fn find_all_list_for_user_with_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<ListGraphQL>, diesel::result::Error> {
    let all_user_lists = find_all_list_for_user(conn, user_id)?;

    // Prepare the final result
    let mut list_graphqls = Vec::new();
    let now: DateTime<Utc> = Utc::now();
    // Fetch tags and items for each list
    for list in all_user_lists.iter() {
        let association_table_values = get_list_user_association(conn, list.id, user_id)?.unwrap();
        let mut current_list = ListGraphQL {
            id: list.id,
            name: list.name.clone(),
            list_type: list.list_type.clone(),
            is_owner: association_table_values.is_owner,
            is_validated: association_table_values.is_validated,
            list_permission: association_table_values.list_permission,
            tags: Vec::new(),
            items: Vec::new(),
            users_awaiting_validation: Vec::new(),
            users_validated: Vec::new(),
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        };
        enrich_list_with_relations(conn, &mut current_list, true, false, true)?;
        list_graphqls.push(current_list);
    }
    Ok(list_graphqls)
}

pub fn find_one_with_items_and_tags(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<Option<ListGraphQL>, diesel::result::Error> {
    let list = find_one(conn, list_id)?;
    if list.is_none() {
        return Ok(None);
    };
    let list = list.unwrap();
    let now: DateTime<Utc> = Utc::now();
    let association_table_values = get_list_user_association(conn, list.id, user_id)?.unwrap();
    let mut res = ListGraphQL {
        id: list.id,
        name: list.name,
        list_type: list.list_type,
        is_owner: association_table_values.is_owner,
        is_validated: association_table_values.is_validated,
        list_permission: association_table_values.list_permission,
        tags: Vec::new(),
        items: Vec::new(),
        users_awaiting_validation: Vec::new(),
        users_validated: Vec::new(),
        created_at: now.naive_utc(),
        updated_at: now.naive_utc(),
    };
    enrich_list_with_relations(conn, &mut res, true, true, true)?;
    Ok(Some(res))
}

fn link_list_to_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    is_owner: bool,
    permission: ListPermission,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddUserToListStatus, diesel::result::Error> {
    let new_link = NewUserList {
        user_id,
        list_id,
        is_owner,
        is_validated: is_owner,
        list_permission: permission,
    };
    diesel::insert_into(user_lists)
        .values(&new_link)
        .execute(conn)?;
    notification_server.do_send(SendFriendshipNotification {
        user_id,
        message: MessageType::RefreshLists,
    });
    Ok(AddUserToListStatus::AddSuccessful)
}

fn remove_link_list_to_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<RemoveUserFromListStatus, diesel::result::Error> {
    diesel::delete(
        user_lists
            .filter(UserList_user_id.eq(user_id))
            .filter(UserList_list_id.eq(list_id)),
    )
    .execute(conn)?;
    notification_server.do_send(SendFriendshipNotification {
        user_id,
        message: MessageType::RefreshLists,
    });
    Ok(RemoveUserFromListStatus::RemoveSuccessful)
}

pub fn create_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateList,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddListResult, diesel::result::Error> {
    let new_list = NewList {
        id: Uuid::new_v4(),
        name: input.name,
        list_type: input.list_type,
    };
    let list = diesel::insert_into(lists)
        .values(&new_list)
        .get_result::<List>(conn)?;
    match link_list_to_user(
        conn,
        list.id,
        input.user_id,
        true,
        ListPermission::Owner,
        notification_server,
    )? {
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
    notification_server: &Addr<NotificationServer>,
) -> Result<DeleteResult, diesel::result::Error> {
    match find_one(conn, list_id)? {
        Some(list) => {
            let all_users = get_users_for_list(conn, list, true)?;
            diesel::delete(list_tags.filter(ListTagListId.eq(list_id))).execute(conn)?;
            diesel::delete(items.filter(ItemListId.eq(list_id))).execute(conn)?;
            diesel::delete(user_lists.filter(UserList_list_id.eq(list_id))).execute(conn)?;
            let rows_deleted =
                diesel::delete(lists.filter(ListId.eq(list_id))).execute(conn)? as usize;
            if rows_deleted > 0 {
                for user in all_users {
                    notification_server.do_send(SendFriendshipNotification {
                        user_id: user.id,
                        message: MessageType::RefreshLists,
                    });
                }
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

pub fn invite_user_to_your_list(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    friend_id: Uuid,
    permission: ListPermission,
    notification_server: &Addr<NotificationServer>,
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
            match link_list_to_user(
                conn,
                list_id,
                friend_id,
                false,
                permission,
                notification_server,
            )? {
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
    notification_server: &Addr<NotificationServer>,
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
    match remove_link_list_to_user(conn, list_id, friend_id, notification_server)? {
        RemoveUserFromListStatus::ErrCannotRemove => Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::ErrServerIssue,
        }),
        RemoveUserFromListStatus::RemoveSuccessful => Ok(RemoveFriendFromMyListResult {
            status: RemoveFriendFromMyListStatus::RemoveSuccessful,
        }),
    }
}

fn get_list_user_association(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
) -> Result<Option<UserList>, diesel::result::Error> {
    match user_lists
        .filter(
            UserList_list_id
                .eq(list_id)
                .and(UserList_user_id.eq(user_id)),
        )
        .first::<UserList>(conn)
    {
        Ok(user_list) => Ok(Some(user_list)),
        Err(_) => Ok(None),
    }
}

pub fn accept_list_invitation(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<AcceptListInvitationResult, diesel::result::Error> {
    match get_list_user_association(conn, list_id, user_id)? {
        Some(_) => {
            diesel::update(
                user_lists.filter(
                    UserList_list_id
                        .eq(list_id)
                        .and(UserList_user_id.eq(user_id)),
                ),
            )
            .set(is_validated.eq(true))
            .execute(conn)?;
            let list = find_one(conn, list_id)?.unwrap();
            for user in get_users_for_list(conn, list, true)? {
                notification_server.do_send(SendFriendshipNotification {
                    user_id: user.id,
                    message: MessageType::RefreshLists,
                });
            }
            Ok(AcceptListInvitationResult {
                status: AcceptListInvitationStatus::AcceptSuccessful,
            })
        }
        None => Ok(AcceptListInvitationResult {
            status: AcceptListInvitationStatus::ErrNoInvitationFound,
        }),
    }
}

pub fn refuse_list_invitation(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    list_id: Uuid,
    user_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<RefuseListInvitationResult, diesel::result::Error> {
    match get_list_user_association(conn, list_id, user_id)? {
        Some(_) => {
            diesel::delete(
                user_lists.filter(
                    UserList_list_id
                        .eq(list_id)
                        .and(UserList_user_id.eq(user_id)),
                ),
            )
            .execute(conn)?;
            let list = find_one(conn, list_id)?.unwrap();
            for user in get_users_for_list(conn, list, true)? {
                notification_server.do_send(SendFriendshipNotification {
                    user_id: user.id,
                    message: MessageType::RefreshLists,
                });
            }
            Ok(RefuseListInvitationResult {
                status: RefuseListInvitationStatus::RefuseSuccessful,
            })
        }
        None => Ok(RefuseListInvitationResult {
            status: RefuseListInvitationStatus::ErrNoInvitationFound,
        }),
    }
}
