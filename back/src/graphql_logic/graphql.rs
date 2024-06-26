use super::context::GraphQLContext;
use crate::models::friendship::FriendshipGraphQL;
use crate::models::item::{CreateItem, UpdateItem};
use crate::models::list::{CreateList, ListGraphQL};
use crate::models::list_tag::{CreateListTag, ListTag};
use crate::models::list_type::ListTypeGraphQL;
use crate::models::notification::{CreateNotification, Notification, UpdateNotificationGQL};
use crate::models::user::{CreateUser, ModifyUser, User};
use crate::models::user_list::ListPermission;
use crate::services::friendship_service::{
    self, AcceptFriendshipResult, AddFriendshipResult, RemoveFriendshipResult,
};
use crate::services::item_service::{self, AddItemResult};
use crate::services::list_service::{
    self, AcceptListInvitationResult, AddFriendToMyListResult, AddListResult,
    RefuseListInvitationResult, RemoveFriendFromMyListResult,
};
use crate::services::list_tag_service;
use crate::services::list_type_service;
use crate::services::notification_service;
use crate::services::user_service;
use juniper::{graphql_value, FieldError};
use juniper::{EmptySubscription, FieldResult, GraphQLEnum, GraphQLObject, RootNode};
use uuid::Uuid;

#[derive(Debug, GraphQLObject)]
pub struct DeleteResult {
    pub status: DeleteStatus,
}

#[derive(Debug, GraphQLEnum)]
pub enum DeleteStatus {
    ResourceDeleted,
    NoDeletion,
}

#[derive(Debug, GraphQLObject)]
pub struct UpdateResult {
    pub status: UpdateStatus,
}

#[derive(Debug, GraphQLEnum)]
pub enum UpdateStatus {
    ResourceUpdated,
    NoUpdate,
}

pub struct Query;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Query {
    // USER
    pub fn find_user(context: &GraphQLContext, user_id: Uuid) -> FieldResult<Option<User>> {
        let conn = &mut context.pool.get()?;
        let res = user_service::find_user(conn, user_id);
        graphql_translate(res)
    }
    pub async fn find_user_with_keycloak_id(
        context: &GraphQLContext,
        keycloak_id: Uuid,
    ) -> FieldResult<Option<User>> {
        let conn = &mut context.pool.get()?;
        let res = user_service::find_user_with_keycloak_id(conn, keycloak_id).await;
        match res {
            Ok(t) => Ok(t),
            Err(e) => FieldResult::Err(FieldError::new(
                e.to_string(),
                graphql_value!({"database_error": "Impossible"}),
            )),
        }
    }
    pub fn get_user_friendships(
        context: &GraphQLContext,
        user_id: Uuid,
    ) -> FieldResult<Vec<FriendshipGraphQL>> {
        let conn = &mut context.pool.get()?;
        let res = friendship_service::get_user_friendships(conn, user_id);
        graphql_translate(res)
    }

    // LIST TYPES
    pub fn find_all_list_types_with_allowed_item_types(
        context: &GraphQLContext,
    ) -> FieldResult<Vec<ListTypeGraphQL>> {
        let conn = &mut context.pool.get()?;
        let res = list_type_service::find_all_with_item_types(conn);
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
        user_id: Uuid,
    ) -> FieldResult<Option<ListGraphQL>> {
        let conn = &mut context.pool.get()?;
        let res = list_service::find_one_with_items_and_tags(conn, list_id, user_id);
        graphql_translate(res)
    }

