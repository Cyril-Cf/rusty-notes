use crate::models::friendship::{
    AddFriendStatus, Friendship, FriendshipAcceptingStatus, FriendshipGraphQL, FriendshipState,
    NewFriendship, RemoveFriendStatus,
};
use crate::models::user::{CreateUser, ModifyUser, NewUser, User, UserChangeset};
use crate::schema::friendships::dsl::{
    friendships, is_validated, user_who_asked_id as friendship_id_1,
    user_who_got_asked_id as friendship_id_2,
};
use crate::schema::users::dsl::{email, id, keycloak_uuid, users};
use crate::{NotificationServer, SendFriendshipNotification};
use actix::Addr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn find_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    match users.filter(id.eq(user_id)).first::<User>(conn) {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}

fn find_user_with_email(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_email: String,
) -> Result<Option<User>, diesel::result::Error> {
    match users.filter(email.eq(user_email)).first::<User>(conn) {
        Ok(user) => Ok(Some(user)),
        Err(_) => Ok(None),
    }
}

pub fn find_user_with_keycloak_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    keycloak_id: Uuid,
) -> Result<Option<User>, diesel::result::Error> {
    let user_option = users
        .filter(keycloak_uuid.eq(keycloak_id))
        .first(conn)
        .optional()?;

    Ok(user_option)
}

pub fn create_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateUser,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: input.email,
        firstname: input.firstname,
        lastname: input.lastname,
        keycloak_uuid: input.keycloak_uuid,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
}

pub fn update_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: ModifyUser,
) -> Result<User, diesel::result::Error> {
    let target = users.filter(id.eq(input.id));
    diesel::update(target)
        .set(&UserChangeset {
            id: None,
            email: Some(input.email),
            firstname: Some(input.firstname),
            lastname: Some(input.lastname),
            keycloak_uuid: Some(input.keycloak_uuid),
        })
        .get_result::<User>(conn)
}

pub fn add_friend_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_email: String,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddFriendStatus, diesel::result::Error> {
    match find_user_with_email(conn, user_email)? {
        Some(user) => match does_friendship_exists(conn, user_id, user.id)? {
            FriendshipState::ExistsAndValidated => Ok(AddFriendStatus::ErrAlreadyFriend),
            FriendshipState::ExistsButNotValidate => Ok(AddFriendStatus::ErrAlreadyPendingDemand),
            FriendshipState::DoesNotExist => {
                add_friendship(conn, user_id, user.id, notification_server)
            }
        },
        None => Ok(AddFriendStatus::ErrNoUserId),
    }
}

fn add_friendship(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_friend_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddFriendStatus, diesel::result::Error> {
    let friendship = NewFriendship {
        id: Uuid::new_v4(),
        user_who_asked_id: user_id,
        user_who_got_asked_id: user_friend_id,
        is_validated: false,
    };
    diesel::insert_into(friendships)
        .values(&friendship)
        .execute(conn)?;
    notification_server.do_send(SendFriendshipNotification {
        user_id: user_friend_id,
        message: "Vous avez un nouvel ami!".to_owned(),
    });
    Ok(AddFriendStatus::AddSuccessful)
}

fn does_friendship_exists(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_friend_id: Uuid,
) -> Result<FriendshipState, diesel::result::Error> {
    let friendship: Option<Friendship> = friendships
        .filter(
            (friendship_id_1
                .eq(user_id)
                .and(friendship_id_2.eq(user_friend_id)))
            .or(friendship_id_1
                .eq(user_id)
                .and(friendship_id_2.eq(user_friend_id))),
        )
        .first(conn)
        .optional()?;
    match friendship {
        Some(friendship) => {
            if friendship.is_validated {
                Ok(FriendshipState::ExistsAndValidated)
            } else {
                Ok(FriendshipState::ExistsButNotValidate)
            }
        }
        None => Ok(FriendshipState::DoesNotExist),
    }
}

pub fn get_user_friendships(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<Vec<FriendshipGraphQL>, diesel::result::Error> {
    let mut res = Vec::new();
    let all_friendships: Vec<Friendship> = friendships
        .filter((friendship_id_1.eq(user_id)).or(friendship_id_1.eq(user_id)))
        .load(conn)?;
    for friendship in all_friendships {
        let friend = if friendship.user_who_asked_id == user_id {
            find_user(conn, friendship.user_who_got_asked_id)?.unwrap()
        } else {
            find_user(conn, user_id)?.unwrap()
        };
        res.push(FriendshipGraphQL {
            id: friendship.id,
            is_validated: friendship.is_validated,
            friend,
        })
    }
    Ok(res)
}

pub fn remove_user_friend(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_friend_id: Uuid,
) -> Result<RemoveFriendStatus, diesel::result::Error> {
    match does_friendship_exists(conn, user_id, user_friend_id)? {
        FriendshipState::DoesNotExist => Ok(RemoveFriendStatus::ErrNoFriendship),
        FriendshipState::ExistsAndValidated | FriendshipState::ExistsButNotValidate => {
            diesel::delete(
                friendships.filter(
                    (friendship_id_1
                        .eq(user_id)
                        .and(friendship_id_2.eq(user_friend_id)))
                    .or(friendship_id_1
                        .eq(user_friend_id)
                        .and(friendship_id_2.eq(user_id))),
                ),
            )
            .execute(conn)?;
            Ok(RemoveFriendStatus::RemoveSuccessful)
        }
    }
}

pub fn confirm_friendship(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_asked_id: Uuid,
    user_friend_id: Uuid,
) -> Result<FriendshipAcceptingStatus, diesel::result::Error> {
    match does_friendship_exists(conn, user_asked_id, user_friend_id)? {
        FriendshipState::DoesNotExist | FriendshipState::ExistsAndValidated => {
            Ok(FriendshipAcceptingStatus::ErrCannotAccept)
        }
        FriendshipState::ExistsButNotValidate => {
            diesel::update(
                friendships.filter(
                    friendship_id_1
                        .eq(user_friend_id)
                        .and(friendship_id_2.eq(user_asked_id)),
                ),
            )
            .set(is_validated.eq(true))
            .execute(conn)?;
            Ok(FriendshipAcceptingStatus::AcceptingSuccessful)
        }
    }
}
