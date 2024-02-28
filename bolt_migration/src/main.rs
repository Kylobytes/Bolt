use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(bolt_migration::Migrator).await
}
