use super::user_service::{find_user, find_user_with_email};
use crate::models::friendship::{
    AddFriendStatus, Friendship, FriendshipAcceptingStatus, FriendshipGraphQL, FriendshipState,
    NewFriendship, RemoveFriendStatus,
};
use crate::models::notification::{CreateNotification, NotifType};
use crate::schema::friendships::dsl::{
    friendships, is_validated, user_who_asked_id as friendship_id_1,
    user_who_got_asked_id as friendship_id_2,
};
use crate::services::notification_service;
use crate::web_socket_logic::web_socket::{
    MessageType, NotificationServer, SendFriendshipNotification,
};
use actix::Addr;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use juniper::GraphQLObject;
use uuid::Uuid;

#[derive(Debug, GraphQLObject)]
pub struct AddFriendshipResult {
    pub status: AddFriendStatus,
}

#[derive(Debug, GraphQLObject)]
pub struct RemoveFriendshipResult {
    pub status: RemoveFriendStatus,
}

#[derive(Debug, GraphQLObject)]
pub struct AcceptFriendshipResult {
    pub status: FriendshipAcceptingStatus,
}

pub fn add_friend_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_email: String,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddFriendshipResult, diesel::result::Error> {
    match find_user_with_email(conn, user_email)? {
        Some(user) => match does_friendship_exists(conn, user_id, user.id)? {
            FriendshipState::ExistsAndValidated => Ok(AddFriendshipResult {
                status: AddFriendStatus::ErrAlreadyFriend,
            }),
            FriendshipState::ExistsButNotValidate => Ok(AddFriendshipResult {
                status: AddFriendStatus::ErrAlreadyPendingDemand,
            }),
            FriendshipState::DoesNotExist => {
                add_friendship(conn, user_id, user.id, notification_server)
            }
        },
        None => Ok(AddFriendshipResult {
            status: AddFriendStatus::ErrNoUserEmail,
        }),
    }
}

fn add_friendship(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_friend_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<AddFriendshipResult, diesel::result::Error> {
    let friendship = NewFriendship {
        id: Uuid::new_v4(),
        user_who_asked_id: user_id,
        user_who_got_asked_id: user_friend_id,
        is_validated: false,
    };
    diesel::insert_into(friendships)
        .values(&friendship)
        .execute(conn)?;
    notification_service::create_notification(
        conn,
        CreateNotification {
            notif_type: NotifType::NewFriendshipDemand,
            user_id: user_friend_id,
        },
    )?;
    notification_server.do_send(SendFriendshipNotification {
        user_id: user_friend_id,
        message: MessageType::RefreshFriendships,
    });
    Ok(AddFriendshipResult {
        status: AddFriendStatus::AddSuccessful,
    })
}

pub fn does_friendship_exists(
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
                .eq(user_friend_id)
                .and(friendship_id_2.eq(user_id))),
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
        .filter((friendship_id_1.eq(user_id)).or(friendship_id_2.eq(user_id)))
        .load(conn)?;
    for friendship in all_friendships {
        res.push(FriendshipGraphQL {
            id: friendship.id,
            is_validated: friendship.is_validated,
            friend_who_asked: find_user(conn, friendship.user_who_asked_id)?.unwrap(),
            friend_who_got_asked: find_user(conn, friendship.user_who_got_asked_id)?.unwrap(),
        })
    }
    Ok(res)
}

pub fn remove_user_friend(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
    user_friend_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<RemoveFriendshipResult, diesel::result::Error> {
    match does_friendship_exists(conn, user_id, user_friend_id)? {
        FriendshipState::DoesNotExist => Ok(RemoveFriendshipResult {
            status: RemoveFriendStatus::ErrNoFriendship,
        }),
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
            notification_server.do_send(SendFriendshipNotification {
                user_id,
                message: MessageType::RefreshFriendships,
            });
            notification_server.do_send(SendFriendshipNotification {
                user_id: user_friend_id,
                message: MessageType::RefreshFriendships,
            });
            Ok(RemoveFriendshipResult {
                status: RemoveFriendStatus::RemoveSuccessful,
            })
        }
    }
}

pub fn confirm_friendship(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_asked_id: Uuid,
    user_friend_id: Uuid,
    notification_server: &Addr<NotificationServer>,
) -> Result<AcceptFriendshipResult, diesel::result::Error> {
    match does_friendship_exists(conn, user_asked_id, user_friend_id)? {
        FriendshipState::DoesNotExist | FriendshipState::ExistsAndValidated => {
            Ok(AcceptFriendshipResult {
                status: FriendshipAcceptingStatus::ErrCannotAccept,
            })
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
            notification_server.do_send(SendFriendshipNotification {
                user_id: user_asked_id,
                message: MessageType::RefreshFriendships,
            });
            notification_server.do_send(SendFriendshipNotification {
                user_id: user_friend_id,
                message: MessageType::RefreshFriendships,
            });
            notification_service::create_notification(
                conn,
                CreateNotification {
                    notif_type: NotifType::NewFriendshipAccepted,
                    user_id: user_friend_id,
                },
            )?;
            Ok(AcceptFriendshipResult {
                status: FriendshipAcceptingStatus::AcceptingSuccessful,
            })
        }
    }
}
