use std::path::PathBuf;

use api::run_server;
use migration::{Migrator, MigratorTrait***REMOVED***
***REMOVED***ConnectOptions, Database***REMOVED***
use serde::Deserialize;

#[derive(Deserialize)]
struct Environment {
    static_dir: PathBuf,
***REMOVED***

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Environment(#[from] envy::Error),
    #[error(transparent)]
    Entity(#[from] entity::error::Error),
    #[error(transparent)]
    Db(#[from] sea_orm::error::DbErr),
***REMOVED***

#[tokio::main]
async fn main(***REMOVED***
    let env = envy::from_env::<Environment>()?;

    // Connect to the database
    let connect_options = ConnectOptions::new("sqlite://pgnpg.sqlite?mode=rwc");
    let db = Database::connect(connect_options).await?;

    // Run the migration
    Migrator::up(&db, None).await?;

    // Run the server
    run_server(api::RunServerConfig {
        db,
        static_dir: env.static_dir,
***REMOVED***)
***REMOVED***;

***REMOVED***
***REMOVED***
