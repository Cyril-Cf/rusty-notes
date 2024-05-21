use sea_orm_migration::prelude::*;
use entity::item::*;
use entity::list::{Entity as ListEntity, Column as ListColumn};
use entity::item_type::{Entity as ItemTypeEntity, Column as ItemTypeColumn};

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
                    .col(ColumnDef::new(Column::IsChecked).boolean().not_null())
                    .col(ColumnDef::new(Column::IdList).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-list")
                            .from(Entity, Column::IdList)
                            .to(ItemTypeEntity, ItemTypeColumn::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .col(ColumnDef::new(Column::IdItemType).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-item-type")
                            .from(Entity, Column::IdItemType)
                            .to(ListEntity, ListColumn::Id)
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
