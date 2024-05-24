use entity::item::{Entity as ItemEntity, ItemType, Model as ItemModel};
use entity::list::{Entity, ListType, Model};
use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    ModelTrait,
};
use uuid::Uuid;

// pub async fn find_one(id: Uuid, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
//     Entity::find_by_id(id).one(conn).await
// }

// pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
//     Entity::find().all(conn).await
// }

pub async fn create(new_name: String, conn: &DatabaseConnection) -> Result<ItemModel, DbErr> {
    let model = entity::list::ActiveModel {
        name: ActiveValue::Set(new_name),
        list_type: ActiveValue::Set(ListType::ToDo),
        id: ActiveValue::Set(Uuid::new_v4()),
    }
    .insert(conn)
    .await?;
    entity::item::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        id_list: ActiveValue::Set(model.id),
        is_checked: ActiveValue::Set(false),
        item_type: ActiveValue::Set(ItemType::BulletPoint),
        name: ActiveValue::Set("test".to_string()),
        ..Default::default()
    }
    .insert(conn)
    .await
}

// pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
//     match Entity::find_by_id(id).one(conn).await? {
//         Some(entity) => {
//             if !user_permission_service::find_all_for_permission(entity.id, conn)
//                 .await?
//                 .is_empty()
//             {
//                 return Ok(None);
//             }
//             Ok(Some(entity.delete(conn).await?))
//         }
//         None => Ok(None),
//     }
// }
