use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};

use crate::models::player::{Player, PlayerUpdateRequestBody};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_player(
    player: Player,
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<Player, DbError> {
    use crate::schema::players::dsl::*;

    diesel::insert_into(players)
        .values(&player)
        .execute(&conn)?;

    // TODO: Handle error like unique constraints

    Ok(player)
}

pub fn select_all_players(
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<Vec<Player>, diesel::result::Error> {
    use crate::schema::players::dsl::*;

    players.load::<Player>(&conn)
}

pub fn update_player(
    existing_player_uuid: String,
    update_request: PlayerUpdateRequestBody,
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::players::dsl::*;

    let player = players.filter(id.eq(existing_player_uuid));
    let result = diesel::update(player)
        .set(name.eq(update_request.name))
        .execute(&conn);

    result
}

pub fn remove_player(
    player_uuid: String,
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::players::dsl::*;

    let player = players.filter(id.eq(player_uuid));
    let result = diesel::delete(player).execute(&conn);

    result
}
