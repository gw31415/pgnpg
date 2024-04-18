pub use sea_orm_migration::prelude::*;

mod m20240410_000001_create_table;
mod m20240418_000002_create_mstdn_token_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240410_000001_create_table::Migration),
            Box::new(m20240418_000002_create_mstdn_token_table::Migration),
        ]
    }
}
