use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::schema::friendships;

use super::user::User;

#[derive(Queryable, AsChangeset, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(table_name = friendships)]
#[diesel(belongs_to(super::user::User))]
#[diesel(primary_key(user_id, user_id2))]
#[graphql(description = "A friendship between two users")]
pub struct Friendship {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_id2: Uuid,
    pub is_validated: bool,
}

#[derive(Insertable)]
#[diesel(table_name = friendships)]
pub struct NewFriendship {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_id2: Uuid,
    pub is_validated: bool,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A friendship")]
pub struct FriendshipGraphQL {
    pub id: Uuid,
    pub is_validated: bool,
    pub friend: User,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddFriendStatus {
    AddSuccessful,
    ErrNoUserId,
    ErrAlreadyFriend,
    ErrAlreadyPendingDemand,
    NotFriendsYet,
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
