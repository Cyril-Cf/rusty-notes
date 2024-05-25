// use diesel::pg::PgConnection;
use diesel::prelude::*;
// use diesel::sql_types::Uuid as DieselUuid;
use juniper::GraphQLObject;
use uuid::Uuid;

use crate::schema::friendships;

#[derive(Queryable, Insertable, Identifiable, Associations, Debug, GraphQLObject)]
#[diesel(table_name = friendships)]
#[diesel(belongs_to(super::user::User))]
#[diesel(primary_key(user_id, user_id2))]
#[graphql(description = "A friendship between two users")]
pub struct Friendship {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_id2: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = friendships)]
pub struct FriendshipChangeset {
    pub user_id: Option<Uuid>,
    pub user_id2: Option<Uuid>,
}
