use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::schema::lists;

#[derive(Queryable, Insertable, Identifiable, Debug, GraphQLObject)]
#[diesel(table_name = lists)]
#[graphql(description = "A list")]
pub struct List {
    pub id: Uuid,
    pub name: String,
    pub list_type: ListType,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ListType"]
pub enum ListType {
    ToDo,
    ToBuy,
}

#[derive(AsChangeset)]
#[diesel(table_name = lists)]
pub struct ListChangeset {
    pub name: Option<String>,
    pub list_type: Option<ListType>,
}
