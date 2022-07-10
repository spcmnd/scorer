use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};
use uuid::Uuid;

use crate::models::player::{Player, PlayerRequestBody, PlayerUpdateRequestBody};
use crate::services::player::{
    insert_new_player, remove_player, select_all_players, update_player,
};

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

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

#[get("/player")]
pub async fn get_players(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let db_conn = pool.get().expect("Couldn't get db connection from pool");
    let players = web::block(|| select_all_players(db_conn))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(players))
}

#[put("/player/{id}")]
pub async fn put_player(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    body: web::Json<PlayerUpdateRequestBody>,
) -> Result<HttpResponse, Error> {
    let db_conn = pool.get().expect("Couldn't get db connection from pool");
    let id = path.into_inner();
    let update_request = body.into_inner();
    let _result = web::block(|| update_player(id, update_request, db_conn)).await?;

    Ok(HttpResponse::NoContent().finish())
}

#[delete("/player/{id}")]
pub async fn delete_player(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let db_conn = pool.get().expect("Couldn't get db connection from pool");
    let id = path.into_inner();
    let _result = web::block(|| remove_player(id, db_conn)).await?;

    Ok(HttpResponse::NoContent().finish())
}
