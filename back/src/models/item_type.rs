use crate::schema::item_types;
use diesel::prelude::*;
use juniper::{GraphQLEnum, GraphQLObject};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, GraphQLObject)]
#[diesel(table_name = item_types)]
#[graphql(description = "An item type")]
pub struct ItemType {
    pub id: Uuid,
    pub item_type_variation: ItemTypeVariation,
}

#[derive(diesel_derive_enum::DbEnum, Debug, GraphQLEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::ItemTypeVariation"]
pub enum ItemTypeVariation {
    #[db_rename = "CONTENT"]
    Content,
    #[db_rename = "CHECKBOX"]
    Checkbox,
    #[db_rename = "MEDIA_URL"]
    MediaUrl,
    #[db_rename = "WEBSITE_URL"]
    WebsiteUrl,
    #[db_rename = "PERSON_IN_CHARGE"]
    PersonInCharge,
    #[db_rename = "DEADLINE_DATE"]
    DeadlineDate,
}
