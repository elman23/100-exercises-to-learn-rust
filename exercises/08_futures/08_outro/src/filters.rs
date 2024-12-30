use warp::Filter;
use super::{handlers, models};

pub fn list_sims(db: models::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any()
        .map(move || db.clone());

    let opt = warp::path::param::<u64>()
        .map(Some)
        .or_else(|_| async {
            //Ok(None)
            Ok::<(Option<u64>,), std::convert::Infallible>((None,))
        });
    
    warp::path!("holodeck" / ..)
        .and(opt)
        .and(warp::path::end())
        .and(db_map)
        .and_then(handlers::handle_list_sims)
}

pub fn post_sim(db: models::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any()
        .map(move || db.clone());
    warp::path!("holodeck")
        .and(warp::post())
        .and(json_body())
        .and(db_map)
        .and_then(handlers::handle_create_sim)
}

pub fn update_sim(db: models::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any()
        .map(move || db.clone());

    warp::path!("holodeck" / u64)
        .and(warp::put())
        .and(json_body_put())
        .and(db_map)
        .and_then(handlers::handle_update_sim)
}

pub fn delete_sim(db: models::Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_map = warp::any()
        .map(move || db.clone());

    warp::path!("holodeck" / u64)
        .and(warp::delete())
        .and(db_map)
        .and_then(handlers::handle_delete_sim)
}

fn json_body() -> impl Filter<Extract = (models::Simulation,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body_put() -> impl Filter<Extract = (models::Name,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}