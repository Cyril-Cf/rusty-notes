use crate::models::user::{CreateUser, ModifyUser, NewUser, User, UserChangeset};
use crate::schema::users::dsl::{id, users};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use uuid::Uuid;

pub fn find_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_id: Uuid,
) -> Result<User, diesel::result::Error> {
    users.filter(id.eq(user_id)).first::<User>(conn)
}

pub fn create_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: CreateUser,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: input.email,
        firstname: input.firstname,
        lastname: input.lastname,
        keycloak_uuid: input.keycloak_uuid,
    };
    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
}

pub fn update_user(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    input: ModifyUser,
) -> Result<User, diesel::result::Error> {
    let target = users.filter(id.eq(input.id));
    diesel::update(target)
        .set(&UserChangeset {
            id: None,
            email: Some(input.email),
            firstname: Some(input.firstname),
            lastname: Some(input.lastname),
            keycloak_uuid: Some(input.keycloak_uuid),
        })
        .get_result::<User>(conn)
}