    // NOTIFICATION
    pub fn find_all_notifications_for_user(
        context: &GraphQLContext,
        user_id: Uuid,
    ) -> FieldResult<Vec<Notification>> {
        let conn = &mut context.pool.get()?;
        let res = notification_service::find_all_notifications_for_user(conn, user_id);
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
    pub fn add_user_friend(
        context: &GraphQLContext,
        user_who_asked_id: Uuid,
        user_who_get_asked_email: String,
    ) -> FieldResult<AddFriendshipResult> {
        let conn = &mut context.pool.get()?;
        let res: Result<AddFriendshipResult, diesel::result::Error> =
            friendship_service::add_friend_user(
                conn,
                user_who_asked_id,
                user_who_get_asked_email,
                &context.notification_server,
            );
        graphql_translate(res)
    }
    pub fn remove_user_friend(
        context: &GraphQLContext,
        user_id: Uuid,
        user_friend_id: Uuid,
    ) -> FieldResult<RemoveFriendshipResult> {
        let conn = &mut context.pool.get()?;
        let res = friendship_service::remove_user_friend(
            conn,
            user_id,
            user_friend_id,
            &context.notification_server,
        );
        graphql_translate(res)
    }
    pub fn confirm_friendship(
        context: &GraphQLContext,
        user_asked_id: Uuid,
        user_asking_id: Uuid,
    ) -> FieldResult<AcceptFriendshipResult> {
        let conn = &mut context.pool.get()?;
        let res = friendship_service::confirm_friendship(
            conn,
            user_asked_id,
            user_asking_id,
            &context.notification_server,
        );
        graphql_translate(res)
    }

    // LIST
    pub fn create_list(context: &GraphQLContext, input: CreateList) -> FieldResult<AddListResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::create_list(conn, input, &context.notification_server);
        graphql_translate(res)
    }
    // pub fn update_list(context: &GraphQLContext, input: String) -> FieldResult<String> {
    //     todo!()
    // }
    pub fn delete_list(context: &GraphQLContext, list_id: Uuid) -> FieldResult<DeleteResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::delete_list(conn, list_id, &context.notification_server);
        graphql_translate(res)
    }
    pub fn invite_user_to_your_list(
        context: &GraphQLContext,
        list_id: Uuid,
        user_id: Uuid,
        friend_id: Uuid,
        permission: ListPermission,
    ) -> FieldResult<AddFriendToMyListResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::invite_user_to_your_list(
            conn,
            list_id,
            user_id,
            friend_id,
            permission,
            &context.notification_server,
        );
        graphql_translate(res)
    }
    pub fn remove_user_from_list(
        context: &GraphQLContext,
        list_id: Uuid,
        friend_id: Uuid,
    ) -> FieldResult<RemoveFriendFromMyListResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::remove_user_from_list(
            conn,
            list_id,
            friend_id,
            &context.notification_server,
        );
        graphql_translate(res)
    }
    pub fn accept_list_invitation(
        context: &GraphQLContext,
        list_id: Uuid,
        user_id: Uuid,
    ) -> FieldResult<AcceptListInvitationResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::accept_list_invitation(
            conn,
            list_id,
            user_id,
            &context.notification_server,
        );
        graphql_translate(res)
    }

    pub fn refuse_list_invitation(
        context: &GraphQLContext,
        list_id: Uuid,
        user_id: Uuid,
    ) -> FieldResult<RefuseListInvitationResult> {
        let conn = &mut context.pool.get()?;
        let res = list_service::refuse_list_invitation(
            conn,
            list_id,
            user_id,
            &context.notification_server,
        );
        graphql_translate(res)
    }

    // LIST TAGS
    pub fn create_list_tag(context: &GraphQLContext, input: CreateListTag) -> FieldResult<ListTag> {
        let conn = &mut context.pool.get()?;
        let res = list_tag_service::create_list_tag(conn, input);
        graphql_translate(res)
    }
    // pub fn update_list_tag(context: &GraphQLContext, input: String) -> FieldResult<String> {
    //     todo!()
    // }
    // pub fn delete_list_tag(context: &GraphQLContext, input: String) -> FieldResult<String> {
    //     todo!()
    // }

    // ITEM
    pub fn create_item(context: &GraphQLContext, input: CreateItem) -> FieldResult<AddItemResult> {
        let conn = &mut context.pool.get()?;
        let res = item_service::create_item(conn, input, &context.notification_server);
        graphql_translate(res)
    }
    pub fn update_item(context: &GraphQLContext, input: UpdateItem) -> FieldResult<UpdateResult> {
        let conn = &mut context.pool.get()?;
        let res = item_service::update_item(conn, input, &context.notification_server);
        graphql_translate(res)
    }
    pub fn delete_item(context: &GraphQLContext, item_id: Uuid) -> FieldResult<DeleteResult> {
        let conn = &mut context.pool.get()?;
        let res = item_service::delete_item(conn, item_id, &context.notification_server);
        graphql_translate(res)
    }

    // NOTIFICATION
    pub fn create_notification(
        context: &GraphQLContext,
        input: CreateNotification,
    ) -> FieldResult<Notification> {
        let conn = &mut context.pool.get()?;
        let res = notification_service::create_notification(conn, input);
        graphql_translate(res)
    }
    pub fn update_notification(
        context: &GraphQLContext,
        input: UpdateNotificationGQL,
    ) -> FieldResult<UpdateResult> {
        let conn = &mut context.pool.get()?;
        let res = notification_service::update_notification(conn, input);
        graphql_translate(res)
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
