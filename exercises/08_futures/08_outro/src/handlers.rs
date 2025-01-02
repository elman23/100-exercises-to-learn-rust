use crate::models::{Ticket, TicketId};

use super::models;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

pub async fn handle_list(
    maybe_id: Option<u64>,
    db: Arc<Mutex<models::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let store = db.lock().await.clone();
    if let Some(param) = maybe_id {
        if let Some(ticket) = store.get(models::TicketId::new(param)) {
            Ok(warp::reply::json(&ticket))
        } else {
            let tickets: Vec<Ticket> = Vec::new();
            Ok(warp::reply::json(&tickets))
        }
    } else {
        let tickets = store.get_all();
        Ok(warp::reply::json(&tickets))
    }
}

pub async fn handle_create(
    ticket_draft: models::TicketDraft,
    db: Arc<Mutex<models::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    let id = store.add_ticket(ticket_draft);
    Ok(warp::reply::json(&id))
}

pub async fn handle_update(
    id: u64,
    ticket_patch: models::TicketPatch,
    db: Arc<Mutex<models::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    let ticket = store.get_mut(TicketId::new(id));
    if let Some(ticket) = ticket {
        if let Some(title) = ticket_patch.title {
            ticket.title = title;
        }
        if let Some(description) = ticket_patch.description {
            ticket.description = description;
        }
        if let Some(status) = ticket_patch.status {
            ticket.status = status;
        }
        return Ok(StatusCode::OK);
    }
    Ok(StatusCode::NOT_FOUND)
}

pub async fn handle_delete(
    id: u64,
    db: Arc<Mutex<models::TicketStore>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut store = db.lock().await;
    if store.del(&TicketId::new(id)) {
        return Ok(StatusCode::OK);
    };

    Ok(StatusCode::NOT_FOUND)
}
