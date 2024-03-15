use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Podcast::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Podcast::Id)
                            .big_integer()
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Podcast::Name).string().not_null())
                    .col(ColumnDef::new(Podcast::Description).string())
                    .col(ColumnDef::new(Podcast::Url).string().not_null())
                    .col(ColumnDef::new(Podcast::ImageUrl).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Podcast::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Podcast {
    #[sea_orm(iden = "podcasts")]
    Table,
    Id,
    Name,
    Description,
    Url,
    ImageUrl,
}
