use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::schema::friendships;

use super::user::User;

#[derive(Queryable, AsChangeset, Identifiable, Debug, GraphQLObject)]
#[diesel(table_name = friendships)]
#[diesel(primary_key(user_who_asked_id, user_who_got_asked_id))]
#[graphql(description = "A friendship between two users")]
pub struct Friendship {
    pub id: Uuid,
    pub user_who_asked_id: Uuid,
    pub user_who_got_asked_id: Uuid,
    pub is_validated: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = friendships)]
pub struct NewFriendship {
    pub user_who_asked_id: Uuid,
    pub user_who_got_asked_id: Uuid,
    pub is_validated: bool,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A friendship")]
pub struct FriendshipGraphQL {
    pub id: Uuid,
    pub is_validated: bool,
    pub friend_who_asked: User,
    pub friend_who_got_asked: User,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddFriendStatus {
    AddSuccessful,
    ErrNoUserEmail,
    ErrAlreadyFriend,
    ErrAlreadyPendingDemand,
}

#[derive(Debug, GraphQLEnum)]
pub enum RemoveFriendStatus {
    RemoveSuccessful,
    ErrNoFriendship,
}

pub enum FriendshipState {
    ExistsAndValidated,
    ExistsButNotValidate,
    DoesNotExist,
}

#[derive(Debug, GraphQLEnum)]
pub enum FriendshipAcceptingStatus {
    AcceptingSuccessful,
    ErrCannotAccept,
}
