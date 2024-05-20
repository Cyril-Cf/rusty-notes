use sea_orm_migration::prelude::*;
use entity::customer::*;

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
                    .col(ColumnDef::new(Column::Firstname).string().not_null())
                    .col(ColumnDef::new(Column::Lastname).string().not_null())
                    .col(ColumnDef::new(Column::Email).string().not_null())
                    .col(ColumnDef::new(Column::KeycloakId).string().not_null())
                    .col(ColumnDef::new(Column::IdCustomer).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-customer-customer-id")
                            .from(Entity, Column::Id)
                            .to(Entity, Column::IdCustomer)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::NoAction),
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
