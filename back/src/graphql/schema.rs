use entity::{friendships, users};
use juniper::{Context as JuniperContext, EmptySubscription, FieldResult, RootNode};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn all_users(context: &Context) -> FieldResult<Vec<users::Model>> {
        let users = users::Entity::find().all(&context.db).await?;
        Ok(users)
    }

    async fn all_friendships(context: &Context) -> FieldResult<Vec<friendships::Model>> {
        let friendships = friendships::Entity::find().all(&context.db).await?;
        Ok(friendships)
    }

    async fn user(context: &Context, id: juniper::ID) -> FieldResult<Option<users::Model>> {
        let user_id = id.parse::<Uuid>()?;
        let user = users::Entity::find_by_id(user_id).one(&context.db).await?;
        Ok(user)
    }

    async fn friendship(
        context: &Context,
        id: juniper::ID,
    ) -> FieldResult<Option<friendships::Model>> {
        let friendship_id = id.parse::<Uuid>()?;
        let friendship = friendships::Entity::find_by_id(friendship_id)
            .one(&context.db)
            .await?;
        Ok(friendship)
    }
}

#[derive(Debug, Clone)]
pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(
        context: &Context,
        username: String,
        email: String,
        password: String,
    ) -> FieldResult<users::Model> {
        let user = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(email),
            ..Default::default()
        };

        let res = users::Entity::insert(user).exec(&context.db).await?;
        let created_user = users::Entity::find_by_id(res.last_insert_id)
            .one(&context.db)
            .await?
            .unwrap();
        Ok(created_user)
    }

    async fn create_friendship(
        context: &Context,
        id_user1: Uuid,
        id_user2: Uuid,
    ) -> FieldResult<friendships::Model> {
        let friendship = friendships::ActiveModel {
            id: Set(Uuid::new_v4()),
            id_user1: Set(id_user1),
            id_user2: Set(id_user2),
        };

        let res = friendships::Entity::insert(friendship)
            .exec(&context.db)
            .await?;
        let created_friendship = friendships::Entity::find_by_id(res.last_insert_id)
            .one(&context.db)
            .await?
            .unwrap();
        Ok(created_friendship)
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

impl JuniperContext for Context {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
