use crate::models::Ticket;

use super::db;
use super::models;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

pub async fn handle_id(
    id: u64,
    db: Arc<Mutex<db::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await.clone();
    if let Some(ticket) = store.get(id) {
        Ok(warp::reply::json(&ticket))
    } else {
        let tickets: Vec<Ticket> = Vec::new();
        Ok(warp::reply::json(&tickets))
    }
}

pub async fn handle_list(db: Arc<Mutex<db::TicketStore>>) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await.clone();
    let tickets = store.get_all();
    Ok(warp::reply::json(&tickets))
}

pub async fn handle_create(
    ticket_draft: models::TicketDraft,
    db: Arc<Mutex<db::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    let id = store.add_ticket(ticket_draft);
    Ok(warp::reply::json(&id))
}

pub async fn handle_update(
    id: u64,
    ticket_patch: models::TicketPatch,
    db: Arc<Mutex<db::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    if store.patch(id, ticket_patch) {
        return Ok(StatusCode::OK);
    }
    Ok(StatusCode::NOT_FOUND)
}

pub async fn handle_delete(
    id: u64,
    db: Arc<Mutex<db::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    if store.del(&id) {
        return Ok(StatusCode::OK);
    };

    Ok(StatusCode::NOT_FOUND)
}
