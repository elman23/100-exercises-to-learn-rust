use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct TicketTitle(String);

impl TicketTitle {
    pub fn new(title: String) -> Self {
        TicketTitle(title)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Serialize, Deserialize)]
pub struct TicketDescription(String);

impl TicketDescription {
    pub fn new(description: String) -> Self {
        TicketDescription(description)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TicketId(u64);

impl TicketId {
    pub fn new(id: u64) -> Self {
        TicketId(id)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
