use diesel::Queryable;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::players;

#[derive(Debug, Clone, Queryable, Serialize, Insertable)]
pub struct Player {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct PlayerRequestBody {
    #[validate(required, length(min = 3, max = 32))]
    pub name: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct PlayerUpdateRequestBody {
    #[validate(required, length(min = 3, max = 32))]
    pub name: Option<String>,
}
