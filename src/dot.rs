use std::io::Write;

use super::brain::{NUM_NEURONS, FIXED_INDICES, INPUT_INDICES, OUTPUT_INDICES, Input, Output};

impl super::Brain {
    pub fn draw_graph(&self, file: &mut impl Write) {
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
            let input: Input = num::FromPrimitive::from_usize(i).unwrap();
            writeln!(file, "_{} [label=\"IN:{:?}\", bgcolor=\"blue\"]", i, input).unwrap();
        }
        writeln!(file, "}}").unwrap();

        for i in relevant_neurons {
            writeln!(file, "_{} [label=\"{}\", bgcolor=\"green\"];", i, i).unwrap();
        }

        writeln!(file, "subgraph cluster_outputs {{").unwrap();
        writeln!(file, "peripheries=0;").unwrap();
        for i in OUTPUT_INDICES {
            let output: Output = num::FromPrimitive::from_usize(i).unwrap();
            writeln!(file, "_{} [label=\"OUT:{:?}\"]", i, output).unwrap();
        }
        writeln!(file, "}}").unwrap();

        writeln!(file, "}}").unwrap();
    }
}