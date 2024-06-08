use super::list_tag::ListTag;
use super::user_list::ListPermission;
use super::{item::ItemGraphQL, list_type::ListTypeGraphQL};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema::lists;

#[derive(
    Queryable, Insertable, Selectable, Identifiable, Associations, Debug, GraphQLObject, Clone,
)]
#[diesel(belongs_to(super::list_type::ListType))]
#[diesel(table_name = lists)]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub list_type_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A list")]
pub struct ListGraphQL {
    pub id: Uuid,
    pub name: String,
    pub tags: Vec<ListTag>,
    pub items: Vec<ItemGraphQL>,
    pub users_validated: Vec<UserListGraphQL>,
    pub users_awaiting_validation: Vec<UserListGraphQL>,
    pub is_owner: bool,
    pub is_validated: bool,
    pub list_permission: ListPermission,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub list_type: ListTypeGraphQL,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A user")]
pub struct UserListGraphQL {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub keycloak_uuid: Uuid,
    pub list_permission: ListPermission,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = lists)]
pub struct ListChangeset {
    pub id: Option<Uuid>,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = lists)]
pub struct NewList {
    pub name: String,
    pub list_type_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct CreateList {
    pub name: String,
    pub user_id: Uuid,
    pub list_type_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct UpdateList {
    pub name: String,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddFriendToMyListStatus {
    AddSuccessful,
    ErrNoFriendshipFound,
    ErrNoListFound,
    ErrNoUserFound,
    ErrNoUserFriendFound,
    ErrNotFriends,
    ErrServerIssue,
}

#[derive(Debug, GraphQLEnum)]
pub enum RemoveFriendFromMyListStatus {
    RemoveSuccessful,
    ErrNoListFound,
    ErrNoUserFriendFound,
    ErrServerIssue,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddListStatus {
    AddSuccessful,
    ErrNoUserFound,
}

#[derive(Debug, GraphQLEnum)]
pub enum AcceptListInvitationStatus {
    AcceptSuccessful,
    ErrNoInvitationFound,
}

#[derive(Debug, GraphQLEnum)]
pub enum RefuseListInvitationStatus {
    RefuseSuccessful,
    ErrNoInvitationFound,
}
