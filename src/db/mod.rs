// SPDX-FileCopyrightText: 2025 Gronner <gronner@mailbox.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncMigrationHarness, AsyncPgConnection,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

mod models;
pub mod operations;
mod schema;
mod trait_util;

pub type DbPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_connection_pool() -> DbPool {
    // TODO: Change hardcoded link with see issue #1
    let db_config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        "postgres://postgres:password@localhost:5433/postgres",
    );
    let pool = bb8::Pool::builder().build(db_config).await.unwrap();
    let mut harness = AsyncMigrationHarness::new(pool.get_owned().await.unwrap());
    harness.run_pending_migrations(MIGRATIONS).unwrap();
    pool
}
