use super::list_tag::ListTag;
use super::user_list::ListPermission;
use super::{item::Item, user::User};
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(table_name = lists)]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, Clone)]
#[ExistingTypePath = "crate::schema::sql_types::ListType"]
pub enum ListType {
    #[db_rename = "TO_DO"]
    ToDo,
    #[db_rename = "TO_BUY"]
    ToBuy,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A list")]
pub struct ListGraphQL {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
    pub tags: Vec<ListTag>,
    pub items: Vec<Item>,
    pub users: Vec<User>,
    pub is_owner: bool,
    pub is_validated: bool,
    pub list_permission: ListPermission,
}

#[derive(AsChangeset)]
#[diesel(table_name = lists)]
pub struct ListChangeset {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub list_type: Option<ListType>,
}

#[derive(Insertable)]
#[diesel(table_name = lists)]
pub struct NewList {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
}

#[derive(GraphQLInputObject)]
pub struct CreateList {
    pub name: String,
    pub list_type: ListType,
    pub user_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct UpdateList {
    pub name: String,
    pub list_type: ListType,
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
