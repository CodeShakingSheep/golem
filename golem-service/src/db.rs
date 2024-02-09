// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Connection, PgConnection, Pool, Postgres, Sqlite, SqliteConnection};
use tracing::info;

use crate::config::{DbPostgresConfig, DbSqliteConfig};

impl From<&DbPostgresConfig> for PgConnectOptions {
    fn from(config: &DbPostgresConfig) -> Self {
        PgConnectOptions::new()
            .host(config.host.as_str())
            .port(config.port)
            .database(config.database.as_str())
            .username(config.username.as_str())
            .password(config.password.as_str())
    }
}

impl From<&DbSqliteConfig> for SqliteConnectOptions {
    fn from(config: &DbSqliteConfig) -> Self {
        SqliteConnectOptions::new()
            .filename(std::path::Path::new(config.database.as_str()))
            .create_if_missing(true)
    }
}

pub async fn create_postgres_pool(
    config: &DbPostgresConfig,
) -> Result<Pool<Postgres>, Box<dyn Error>> {
    info!(
        "DB Pool: postgresql://{}:{}/{}",
        config.host, config.port, config.database
    );
    let conn_options = PgConnectOptions::from(config);

    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect_with(conn_options)
        .await
        .map_err(|e| e.into())
}

pub async fn postgres_migrate(config: &DbPostgresConfig) -> Result<(), Box<dyn Error>> {
    info!(
        "DB migration: postgresql://{}:{}/{}",
        config.host, config.port, config.database
    );
    let conn_options = PgConnectOptions::from(config);
    let mut conn = PgConnection::connect_with(&conn_options).await?;

    sqlx::migrate!("./db/migration/postgres")
        .run(&mut conn)
        .await?;

    let _ = conn.close().await;
    Ok(())
}

pub async fn create_sqlite_pool(config: &DbSqliteConfig) -> Result<Pool<Sqlite>, Box<dyn Error>> {
    info!("DB Pool: sqlite://{}", config.database);
    let conn_options = SqliteConnectOptions::from(config);

    SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect_with(conn_options)
        .await
        .map_err(|e| e.into())
}

pub async fn sqlite_migrate(config: &DbSqliteConfig) -> Result<(), Box<dyn Error>> {
    info!("DB migration: sqlite://{}", config.database);
    let conn_options = SqliteConnectOptions::from(config);
    let mut conn = SqliteConnection::connect_with(&conn_options).await?;
    sqlx::migrate!("./db/migration/sqlite")
        .run(&mut conn)
        .await?;
    let _ = conn.close().await;
    Ok(())
}
