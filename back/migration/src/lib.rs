pub use sea_orm_migration::*;

mod m20240520_200943_create_table_notification;
mod m20240520_201004_create_table_notification_type;
mod m20240520_201034_create_table_notification_notification_type;
mod m20240520_201516_create_table_customer;
mod m20240520_201533_create_table_customer_notification;
mod m20240520_201630_create_table_list_type;
mod m20240520_201541_create_table_list;
mod m20240520_201551_create_table_customer_list;
mod m20240520_201557_create_table_list_tag;
mod m20240520_201603_create_table_list_tag_list;
mod m20240520_201718_create_table_item_type;
mod m20240520_201714_create_table_item;
mod m20240520_201728_create_table_list_type_item_type;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240520_200943_create_table_notification::Migration),
            Box::new(m20240520_201004_create_table_notification_type::Migration),
            Box::new(m20240520_201034_create_table_notification_notification_type::Migration),
            Box::new(m20240520_201516_create_table_customer::Migration),
            Box::new(m20240520_201533_create_table_customer_notification::Migration),
            Box::new(m20240520_201630_create_table_list_type::Migration),
            Box::new(m20240520_201541_create_table_list::Migration),
            Box::new(m20240520_201551_create_table_customer_list::Migration),
            Box::new(m20240520_201557_create_table_list_tag::Migration),
            Box::new(m20240520_201603_create_table_list_tag_list::Migration),
            Box::new(m20240520_201718_create_table_item_type::Migration),
            Box::new(m20240520_201714_create_table_item::Migration),
            Box::new(m20240520_201728_create_table_list_type_item_type::Migration),
        ]
    }
}
