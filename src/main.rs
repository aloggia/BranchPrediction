use BranchPrediction::branch_predictor::prediction_buffer::PredictionBuffer;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[allow(unused_variables, non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    //let input_file = "cholesky64.trace.out";
    let input_file_string = "./".to_owned() + input_file;
    let num_bits: &u8 = &args[2].parse::<u8>().unwrap();
    //let prediction_buffer_size: &usize = &args[3].parse::<usize>().unwrap();
    let prediction_buffer_size = 4096;
    let mut prediction_vector: Vec<(usize, usize)> = Vec::new();

    let prediction_buffer = PredictionBuffer::new(prediction_buffer_size, *num_bits);
    // Read the output trace into a vector
    if let Ok(lines) = read_lines(input_file_string) {
        // loop through each line of the file
        for line in lines {
            // If the line is valid, then do something???
            if let Ok(ref prediction) = line {
                // What should we do here?
                /*
                Split string at the space,
                convert both values to ints
                Put both int's into a tuple
                insert that tuple into the vector
                 */
                let unwrapped_line = prediction;
                let string_iter = unwrapped_line.split_whitespace();
                let string_vec: Vec<&str> = string_iter.collect();
                // Need to parse from hex string -> int
                let addr_int = usize::from_str_radix(string_vec[0], 16);
                let address = addr_int.unwrap();
                let result: usize = string_vec[1].parse::<usize>().unwrap();
                prediction_vector.push((address as usize, result));
            }
        }
    }

}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
