use super::context::GraphQLContext;
use crate::models::item::{CreateItem, Item};
use crate::models::list::{CreateList, ListGraphQL};
use crate::models::list_tag::{CreateListTag, ListTag};
use crate::models::user::{CreateUser, ModifyUser, User};
use crate::services::item_service;
use crate::services::list_service;
use crate::services::list_tag_service;
use crate::services::user_service;
use juniper::FieldError;
use juniper::{EmptySubscription, FieldResult, RootNode};
use uuid::Uuid;

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    // USER
    pub fn find_user(context: &GraphQLContext, user_id: Uuid) -> FieldResult<User> {
        let conn = &mut context.pool.get()?;
        let res = user_service::find_user(conn, user_id);
        graphql_translate(res)
    }
    pub fn find_user_with_keycloak_id(
        context: &GraphQLContext,
        keycloak_id: Uuid,
    ) -> FieldResult<Option<User>> {
        let conn = &mut context.pool.get()?;
        let res = user_service::find_user_with_keycloak_id(conn, keycloak_id);
        graphql_translate(res)
    }

    // LIST
    pub fn find_all_list_for_user_with_tags(
        context: &GraphQLContext,
        user_id: Uuid,
    ) -> FieldResult<Vec<ListGraphQL>> {
        let conn = &mut context.pool.get()?;
        let res = list_service::find_all_list_for_user_with_tags(conn, user_id);
        graphql_translate(res)
    }
    pub fn find_one_with_items_and_tags(
        context: &GraphQLContext,
        list_id: Uuid,
    ) -> FieldResult<ListGraphQL> {
        let conn = &mut context.pool.get()?;
        let res = list_service::find_one_with_items_and_tags(conn, list_id);
        graphql_translate(res)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    // USER
    pub fn create_user(context: &GraphQLContext, input: CreateUser) -> FieldResult<User> {
        let conn = &mut context.pool.get()?;
        let res = user_service::create_user(conn, input);
        graphql_translate(res)
    }
    pub fn update_user(context: &GraphQLContext, input: ModifyUser) -> FieldResult<User> {
        let conn = &mut context.pool.get()?;
        let res = user_service::update_user(conn, input);
        graphql_translate(res)
    }

    // LIST
    pub fn create_list(context: &GraphQLContext, input: CreateList) -> FieldResult<ListGraphQL> {
        let conn = &mut context.pool.get()?;
        let res = list_service::create_list(conn, input);
        graphql_translate(res)
    }
    pub fn update_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn delete_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }

    // LIST TAGS
    pub fn create_list_tag(context: &GraphQLContext, input: CreateListTag) -> FieldResult<ListTag> {
        let conn = &mut context.pool.get()?;
        let res = list_tag_service::create_list_tag(conn, input);
        graphql_translate(res)
    }
    pub fn update_list_tag(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }
    pub fn delete_list_tag(context: &GraphQLContext, input: String) -> FieldResult<String> {
        todo!()
    }

    // ITEM
    pub fn create_item(context: &GraphQLContext, input: CreateItem) -> FieldResult<Item> {
        let conn = &mut context.pool.get()?;
        let res = item_service::create_item(conn, input);
        graphql_translate(res)
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

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}
