use diesel::prelude::*;
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::schema::list_tags;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(belongs_to(super::user::User))]
#[diesel(table_name = list_tags)]
#[graphql(description = "A tag associated with a list")]
pub struct ListTag {
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = list_tags)]
pub struct ListTagChangeset {
    pub name: Option<String>,
    pub user_id: Option<Uuid>,
}
