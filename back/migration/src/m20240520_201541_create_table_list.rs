use sea_orm_migration::prelude::*;
use entity::list::*;
use entity::list_type::{Entity as ListTypeEntity, Column as ListTypeColumn};

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
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::IdListType).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-list-list-type")
                            .from(Entity, Column::IdListType)
                            .to(ListTypeEntity, ListTypeColumn::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
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
