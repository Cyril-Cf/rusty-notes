use super::item::Item;
use super::list_tag::ListTag;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = lists)]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
    pub user_id: Uuid,
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
    pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = lists)]
pub struct ListChangeset {
    pub name: Option<String>,
    pub list_type: Option<ListType>,
}

#[derive(Insertable)]
#[diesel(table_name = lists)]
pub struct NewList {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
    pub user_id: Uuid,
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
