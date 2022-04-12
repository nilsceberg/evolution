use rand::{Rng, prelude::SliceRandom};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use log::{info};

mod dot;
mod viewer;
mod brain;
mod genetics;

const WORLD_RADIUS: f32 = 500.0;

use brain::Brain;

pub struct Agent {
    uuid: Uuid,
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
        let r = world_radius * rng.gen::<f32>().sqrt();
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

    fn procreate(&self) -> Agent {
        let mut rng = rand::thread_rng();
        let mut genome = self.genome.clone();
        genetics::mutate(&mut genome, 0.01);
        let brain = genetics::create_brain(&genome);
        Agent {
            genome,
            brain,
            uuid: Uuid::new_v4(),
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

    let publisher = viewer::start_viewer();
    publisher.send(viewer::Event::Clear).unwrap();

    const NUM_AGENTS: usize = 100;
    const GENERATION_TIME: f32 = 50.0;

    let mut rng = rand::thread_rng();
    let mut agents : Vec<Agent> = vec![];
    for _ in 0..NUM_AGENTS {
        agents.push(Agent::new());
    }

    let mut generation = 1;
    loop {
        info!("simulating generation {} for {} seconds", generation, GENERATION_TIME);

        let safe_zone = Zone::random(WORLD_RADIUS, 50.0..100.0);

        publisher.send(viewer::Event::Settings(viewer::Settings {
            radius: WORLD_RADIUS,
            zone: Some(safe_zone.clone()),
        })).unwrap();
        publisher.send(viewer::Event::Clear).unwrap();
        publisher.send(viewer::spawn(&agents)).unwrap();

        let mut time: f32 = 0.0;
        while time < GENERATION_TIME {
            std::thread::sleep(std::time::Duration::from_millis(10));
            time += 0.050;

            publisher.send(viewer::frame(&agents)).unwrap(); 

            for agent in &mut agents {
                agent.simulate(time, &safe_zone);
            }
        }

        // Impose selection!
        let mut survivors = vec![];
        for agent in agents {
            if safe_zone.contains(agent.position) {
                survivors.push(agent);
            }
        }

        if survivors.is_empty() {
            info!("no survivors, exiting");
            break;
        }
        else {
            info!("{} survivors", survivors.len());
        }

        // Randomly pick a survivor to procreate until we reach cap.
        agents = vec![];
        while agents.len() < NUM_AGENTS {
            agents.push(survivors.choose(&mut rng).unwrap().procreate());
        }

        generation += 1;
    }
}
