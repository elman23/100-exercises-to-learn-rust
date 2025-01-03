use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketPatch {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Status {
    pub fn parse_from_str(status: &str) -> Status {
        match status {
            "ToDo" => Status::ToDo,
            "InProgress" => Status::InProgress,
            "Done" => Status::Done,
            _ => panic!("Invalid status!"),
        }
    }
}
