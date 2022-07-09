use diesel::Queryable;
use serde::{Deserialize, Serialize};

use crate::schema::players;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, Insertable)]
pub struct Player {
    pub id: String,
    pub name: String,
}