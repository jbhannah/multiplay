use diesel::{migration::Result, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub struct Library {
    db: SqliteConnection,
}

impl Library {
    pub fn new(database_url: &str) -> Result<Self> {
        let mut db = SqliteConnection::establish(database_url)?;

        if db.has_pending_migration(MIGRATIONS)? {
            db.run_pending_migrations(MIGRATIONS)?;
        }

        Ok(Self { db })
    }
}
