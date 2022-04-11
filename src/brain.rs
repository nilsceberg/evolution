use std::ops::Range;
use num_derive::{FromPrimitive, ToPrimitive};

pub const NUM_NEURONS: usize = 32;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Inputs {
    X,
    Y,
    Number
}

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Outputs {
    Left = Inputs::Number as isize,
    Right,
    Back,
    Forward,
    Number
}

pub const NUM_FIXED: usize = Outputs::Number as usize;
pub const INPUT_INDICES: Range<usize> = 0..(Inputs::Number as usize);
pub const OUTPUT_INDICES: Range<usize> = (Inputs::Number as usize)..(Outputs::Number as usize);
pub const FIXED_INDICES: Range<usize> = 0..(Outputs::Number as usize);

pub struct Brain {
    pub weights: [[f32; NUM_NEURONS]; NUM_NEURONS],
    excitation: [f32; NUM_NEURONS],
    activation: [f32; NUM_NEURONS],
}

impl Brain {
    pub fn new() -> Brain {
        Brain {
            weights: [[0.0; NUM_NEURONS]; NUM_NEURONS],
            excitation: [0.0; NUM_NEURONS],
            activation: [0.0; NUM_NEURONS],
        }
    }
}
