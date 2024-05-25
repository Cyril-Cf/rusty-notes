use entity::items::*;
use entity::lists::{Column as ListColumn, Entity as ListEntity};
use sea_orm::{DbBackend, Schema};
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_type(schema.create_enum_from_active_enum::<ItemType>())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::IsChecked).boolean().not_null())
                    .col(ColumnDef::new(Column::IdList).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-item-list")
                            .from(Entity, Column::IdList)
                            .to(ListEntity, ListColumn::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .col(ColumnDef::new(Column::ItemType).string().not_null())
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
        manager
            .get_connection()
            .execute_unprepared("DROP TYPE item_type;")
            .await?;
        Ok(())
    }
}
