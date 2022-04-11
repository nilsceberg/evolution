use rand::Rng;
use uuid::Uuid;

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

    fn simulate(&mut self, time: f32) {
        self.brain.input(brain::Input::Constant, 1.0);
        self.brain.input(brain::Input::Oscillator, time * std::f32::consts::TAU);
        self.brain.input(brain::Input::X, self.position.0);
        self.brain.input(brain::Input::Y, self.position.1);
        self.brain.simulate();
        self.position.0 += self.brain.output(brain::Output::SpeedX);
        self.position.1 += self.brain.output(brain::Output::SpeedY);
        self.position = keep_inside_radius(self.position, WORLD_RADIUS);
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


    let mut agents : Vec<Agent> = vec![];
    for _ in 0..100 {
        agents.push(Agent::new());
    }

    publisher.send(viewer::spawn(&agents)).unwrap();
    let mut time: f32 = 0.0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        time += 0.050;

        publisher.send(viewer::frame(&agents)).unwrap(); 

        for agent in &mut agents {
            agent.simulate(time);
        }
    }
}
