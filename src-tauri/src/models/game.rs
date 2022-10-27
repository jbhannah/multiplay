use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub platform_id: i64,
    pub shasum: String,
}
