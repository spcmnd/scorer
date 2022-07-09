use actix_web::{post, web, Error, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::player;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRequestBody {
    name: String,
}

fn insert_new_player(
    player: player::Player,
    conn: PooledConnection<ConnectionManager<MysqlConnection>>,
) -> Result<player::Player, DbError> {
    use crate::schema::players::dsl::*;

    diesel::insert_into(players)
        .values(&player)
        .execute(&conn)?;

    Ok(player)
}

#[post("/player")]
pub async fn post_player(
    pool: web::Data<DbPool>,
    body: web::Json<PlayerRequestBody>,
) -> Result<HttpResponse, Error> {
    let new_player = player::Player {
        id: Uuid::new_v4().to_string(),
        name: body.into_inner().name,
    };
    let db_conn = pool.get().expect("Couldn't get db connection from pool");

    let player = web::block(move || insert_new_player(new_player, db_conn))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(player))
}
