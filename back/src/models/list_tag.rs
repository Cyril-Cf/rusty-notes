use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema::list_tags;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::list::List))]
#[diesel(table_name = list_tags)]
#[graphql(description = "A tag associated with a list")]
pub struct ListTag {
    pub id: Uuid,
    pub name: String,
    pub list_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = list_tags)]
pub struct ListTagChangeset {
    pub name: Option<String>,
    pub list_id: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = list_tags)]
pub struct NewListTag {
    pub name: String,
    pub list_id: Uuid,
}

#[derive(GraphQLInputObject)]
pub struct CreateListTag {
    pub name: String,
    pub list_id: Uuid,
}
