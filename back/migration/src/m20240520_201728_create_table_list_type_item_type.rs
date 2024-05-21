use sea_orm_migration::prelude::*;
use entity::list_type_item_type::*;

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
                    .col(ColumnDef::new(Column::IdItemType).integer().not_null())
                    .col(ColumnDef::new(Column::IdListType).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-list-type-item-type")
                            .col(Column::IdItemType)
                            .col(Column::IdListType)
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
