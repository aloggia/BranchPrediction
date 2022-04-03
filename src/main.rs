
use std::env;
use std::fs;
use BranchPrediction::branch_predictor::predictor_pair::BranchPair;
use BranchPrediction::branch_predictor::prediction_buffer::PredictionBuffer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let num_bits = &args[2];
    let prediction_buffer_size = &args[3];

    let new_branch_pair = BranchPair::new(5000);


}
