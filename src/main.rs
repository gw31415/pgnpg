use std::{path::PathBuf, sync::Arc};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;

#[derive(Deserialize)]
struct Environment {
    static_dir: PathBuf,
    fetch_url: Arc<str>,
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
    server::run(server::Config {
        db,
        static_dir: env.static_dir,
        fetch_url: env.fetch_url,
    })
    .await;

    Ok(())
}
