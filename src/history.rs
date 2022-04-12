use std::io::Write;

use log::{warn, info, debug};
use serde::{Serialize, Deserialize};
use rand::{Rng, prelude::SliceRandom};
use uuid::Uuid;

use crate::genetics::NUM_CODONS;

use super::Agent;

pub struct History {
    header: Header,
    write: Box<dyn Write>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentEntry {
    id: Uuid,
    parent: Option<Uuid>,
    survived: bool,
    genome: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub id: Uuid,
    pub revived_from: Option<Uuid>,
    pub revived_generation: Option<usize>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: uuid::Uuid::new_v4(),
            revived_from: None,
            revived_generation: None,
        }
    }

    pub fn clone(&self) -> Header {
        Header {
            id: uuid::Uuid::new_v4(),
            revived_from: self.revived_from,
            revived_generation: self.revived_generation,
        }
    }
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

    pub fn from_log_entry(entry: AgentEntry) -> Agent {
        let mut genome = [0.0; NUM_CODONS];
        for i in 0..NUM_CODONS {
            genome[i] = entry.genome[i];
        }

        let mut rng = rand::thread_rng();
        Agent {
            uuid: entry.id,
            parent: entry.parent,
            brain: super::genetics::create_brain(&genome),
            genome,
            position: (
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
            ),
        }
    }
}

impl History {
    pub fn new(header: Header) -> History {
        let header = Header::new();

        let mut write = std::fs::File::create(format!("output/{}.log", header.id)).unwrap();
        writeln!(write, "{}", serde_json::to_string(&header).unwrap()).unwrap();

        History {
            header,
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
    
    pub fn revive(path: &str, generation: Option<usize>) -> (Vec<Agent>, super::Settings, Header) {
        use std::io::BufRead;
        info!("reviving {}, generation {:?}...", path, generation);

        // Awful, sorry.
        let file = std::fs::File::open(path).unwrap();
        let lines = std::io::BufReader::new(file).lines();
        let mut header: Option<Header> = None;
        let mut settings: Option<super::Settings> = None;
        let mut agents: Vec<AgentEntry> = vec![];

        let mut last_gen_agents: Vec<AgentEntry> = vec![];
        let mut last_gen_settings = None;
        let mut last_gen = 0;

        for line in lines {
            if let Ok(line) = line {
                if header.is_none() {
                    let parent_header = Some(serde_json::from_str::<Header>(&line).unwrap()).unwrap();
                    header = Some(Header {
                        id: Uuid::new_v4(),
                        revived_from: Some(parent_header.id),
                        revived_generation: Some(0),
                    });
                }
                else if line.starts_with(":") {
                    if let Some(settings) = &settings {
                        if agents.len() != settings.num_agents {
                            warn!("missing agents in log");
                        }
                    }

                    let sep = line.find(" ").unwrap();
                    let next_generation = *&line[1..sep].parse::<usize>().unwrap();
                    let next_settings = serde_json::from_str::<super::Settings>(&line[(sep+1)..]).unwrap();
                    debug!("loading generation {}", next_generation);

                    if let Some(generation) = generation {
                        if generation < next_generation {
                            debug!("found generation {}", generation);
                            // We've found the generation we're looking for.
                            break;
                        }
                    }
                    // Else, set as current
                    last_gen_agents = agents;
                    agents = vec![];
                    last_gen_settings = settings;
                    settings = Some(next_settings);
                    last_gen = header.as_ref().unwrap().revived_generation.unwrap();
                    header.as_mut().unwrap().revived_generation = Some(next_generation);
                }
                else {
                    // Read agent!
                    agents.push(serde_json::from_str::<AgentEntry>(&line).unwrap());
                }
            }
            else {
                break;
            }
        }

        debug!("reached end of file");
        // If we've loaded the correct number of agents, we're done;
        // otherwise, we return the last generation that was complete.
        if agents.len() != settings.as_ref().unwrap().num_agents {
            debug!("generation incomplete");
            agents = last_gen_agents;
            settings = last_gen_settings;
            header.as_mut().unwrap().revived_generation = Some(last_gen);
        }
        else {
            debug!("loaded last generation");
        }

        (
            agents.into_iter().map(|entry| Agent::from_log_entry(entry)).collect(),
            settings.unwrap(),
            header.unwrap(),
        )
    }
}