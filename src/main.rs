use rand::Rng;
use uuid::Uuid;

mod dot;
mod viewer;
mod brain;

use brain::Brain;

pub struct Agent {
    uuid: Uuid,
    position: (f32, f32),
}

impl Agent {
    fn new() -> Agent {
        let mut rng = rand::thread_rng();
        Agent {
            uuid: Uuid::new_v4(),
            position: (
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
                (rng.gen::<f32>() * 2.0 - 1.0) * 300.0,
            ),
        }
    }

    fn simulate(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = (rng.gen::<f32>() * 2.0 - 1.0) * 10.0;
        let dy = (rng.gen::<f32>() * 2.0 - 1.0) * 10.0;
        self.position.0 += dx;
        self.position.1 += dy;
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

    let mut brain = Brain::new();
    brain.weights = Brain::random_weights();

    //brain.draw_graph(&mut std::io::stdout());

    let mut agents : Vec<Agent> = vec![];
    for _ in 0..100 {
        agents.push(Agent::new());
    }

    publisher.send(viewer::spawn(&agents)).unwrap();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(50));
        publisher.send(viewer::frame(&agents)).unwrap(); 

        for agent in &mut agents {
            agent.simulate();
        }
    }
}
