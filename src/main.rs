use api::run_server;
use entity::error::Error;
use migration::{Migrator, MigratorTrait***REMOVED***
***REMOVED***ConnectOptions, Database***REMOVED***

#[tokio::main]
async fn main(***REMOVED***
    // Connect to the database
    let connect_options = ConnectOptions::new("sqlite://pgnpg.sqlite?mode=rwc");
    let db = Database::connect(connect_options).await?;

    // Run the migration
    Migrator::up(&db, None).await?;

    // Run the server
    run_server(db).await;

***REMOVED***
***REMOVED***
