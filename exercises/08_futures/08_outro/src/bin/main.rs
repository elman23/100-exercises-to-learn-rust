// I made "models" and "filters" public in lib.rs
use std::sync::Arc;
use ticket_api::{db, filters};
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(db::TicketStore::read_from_data()));
    let routes = filters::get_one(db.clone())
        .or(filters::post(db.clone()))
        .or(filters::update(db.clone()))
        .or(filters::list(db.clone()))
        .or(filters::delete(db));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
