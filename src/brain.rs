use std::ops::Range;
use num_derive::{FromPrimitive, ToPrimitive};

pub const NUM_NEURONS: usize = 32;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Input {
    X,
    Y,
    Number
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Output {
    SpeedX = Input::Number as isize,
    SpeedY,
    Number
}

pub const NUM_FIXED: usize = Output::Number as usize;
pub const NUM_INPUTS: usize = Input::Number as usize;
pub const NUM_OUTPUTS: usize = Output::Number as usize;
pub const INPUT_INDICES: Range<usize> = 0..(Input::Number as usize);
pub const OUTPUT_INDICES: Range<usize> = (Input::Number as usize)..(Output::Number as usize);
pub const FIXED_INDICES: Range<usize> = 0..(Output::Number as usize);

pub struct Brain {
    pub weights: [[f32; NUM_NEURONS]; NUM_NEURONS],
    activation: [f32; NUM_NEURONS],
}

fn activation_function(index: usize, value: f32) -> f32 {
    if index == Output::SpeedX as usize || index == Output::SpeedY as usize {
        value
    }
    else {
        // Sigmoidal function:
        1.0 / (1.0 + (-4.0 * value).exp())
    }
}

impl Brain {
    pub fn new() -> Brain {
        Brain {
            weights: [[0.0; NUM_NEURONS]; NUM_NEURONS],
            activation: [0.0; NUM_NEURONS],
        }
    }

    pub fn input(&mut self, input: Input, value: f32) {
        self.activation[input as usize] = value;
    }

    pub fn output(&self, output: Output) -> f32 {
        self.activation[output as usize]
    }

    pub fn simulate(&mut self) {
        let mut new_activation = [0.0; NUM_NEURONS];
        for j in 0..NUM_NEURONS {
            for i in 0..NUM_NEURONS {
                if i == j { continue; }
                new_activation[j] += self.activation[i] * self.weights[i][j];
            }
            new_activation[j] = activation_function(j, new_activation[j] + self.weights[j][j]);
        }

        self.activation = new_activation;
    }
}
