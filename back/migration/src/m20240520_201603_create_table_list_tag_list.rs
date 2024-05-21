use sea_orm_migration::prelude::*;
use entity::list_tag_list::*;

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
                    .col(ColumnDef::new(Column::IdList).integer().not_null())
                    .col(ColumnDef::new(Column::IdTagList).integer().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk-list-tag-list")
                            .col(Column::IdList)
                            .col(Column::IdTagList)
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
