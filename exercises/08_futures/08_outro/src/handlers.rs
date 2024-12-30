use models::Simulation;
use warp::http::StatusCode;
use std::convert::Infallible;
use super::models;

pub async fn handle_list_sims(opt: Option<u64>, db: models::Db) -> Result<impl warp::Reply, Infallible> {
    let mut result = db.lock().await.clone();
    if let Some(param) = opt {
        result.retain(|k| k.id == param);
    }
    Ok(warp::reply::json(&result))
}

pub async fn handle_create_sim(sim: models::Simulation, db: models::Db) -> Result<impl warp::Reply, Infallible> {
    let mut map = db.lock().await;

    if let Some(result) = map.get(&sim) {
        return Ok(warp::reply::with_status(
            format!("Simulation #{} already exists under the name {}", result.id, result.name),
            StatusCode::BAD_REQUEST
        ));
    }

    map.insert(sim.clone());
    Ok(warp::reply::with_status(format!("Simulation #{} created", sim.id), StatusCode::CREATED))
}    

pub async fn handle_update_sim(id: u64, name: models::Name, db: models::Db) -> Result<impl warp::Reply, Infallible> {
    if let Some(_) = db.lock().await.replace(Simulation { id, name: name.name }) {
        return Ok(warp::reply::with_status(
            format!("Simulation #{} was updated.\n", id),
            StatusCode::OK
        ));
    }
    Ok(warp::reply::with_status(
        format!("Simulation #{} was inserted.\n", id),
        StatusCode::CREATED
    ))
}

pub async fn handle_delete_sim(id: u64, db: models::Db) -> Result<impl warp::Reply, Infallible> {
    if db.lock().await.remove(&Simulation{id, name: String::new(),}){
        return Ok(warp::reply::with_status(
            format!("Simulation #{} was deleted", id), 
            StatusCode::OK,
        ))
    };

    Ok(warp::reply::with_status(
        format!("No data was deleted."),
        StatusCode::OK,
    ))
}   