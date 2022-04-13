use rand::{Rng, prelude::SliceRandom};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::{info};
use clap::Parser;

mod dot;
mod viewer;
mod brain;
mod genetics;
mod history;

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
    pub world_radius: f32,
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

#[derive(Parser)]
pub struct Args {
    /// Override probability that a weight will be changed.
    #[clap(long)]
    mutation_rate: Option<f32>,
    
    /// Override strength factor of each mutation.
    #[clap(long)]
    mutation_strength: Option<f32>,
    
    /// Override safe zone size lower bound.
    #[clap(long)]
    safe_lower: Option<f32>,
    
    /// Override safe zone radius upper bound.
    #[clap(long)]
    safe_upper: Option<f32>,
    
    /// Override generation time.
    #[clap(long)]
    generation_time: Option<f32>,

    /// Seed from log file.
    #[clap(long)]
    revive: Option<String>,

    /// If seeding from log file, use this generation.
    #[clap(long, requires = "revive")]
    generation: Option<usize>,
    
    /// Enable viewer
    #[clap(long)]
    viewer: bool,

    /// Disable logging (not implemented)
    #[clap(long)]
    no_log: bool,
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
        let r = (world_radius - radius.end) * rng.gen::<f32>().sqrt();
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
        let genome = genetics::randomize();
        let brain = genetics::create_brain(&genome);
        let mut rng = rand::thread_rng();
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

    fn simulate(&mut self, time: f32, world_radius: f32, safe_zone: &Zone) {
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
        self.position = keep_inside_radius(self.position, world_radius);
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

    fn clone(&self) -> Agent {
        let mut rng = rand::thread_rng();
        Agent {
            brain: genetics::create_brain(&self.genome),
            genome: self.genome,
            uuid: Uuid::new_v4(),
            parent: None,
            position: (
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
            ),
        }
    }
}

fn override_setting<T: Clone>(setting: &mut T, option: &Option<T>) {
    if let Some(value) = option {
        *setting = value.clone();
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

    let args = Args::parse();

    let viewer = if args.viewer {
        viewer::start_viewer()
    }
    else {
        viewer::ViewerHandle::Disabled
    };

    let (start_agents, mut start_settings, start_header) = if let Some(filename) = args.revive {
        history::History::revive(&filename, args.generation)
    }
    else {
        let mut agents = vec![];
        let settings = Settings {
            world_radius: 500.0,
            title: "".to_string(),
            num_agents: 100,
            zone: None,
            mutation_rate: 0.03,
            mutation_strength: 0.25,
            frame_interval: 5,
            time_step: 0.05,
            generation_time: 50.0,
            mode: SimulationMode::SafeZoneRace { radius_low: 50.0, radius_high: 100.0 },
        };

        for _ in 0..settings.num_agents {
            agents.push(Agent::new());
        }

        (
            agents,
            settings,
            history::Header::new(),
        )
    };

    // Override start settings with args:
    override_setting(&mut start_settings.mutation_rate, &args.mutation_rate);
    override_setting(&mut start_settings.mutation_strength, &args.mutation_strength);
    override_setting(&mut start_settings.generation_time, &args.generation_time);
    match &mut start_settings.mode {
        SimulationMode::SafeZoneRace { radius_low, radius_high } => {
            override_setting(radius_low, &args.safe_lower);
            override_setting(radius_high, &args.safe_upper);
        }
    }

    let mut agents : Vec<Agent> = vec![];
    let mut settings = start_settings;

    let mut log = history::History::new(start_header.clone());

    let mut rng = rand::thread_rng();
    let mut generation = start_header.revived_generation.unwrap_or(1);
    match settings.mode {
        SimulationMode::SafeZoneRace { radius_low, radius_high } => {
            loop {
                if agents.is_empty() {
                    info!("seeding...");
                    for agent in &start_agents {
                        agents.push(agent.clone());
                    }
                }

                info!("simulating generation {} for {} seconds", generation, settings.generation_time);
                log.log_generation(generation, &settings);


                settings.title = format!("Generation {}", generation);

                // If we want to load safe zone from history:
                //let safe_zone = match settings.zone {
                //    Some(zone) => zone,
                //    None => {
                //        let safe_zone = Zone::random(settings.world_radius, radius_low..radius_high);
                //        safe_zone
                //    }
                //};
                let mut safe_zone = Zone::random(settings.world_radius, radius_low..radius_high);
                settings.zone = Some(safe_zone.clone());

                viewer.publish(viewer::Event::Settings(settings.clone()));
                viewer.publish(viewer::Event::Clear);
                viewer.publish(viewer::spawn(&agents));

                let mut time: f32 = 0.0;
                while time < settings.generation_time {
                    if args.viewer {
                        std::thread::sleep(std::time::Duration::from_millis(settings.frame_interval.into()));
                    }

                    let last_time = time;
                    time += settings.time_step;

                    if last_time <= settings.generation_time / 2.0 && time > settings.generation_time / 2.0 {
                        // Re-place safe zone
                        safe_zone = Zone::random(settings.world_radius, radius_low..radius_high);
                        settings.zone = Some(safe_zone.clone());
                        viewer.publish(viewer::Event::Settings(settings.clone()));
                    }

                    viewer.publish(viewer::frame(&agents)); 

                    for agent in &mut agents {
                        agent.simulate(time, settings.world_radius, &safe_zone);
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
                    generation = start_header.revived_generation.unwrap_or(1);
                    log = history::History::new(start_header.clone());
                }
                else {
                    info!("{} survivors", survivors.len());

                    // Randomly pick a survivor to procreate until we reach cap.
                    while agents.len() < settings.num_agents {
                        agents.push(survivors.choose(&mut rng).unwrap().procreate(settings.mutation_rate, settings.mutation_strength));
                    }

                    generation += 1;
                }

                // Clear zone for next run.
                settings.zone = None;
            }
        }
    }
}
