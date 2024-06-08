use crate::schema::items;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use super::item_type::ItemType;

#[derive(Queryable, Identifiable, Associations, Debug)]
#[diesel(belongs_to(super::item_type::ItemType))]
#[diesel(belongs_to(super::list::List))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub content: Option<String>,
    pub is_checked: Option<bool>,
    pub media_url: Option<String>,
    pub website_url: Option<String>,
    pub person_in_charge: Option<String>,
    pub priority_type: Option<PriorityType>,
    pub list_id: Uuid,
    pub item_type_id: Uuid,
    pub deadline_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLObject)]
#[graphql(description = "An item")]
pub struct ItemGraphQL {
    pub id: Uuid,
    pub content: Option<String>,
    pub is_checked: Option<bool>,
    pub media_url: Option<String>,
    pub website_url: Option<String>,
    pub person_in_charge: Option<String>,
    pub priority_type: Option<PriorityType>,
    pub item_type: ItemType,
    pub deadline_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::PriorityType"]
pub enum PriorityType {
    #[db_rename = "LOW"]
    Low,
    #[db_rename = "MIDDLE"]
    Middle,
    #[db_rename = "HIGH"]
    High,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub content: Option<String>,
    pub is_checked: Option<bool>,
    pub website_url: Option<String>,
    pub media_url: Option<String>,
    pub deadline_date: Option<NaiveDate>,
    pub priority_type: Option<PriorityType>,
    pub person_in_charge: Option<String>,
    pub list_id: Uuid,
    pub item_type_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct CreateItem {
    pub content: Option<String>,
    pub is_checked: Option<bool>,
    pub website_url: Option<String>,
    pub media_url: Option<String>,
    pub deadline_date: Option<NaiveDate>,
    pub person_in_charge: Option<String>,
    pub priority_type: Option<PriorityType>,
    pub list_id: Uuid,
    pub item_type_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct UpdateItem {
    pub id: Uuid,
    pub content: Option<String>,
    pub is_checked: Option<bool>,
    pub website_url: Option<String>,
    pub media_url: Option<String>,
    pub deadline_date: Option<NaiveDate>,
    pub priority_type: Option<PriorityType>,
    pub person_in_charge: Option<String>,
    pub list_id: Uuid,
    pub item_type_id: Uuid,
}

#[derive(Debug, GraphQLEnum)]
pub enum AddItemStatus {
    AddSuccessful,
    ErrNoListFound,
}
