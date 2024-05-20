use sea_orm_migration::prelude::*;
use entity::customer_list::*;

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
                    .col(ColumnDef::new(Column::IsOwner).boolean().not_null())
                    .col(ColumnDef::new(Column::IdCustomer).integer().not_null())
                    .col(ColumnDef::new(Column::IdList).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-customer-list")
                            .col(Column::IdCustomer)
                            .col(Column::IdList)
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
