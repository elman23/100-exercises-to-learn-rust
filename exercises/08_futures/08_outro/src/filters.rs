use super::{handlers, models};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub fn list(
    db: Arc<Mutex<models::TicketStore>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    let opt = warp::path::param::<u64>().map(Some).or_else(|_| async {
        //Ok(None)
        Ok::<(Option<u64>,), std::convert::Infallible>((None,))
    });

    warp::path!("tickets" / ..)
        .and(opt)
        .and(warp::path::end())
        .and(db_map)
        .and_then(handlers::handle_list)
}

pub fn post(
    db: Arc<Mutex<models::TicketStore>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());
    warp::path!("tickets")
        .and(warp::post())
        .and(json_body_post())
        .and(db_map)
        .and_then(handlers::handle_create)
}

pub fn update(
    db: Arc<Mutex<models::TicketStore>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    warp::path!("tickets" / u64)
        .and(warp::put())
        .and(json_body_put())
        .and(db_map)
        .and_then(handlers::handle_update)
}

pub fn delete(
    db: Arc<Mutex<models::TicketStore>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any().map(move || db.clone());

    warp::path!("tickets" / u64)
        .and(warp::delete())
        .and(db_map)
        .and_then(handlers::handle_delete)
}

fn json_body_post() -> impl Filter<Extract = (models::TicketDraft,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_put() -> impl Filter<Extract = (models::TicketPatch,), Error = warp::Rejection> + Clone
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
