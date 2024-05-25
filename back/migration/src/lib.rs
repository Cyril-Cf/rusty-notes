pub use sea_orm_migration::*;

mod m20240520_200943_create_table_notifications;
mod m20240520_201516_create_table_users;
mod m20240520_201533_create_table_user_notification;
mod m20240520_201541_create_table_lists;
mod m20240520_201551_create_table_user_list;
mod m20240520_201557_create_table_list_tags;
mod m20240520_201603_create_table_list_tag_list;
mod m20240520_201714_create_table_items;
mod m20240520_201715_create_table_friendships;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240520_200943_create_table_notifications::Migration),
            Box::new(m20240520_201516_create_table_users::Migration),
            Box::new(m20240520_201533_create_table_user_notification::Migration),
            Box::new(m20240520_201541_create_table_lists::Migration),
            Box::new(m20240520_201551_create_table_user_list::Migration),
            Box::new(m20240520_201557_create_table_list_tags::Migration),
            Box::new(m20240520_201603_create_table_list_tag_list::Migration),
            Box::new(m20240520_201714_create_table_items::Migration),
            Box::new(m20240520_201715_create_table_friendships::Migration),
        ]
    }
}
