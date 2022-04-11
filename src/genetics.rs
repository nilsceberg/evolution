//! Conceptual separation of genetics from brain weights, even
//! though there is a one-to-one mapping.

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::brain::{NUM_NEURONS, Brain};

pub const NUM_CODONS: usize = NUM_NEURONS * NUM_NEURONS;

pub type Genome = [f32; NUM_CODONS];

pub fn randomize() -> Genome {
    const PROB_LIVE: f32 = 0.1;
    const WEIGHT_RANGE: std::ops::Range<f32> = -1.0..1.0;

    let mut rng = rand::thread_rng();

    let mut codons :Genome = [0.0; NUM_CODONS];
    for i in 0..NUM_CODONS {
        if rng.gen::<f32>() < PROB_LIVE {
            codons[i] = rng.gen_range(WEIGHT_RANGE);
        }
    }

    return codons;
}

pub fn create_brain(genome: &Genome) -> Brain {
    let mut brain = Brain::new();
    for i in 0..NUM_NEURONS {
        for j in 0..NUM_NEURONS {
            brain.weights[i][j] = genome[i * NUM_NEURONS + j];
        }
    }
    brain
}

