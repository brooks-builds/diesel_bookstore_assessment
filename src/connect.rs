use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use eyre::{Context, Result};
use std::env;

pub fn connect() -> Result<PgConnection> {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").context("extracting DATABASE_URL environment variable")?;

    PgConnection::establish(&database_url).context("Connecting to Postgres database")
}
