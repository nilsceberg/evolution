use std::io::Write;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::Agent;

pub struct History {
    id: Uuid,
    write: Box<dyn Write>
}

#[derive(Serialize, Deserialize)]
pub struct AgentEntry {
    id: Uuid,
    parent: Option<Uuid>,
    survived: bool,
    genome: Vec<f32>,
}

impl Agent {
    pub fn to_log_entry(&self, survived: bool) -> AgentEntry {
        AgentEntry {
            id: self.uuid,
            parent: self.parent,
            genome: self.genome.to_vec(),
            survived,
        }
    }
}

impl History {
    pub fn new() -> History {
        let id = uuid::Uuid::new_v4();

        let write = std::fs::File::create(format!("output/{}.log", id)).unwrap();

        History {
            id,
            write: Box::new(write),
        }
    }

    pub fn log_generation(&mut self, number: usize, settings: &super::Settings) {
        // Make sure last generation is flushed
        self.write.flush().unwrap();
        writeln!(&mut self.write, ":{} {}", number, serde_json::to_string(&settings).unwrap()).unwrap();
    }

    pub fn log_agent(&mut self, entry: AgentEntry) {
        writeln!(&mut self.write, "{}", serde_json::to_string(&entry).unwrap()).unwrap();
    }
}