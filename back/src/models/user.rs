use diesel::prelude::*;
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::schema::users;

#[derive(Queryable, Insertable, Identifiable, Debug, GraphQLObject)]
#[diesel(table_name = users)]
#[graphql(description = "A user")]
pub struct User {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub keycloak_uuid: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChangeset {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub keycloak_uuid: Option<Uuid>,
}
