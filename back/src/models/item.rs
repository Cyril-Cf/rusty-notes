use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

use crate::schema::items;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::list::List))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub is_checked: bool,
    pub list_id: Uuid,
    pub item_type: ItemType,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ItemType"]
pub enum ItemType {
    Checkbox,
    BulletPoint,
}