use sea_orm_migration::prelude::*;

use crate::m20240223_123539_create_shows_table::Show;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Episode::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Episode::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Episode::Title).string().not_null())
                    .col(ColumnDef::new(Episode::Description).string())
                    .col(ColumnDef::new(Episode::Url).string())
                    .col(ColumnDef::new(Episode::ImageUrl).string())
                    .col(ColumnDef::new(Episode::MediaUrl).string())
                    .col(
                        ColumnDef::new(Episode::Queued)
                            .boolean()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(
                        ColumnDef::new(Episode::DatePublished)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Episode::ShowId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Episode::Table, Episode::ShowId)
                            .to(Show::Table, Show::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Episode::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Episode {
    Table,
    Id,
    Title,
    Description,
    Url,
    ImageUrl,
    MediaUrl,
    Queued,
    DatePublished,
    ShowId,
}
