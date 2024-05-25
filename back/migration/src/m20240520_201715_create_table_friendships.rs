use entity::friendships::*;
use entity::users::{Column as UserColumn, Entity as UserEntity};
use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(Column::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Column::IdUser1).uuid().not_null())
                    .col(ColumnDef::new(Column::IdUser2).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user1-friendship")
                            .from(Entity, Column::IdUser1)
                            .to(UserEntity, UserColumn::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user2-friendship")
                            .from(Entity, Column::IdUser2)
                            .to(UserEntity, UserColumn::Id)
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
