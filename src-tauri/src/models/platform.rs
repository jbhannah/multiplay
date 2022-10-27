use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    pub acronmym: String,
    pub rom_extensions: String,
    pub save_extensions: String,
}
