use std::sync::Mutex;

use crate::schema::platforms::dsl::*;
use diesel::{migration::Result, prelude::*, Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Default)]
pub struct Library {
    pub db: Mutex<Option<SqliteConnection>>,
}

impl Library {
    pub fn connect(&self, database_url: &str) -> Result<()> {
        let mut db = SqliteConnection::establish(database_url)?;

        if db.has_pending_migration(MIGRATIONS)? {
            db.run_pending_migrations(MIGRATIONS)?;
        }

        *self.db.lock().unwrap() = Some(db);
        Ok(())
    }

    pub fn rom_extensions(&self) -> QueryResult<Vec<String>> {
        let mut binding = self.db.lock().unwrap();
        let db = binding.as_mut().unwrap();

        Ok(platforms
            .select(rom_extensions)
            .load::<String>(db)?
            .into_iter()
            .flat_map(|extensions| {
                extensions
                    .split(',')
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .collect())
    }
}
