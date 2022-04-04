use BranchPrediction::branch_predictor::prediction_buffer::PredictionBuffer;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::convert::TryInto;

#[allow(unused_variables, non_snake_case)]
fn main() {
    // Collect all command line args
    let args: Vec<String> = env::args().collect();
    //let args = vec!["cholesky64.trace.out", "1", "4096"];
    let input_file = &args[1];
    //let input_file = "cholesky64.trace.out";
    // Might want to error check this to ensure num_bits is between 0 and 3
    let num_bits: &usize = &args[2].parse::<usize>().unwrap();
    //let num_bits: &u8 = &0;
    // Error check to ensure buffer_size is a multiple of two
    let mut prediction_buffer_size: &usize = &args[3].parse::<usize>().unwrap();
    /*if prediction_buffer_size % 2 != 0 {
        // buffer size not a multiple of two, lets increment it by 1, to ensure it's a legal size
        *prediction_buffer_size += 1;
    }*/
    //let prediction_buffer_size = 4096;
    let mut prediction_vector: Vec<(usize, usize)> = Vec::new();
    let mut prediction_buffer = PredictionBuffer::new(*prediction_buffer_size, (*num_bits).try_into().unwrap());

    // Read the output trace into a vector
    /*if let Ok(lines) = read_lines(input_file_string) {
        println!("enters read_lines");
        // loop through each line of the file
        for line in lines {
            println!("Looping");
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
    }*/
    let in_file = File::open(input_file).unwrap();
    let reader = BufReader::new(in_file);

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let string_iter = unwrapped_line.split_whitespace();
        let string_vec: Vec<&str> = string_iter.collect();
        let addr_int = usize::from_str_radix(string_vec[0], 16);
        let address = addr_int.unwrap();
        let result: usize = string_vec[1].parse::<usize>().unwrap();
        prediction_vector.push((address as usize, result));
    }
    // Prog structure:
    /*
    loop through the prediction vector,
    for each prediction in prediction vector, make a prediction
        look at the actual branch result
        if prediction matched:
            incr correct counter
        else:
            incr incorrect counter
        incr total branches counter
     */
    let mut total_branches = 0;
    let mut correct_predictions = 0;
    let mut wrong_predictions = 0;
    for branch in prediction_vector.iter() {
        let pred_result = prediction_buffer.make_prediction(branch.0);
        if pred_result && branch.1 == 1{
            correct_predictions += 1;
        } else {
            wrong_predictions += 1;
        }
        total_branches += 1;

        if branch.1 == 0 {
            // branch not taken
            prediction_buffer.update_prediction_bit(branch.0, false)
        } else if branch.1 == 1 {
            // branch taken
            prediction_buffer.update_prediction_bit(branch.0, true)
        }

    }
    println!("Total branches: {}\n Correct predictions: {}\n Incorrect predictions: {}",
             total_branches, correct_predictions, wrong_predictions);

}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
