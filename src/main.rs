use rand::{Rng, prelude::SliceRandom};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::{info};

mod dot;
mod viewer;
mod brain;
mod genetics;
mod history;

const WORLD_RADIUS: f32 = 500.0;

use brain::Brain;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SimulationMode {
    SafeZoneRace {
        radius_low: f32,
        radius_high: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub radius: f32,
    pub title: String,
    pub zone: Option<Zone>,
    pub mutation_rate: f32,
    pub mutation_strength: f32,
    pub num_agents: usize,
    pub time_step: f32,
    pub frame_interval: u32,
    pub mode: SimulationMode,
    pub generation_time: f32,
}

pub struct Agent {
    uuid: Uuid,
    parent: Option<Uuid>,
    position: (f32, f32),
    genome: genetics::Genome,
    brain: brain::Brain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zone {
    x: f32,
    y: f32,
    radius: f32,
}

impl Zone {
    fn random(world_radius: f32, radius: std::ops::Range<f32>) -> Zone {
        let mut rng = rand::thread_rng();
        let r = world_radius * rng.gen::<f32>(); //.sqrt();
        let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let radius = rng.gen_range(radius);
        Zone {
            radius,
            x: r * theta.cos(),
            y: r * theta.sin(),
        }
    }

    fn contains(&self, position: (f32, f32)) -> bool {
        (position.0 - self.x).powf(2.0) + (position.1 - self.y).powf(2.0) < self.radius.powf(2.0)
    }
}

fn keep_inside_radius(mut position: (f32, f32), radius: f32) -> (f32, f32) {
    let length = (position.0.powf(2.0) + position.1.powf(2.0)).sqrt();
    if length > radius {
        let correction = radius / length;
        position.0 *= correction;
        position.1 *= correction;
    }
    position
}

impl Agent {
    fn new() -> Agent {
        let mut rng = rand::thread_rng();
        let genome = genetics::randomize();
        let brain = genetics::create_brain(&genome);
        Agent {
            genome,
            brain,
            uuid: Uuid::new_v4(),
            parent: None,
            position: (
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
            ),
        }
    }

    fn simulate(&mut self, time: f32, safe_zone: &Zone) {
        self.brain.input(brain::Input::Constant, 1.0);
        self.brain.input(brain::Input::Oscillator, time * std::f32::consts::TAU);
        self.brain.input(brain::Input::X, self.position.0);
        self.brain.input(brain::Input::Y, self.position.1);
        self.brain.input(brain::Input::SafeX, safe_zone.x);
        self.brain.input(brain::Input::SafeY, safe_zone.y);
        self.brain.input(brain::Input::SafeRadius, safe_zone.radius);
        self.brain.simulate();
        self.position.0 += self.brain.output(brain::Output::SpeedX);
        self.position.1 += self.brain.output(brain::Output::SpeedY);
        self.position = keep_inside_radius(self.position, WORLD_RADIUS);
    }

    fn procreate(&self, rate: f32, strength: f32) -> Agent {
        let mut rng = rand::thread_rng();
        let mut genome = self.genome.clone();
        genetics::mutate(&mut genome, rate, strength);
        let brain = genetics::create_brain(&genome);
        Agent {
            genome,
            brain,
            uuid: Uuid::new_v4(),
            parent: Some(self.uuid),
            position: (
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
            ),
        }
    }
}

fn main() {
    use simplelog::{CombinedLogger, TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            //WriteLogger::new(LevelFilter::Info, Config::default(), File::create("run.log").unwrap()),
        ]
    ).unwrap();

    let enable_viewer = true;

    let viewer = if enable_viewer {
        viewer::start_viewer()
    }
    else {
        viewer::ViewerHandle::Disabled
    };

    let mut settings = Settings {
        radius: WORLD_RADIUS,
        title: "".to_string(),
        num_agents: 100,
        zone: None,
        mutation_rate: 0.1,
        mutation_strength: 0.25,
        frame_interval: 5,
        time_step: 0.05,
        generation_time: 50.0,
        mode: SimulationMode::SafeZoneRace { radius_low: 50.0, radius_high: 100.0 },
    };

    let mut log = history::History::new();

    let mut rng = rand::thread_rng();
    let mut agents : Vec<Agent> = vec![];

    let mut generation = 1;
    match settings.mode {
        SimulationMode::SafeZoneRace { radius_low, radius_high } => {
            loop {
                if agents.is_empty() {
                    info!("seeding...");
                    for _ in 0..settings.num_agents {
                        agents.push(Agent::new());
                    }
                }

                info!("simulating generation {} for {} seconds", generation, settings.generation_time);
                log.log_generation(generation, &settings);

                let safe_zone = Zone::random(WORLD_RADIUS, radius_low..radius_high);

                settings.title = format!("Generation {}", generation);
                settings.zone = Some(safe_zone.clone());
                viewer.publish(viewer::Event::Settings(settings.clone()));
                viewer.publish(viewer::Event::Clear);
                viewer.publish(viewer::spawn(&agents));

                let mut time: f32 = 0.0;
                while time < settings.generation_time {
                    std::thread::sleep(std::time::Duration::from_millis(settings.frame_interval.into()));
                    time += settings.time_step;

                    viewer.publish(viewer::frame(&agents)); 

                    for agent in &mut agents {
                        agent.simulate(time, &safe_zone);
                    }
                }

                // Impose selection!
                let mut survivors = vec![];
                for agent in agents {
                    let survived = safe_zone.contains(agent.position);
                    log.log_agent(agent.to_log_entry(survived));
                    if survived {
                        survivors.push(agent);
                    }
                }

                agents = vec![];
                if survivors.is_empty() {
                    info!("no survivors, reseeding");
                    generation = 1;
                    log = history::History::new();
                }
                else {
                    info!("{} survivors", survivors.len());

                    // Randomly pick a survivor to procreate until we reach cap.
                    while agents.len() < settings.num_agents {
                        agents.push(survivors.choose(&mut rng).unwrap().procreate(settings.mutation_rate, settings.mutation_strength));
                    }

                    generation += 1;
                }
            }
        }
    }
}
