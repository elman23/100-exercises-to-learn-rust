use crate::models::{Status, Ticket, TicketDraft, TicketPatch};
use std::fs;

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<u64, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn read_from_data() -> Self {
        let mut counter: u64 = 1;
        let mut tickets: BTreeMap<u64, Ticket> = BTreeMap::new();
        let data: String = fs::read_to_string("./data/tickets.csv").unwrap();
        let mut reader = csv::Reader::from_reader(data.as_bytes());
        for record in reader.records() {
            let record = record.unwrap();
            let ticket = Ticket {
                id: record[0].parse::<u64>().unwrap(),
                title: record[1].to_string(),
                description: record[2].to_string(),
                status: Status::parse_from_str(&record[3]),
            };
            tickets.insert(ticket.id, ticket);
            counter += 1;
        }
        TicketStore { tickets, counter }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> u64 {
        let id = self.counter;
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

    pub fn get(&self, id: u64) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }

    pub fn patch(&mut self, id: u64, ticket_patch: TicketPatch) -> bool {
        let ticket = self.get_mut(id);
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

    pub fn del(&mut self, id: &u64) -> bool {
        if self.tickets.contains_key(id) {
            self.tickets.remove(id);
            return true;
        }
        false
    }
}
