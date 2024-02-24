use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Show::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Show::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Show::Name).string().not_null())
                    .col(ColumnDef::new(Show::Description).string())
                    .col(ColumnDef::new(Show::Url).string().not_null())
                    .col(ColumnDef::new(Show::ImageUrl).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Show::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Show {
    Table,
    Id,
    Name,
    Description,
    Url,
    ImageUrl,
}
