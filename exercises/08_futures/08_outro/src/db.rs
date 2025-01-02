use std::collections::BTreeMap;

use crate::models::{Status, Ticket, TicketDraft, TicketId, TicketPatch};

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId::new(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn patch(&mut self, id: u64, ticket_patch: TicketPatch) -> bool {
        let ticket = self.get_mut(TicketId::new(id));
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
            return true;
        }
        false
    }

    pub fn get_all(&self) -> Vec<&Ticket> {
        self.tickets.values().collect()
    }

    pub fn del(&mut self, id: &TicketId) -> bool {
        if self.tickets.contains_key(id) {
            self.tickets.remove(id);
            return true;
        }
        false
    }
}
