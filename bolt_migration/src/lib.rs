pub use sea_orm_migration::prelude::*;

mod m20240223_123539_create_podcasts_table;
mod m20240223_123546_create_episodes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240223_123539_create_podcasts_table::Migration),
            Box::new(m20240223_123546_create_episodes_table::Migration),
        ]
    }
}
