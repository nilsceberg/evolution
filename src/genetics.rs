//! Conceptual separation of genetics from brain weights, even
//! though there is a one-to-one mapping.

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::brain::{NUM_NEURONS, Brain};

pub const NUM_CODONS: usize = NUM_NEURONS * NUM_NEURONS;

pub type Genome = [f32; NUM_CODONS];

pub fn randomize() -> Genome {
    let mut genome :Genome = [0.0; NUM_CODONS];
    mutate(&mut genome, 0.1, 1.0);
    genome
}

pub fn mutate(genome: &mut Genome, rate: f32, strength: f32) {
    let mut rng = rand::thread_rng();
    for i in 0..NUM_CODONS {
        if rng.gen::<f32>() < rate {
            genome[i] += rng.sample::<f32, _>(rand_distr::StandardNormal) * strength;
        }
    }
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
