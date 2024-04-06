use sea_orm_migration::prelude::*;

use crate::m20240223_123539_create_podcasts_table::Podcast;

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
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Episode::Guid).string().not_null().unique_key())
                    .col(ColumnDef::new(Episode::Title).string().not_null())
                    .col(ColumnDef::new(Episode::Description).string())
                    .col(ColumnDef::new(Episode::Url).string().not_null())
                    .col(ColumnDef::new(Episode::ImageUrl).string())
                    .col(
                        ColumnDef::new(Episode::EnclosureUrl)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Episode::Queued)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(
                        ColumnDef::new(Episode::DatePublished)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Episode::PodcastId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Episode::Table, Episode::PodcastId)
                            .to(Podcast::Table, Podcast::Id)
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
    #[sea_orm(iden = "episodes")]
    Table,
    Id,
    Guid,
    Title,
    Description,
    Url,
    ImageUrl,
    EnclosureUrl,
    Queued,
    DatePublished,
    PodcastId,
}
