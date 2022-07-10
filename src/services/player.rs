use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    MysqlConnection, RunQueryDsl,
};

use crate::models::player::Player;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_player(
    player: Player,
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<Player, DbError> {
    use crate::schema::players::dsl::*;

    diesel::insert_into(players)
        .values(&player)
        .execute(&conn)?;

    Ok(player)
}
