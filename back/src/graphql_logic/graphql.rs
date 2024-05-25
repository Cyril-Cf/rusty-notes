use super::context::GraphQLContext;
use crate::schema::users::dsl::users;
use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use juniper::FieldError;
use juniper::{EmptySubscription, FieldResult, RootNode};
use uuid::Uuid;

use crate::models::user::{CreateUser, NewUser, User};
pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    // USER
    pub fn find_user(context: &GraphQLContext) -> FieldResult<Vec<String>> {
        // // TODO: pass the GraphQLContext into the querying functions rather
        // // than a PgConnection (for brevity's sake)
        // let conn: &PgConnection = &context.pool.get().unwrap();

        // Todos::all_todos(conn)
        todo!()
    }

    // LIST
    pub fn find_all_list_for_user_with_tags(
        context: &GraphQLContext,
        user_id: Uuid,
    ) -> FieldResult<Vec<String>> {
        todo!()
    }
    pub fn find_one_with_items_and_tags(
        context: &GraphQLContext,
        user_id: Uuid,
    ) -> FieldResult<Vec<String>> {
        todo!()
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    // USER
    pub fn create_user(context: &GraphQLContext, input: CreateUser) -> FieldResult<User> {
        let conn = &mut context.pool.get()?;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            email: input.email,
            firstname: input.firstname,
            lastname: input.lastname,
            keycloak_uuid: input.keycloak_uuid,
        };

        let res = diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(conn)?;
        Ok(res)
    }
    pub fn update_user(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }

    // LIST
    pub fn create_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn update_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn delete_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }

    // ITEM
    pub fn create_item(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn update_item(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn delete_item(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }

    // NOTIFICATION
    pub fn create_notification(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn update_notification(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn delete_notification(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
