use crate::models::item_type::ItemType;
use crate::models::list_type::ListType;
use crate::schema::allowed_items;
use diesel::prelude::*;
use juniper::GraphQLObject;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(table_name = allowed_items)]
#[diesel(belongs_to(ItemType))]
#[diesel(belongs_to(ListType))]
#[diesel(primary_key(list_type_id, item_type_id))]
#[graphql(description = "Allowed item type for a list type")]
pub struct AllowedItem {
    pub list_type_id: Uuid,
    pub item_type_id: Uuid,
}
