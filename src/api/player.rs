use actix_web::{post, web, Error, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::player::Player;
use crate::services::player::insert_new_player;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRequestBody {
    name: String,
}

#[post("/player")]
pub async fn post_player(
    pool: web::Data<DbPool>,
    body: web::Json<PlayerRequestBody>,
) -> Result<HttpResponse, Error> {
    let new_player = Player {
        id: Uuid::new_v4().to_string(),
        name: body.into_inner().name,
    };
    let db_conn = pool.get().expect("Couldn't get db connection from pool");

    let player = web::block(move || insert_new_player(new_player, db_conn))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(player))
}
