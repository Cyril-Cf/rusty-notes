use sea_orm_migration::prelude::*;
use entity::notification_notification_type::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::IdNotification).integer().not_null())
                    .col(ColumnDef::new(Column::IdNotificationType).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-notification-notification-type")
                            .col(Column::IdNotification)
                            .col(Column::IdNotificationType)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Entity).to_owned())
            .await
    }
}
