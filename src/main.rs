use std::io::Write;
use std::ops::Range;
use num_derive::{FromPrimitive, ToPrimitive};
use rand::Rng;

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

    fn draw_graph(&self, file: &mut impl Write) {
        writeln!(file, "digraph {{").unwrap();
        writeln!(file, "splines=false;").unwrap();
        writeln!(file, "rankdir=\"LR\";").unwrap();

        let mut relevant_neurons = std::collections::HashSet::<usize>::new();
        for i in 0..NUM_NEURONS {
            for j in 0..NUM_NEURONS {
                const EPSILON: f32 = 0.0001;
                if INPUT_INDICES.contains(&j) || OUTPUT_INDICES.contains(&i) || self.weights[i][j].abs() < EPSILON {
                    // Irrelevant connection.
                    continue;
                }

                // May form an island, but are at least not individually isolated.
                if !FIXED_INDICES.contains(&i) { relevant_neurons.insert(i); }
                if !FIXED_INDICES.contains(&j) { relevant_neurons.insert(j); }
                writeln!(file, "_{} -> _{} [penwidth={}];", i, j, self.weights[i][j].abs() * 10.0).unwrap();
            }
        }

        writeln!(file, "subgraph cluster_inputs {{").unwrap();
        writeln!(file, "peripheries=0;").unwrap();
        for i in INPUT_INDICES {
            let input: Inputs = num::FromPrimitive::from_usize(i).unwrap();
            writeln!(file, "_{} [label=\"IN:{:?}\", bgcolor=\"blue\"]", i, input).unwrap();
        }
        writeln!(file, "}}").unwrap();

        for i in relevant_neurons {
            writeln!(file, "_{} [label=\"{}\", bgcolor=\"green\"];", i, i).unwrap();
        }

        writeln!(file, "subgraph cluster_outputs {{").unwrap();
        writeln!(file, "peripheries=0;").unwrap();
        for i in OUTPUT_INDICES {
            let output: Outputs = num::FromPrimitive::from_usize(i).unwrap();
            writeln!(file, "_{} [label=\"OUT:{:?}\"]", i, output).unwrap();
        }
        writeln!(file, "}}").unwrap();

        writeln!(file, "}}").unwrap();
    }
}

fn main() {
    let mut brain = Brain::new();
    brain.weights = Brain::random_weights();

    brain.draw_graph(&mut std::io::stdout());
}
