use BranchPrediction::branch_predictor::predictor_pair::BranchPair;
use BranchPrediction::branch_predictor::prediction_buffer::PredictionBuffer;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let input_file_string = "./".to_owned() + input_file;
    let num_bits: &u8 = &args[2].parse::<u8>().unwrap();
    let prediction_buffer_size: &usize = &args[3].parse::<usize>().unwrap();
    let mut prediction_vector: Vec<prediction> = Vec::new();

    let mut prediction_buffer = PredictionBuffer::new(*prediction_buffer_size);

    if let Ok(lines) = read_lines(input_file_string) {
        for line in lines {
            if let Ok(prediction) = line {
                prediction_vector.push(prediction);
            }
        }
    }


}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
