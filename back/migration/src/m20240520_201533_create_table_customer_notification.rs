use sea_orm_migration::prelude::*;
use entity::customer_notification::*;

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
                    .col(ColumnDef::new(Column::IdCustomer).integer().not_null())
                    .col(ColumnDef::new(Column::IdNotification).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-customer-notification")
                            .col(Column::IdCustomer)
                            .col(Column::IdNotification)
                            .primary(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if manager.has_table(Entity).await? {
            manager
                .drop_table(sea_query::Table::drop().table(Entity).cascade().to_owned())
                .await?;
        }
        Ok(())
        
    }
}
