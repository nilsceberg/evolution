use std::ops::Range;
use num_derive::{FromPrimitive, ToPrimitive};
use rand::Rng;

mod dot;
mod viewer;

const NUM_NEURONS: usize = 32;

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Inputs {
    X,
    Y,
    Number
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
enum Outputs {
    Left = Inputs::Number as isize,
    Right,
    Back,
    Forward,
    Number
}

const NUM_FIXED: usize = Outputs::Number as usize;
const INPUT_INDICES: Range<usize> = 0..(Inputs::Number as usize);
const OUTPUT_INDICES: Range<usize> = (Inputs::Number as usize)..(Outputs::Number as usize);
const FIXED_INDICES: Range<usize> = 0..(Outputs::Number as usize);

struct Brain {
    weights: [[f32; NUM_NEURONS]; NUM_NEURONS],
    excitation: [f32; NUM_NEURONS],
    activation: [f32; NUM_NEURONS],
}

impl Brain {
    fn new() -> Brain {
        Brain {
            weights: [[0.0; NUM_NEURONS]; NUM_NEURONS],
            excitation: [0.0; NUM_NEURONS],
            activation: [0.0; NUM_NEURONS],
        }
    }

    fn random_weights() -> [[f32; NUM_NEURONS]; NUM_NEURONS] {
        const PROB_LIVE: f32 = 0.1;
        const WEIGHT_RANGE: std::ops::Range<f32> = -1.0..1.0;

        let mut rng = rand::thread_rng();

        let mut weights = [[0.0; NUM_NEURONS]; NUM_NEURONS];
        for i in 0..NUM_NEURONS {
            for j in 0..NUM_NEURONS {
                if rng.gen::<f32>() < PROB_LIVE {
                    weights[i][j] = rng.gen_range(WEIGHT_RANGE);
                }
            }
        }

        return weights;
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
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        publisher.send(viewer::Message::Clear).unwrap(); 
    }
}
