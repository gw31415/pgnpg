use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;

#[derive(Deserialize)]
struct Environment {
    #[serde(flatten)]
    server_config: server::Config,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Environment(#[from] envy::Error),
    #[error(transparent)]
    Entity(#[from] entity::error::Error),
    #[error(transparent)]
    Db(#[from] sea_orm::error::DbErr),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = envy::from_env::<Environment>()?;

    // Connect to the database
    let connect_options = ConnectOptions::new("sqlite://pgnpg.sqlite?mode=rwc");
    let db = Database::connect(connect_options).await?;

    // Run the migration
    Migrator::up(&db, None).await?;

    // Run the server
    server::run(db, env.server_config).await;

    Ok(())
}
