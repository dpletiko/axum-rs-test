use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

use crate::error_handler::AppError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub async fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");

    run_db_migrations(&mut conn);
}

pub fn connection() -> Result<DbConnection, anyhow::Error> {
    POOL.get()
        .map_err(|e| anyhow::anyhow!(e))
        // .map_err(|e| AppError::from(e))
}

type DB = diesel::pg::Pg;
pub fn run_db_migrations(conn: &mut impl MigrationHarness<DB>) {
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run db migrations");
}
