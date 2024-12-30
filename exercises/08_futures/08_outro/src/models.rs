use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Simulation {
    pub id: u64,
    pub name: String,
}

impl PartialEq for Simulation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Simulation { }

impl Hash for Simulation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub fn get_simulation<'a>(sims: &'a HashSet<Simulation>, id: u64) -> Option<&'a Simulation> {
    sims.get(&Simulation {id, name: String::new()})
}

pub type Db = Arc<Mutex<HashSet<Simulation>>>;

pub fn new_db() -> Db {
    Arc::new(Mutex::new(HashSet::new()))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Name {
    pub name: String
}