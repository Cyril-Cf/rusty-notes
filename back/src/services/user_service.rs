use entity::users::{Entity, Model};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait,
};
use uuid::Uuid;

pub async fn find_one(id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(conn).await
}

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    entity::users::ActiveModel {
        firstname: ActiveValue::Set(new_name),
        ..Default::default()
    }
    .insert(conn)
    .await
}
