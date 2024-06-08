use crate::schema::items;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::list::List))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub is_checked: bool,
    pub list_id: Uuid,
    pub item_type: ItemType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ItemType"]
pub enum ItemType {
    #[db_rename = "CHECKBOX"]
    Checkbox,
    #[db_rename = "BULLETPOINT"]
    BulletPoint,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub id: Uuid,
    pub name: String,
    pub is_checked: bool,
    pub list_id: Uuid,
    pub item_type: ItemType,
}

#[derive(GraphQLInputObject)]
pub struct CreateItem {
    pub name: String,
    pub is_checked: bool,
    pub list_id: Uuid,
    pub item_type: ItemType,
}

#[derive(GraphQLInputObject)]
pub struct UpdateItem {
    pub id: Uuid,
    pub name: String,
    pub is_checked: bool,
    pub list_id: Uuid,
    pub item_type: ItemType,
}
