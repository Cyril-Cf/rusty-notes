use crate::schema::list_types;
use diesel::prelude::*;
use juniper::GraphQLObject;
use uuid::Uuid;

use super::item_type::ItemType;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, GraphQLObject)]
#[diesel(table_name = list_types)]
#[graphql(description = "A list type")]
pub struct ListType {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A list")]
pub struct ListTypeGraphQL {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub allowed_item_types: Vec<ItemType>,
}
