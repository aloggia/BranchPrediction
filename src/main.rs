use BranchPrediction::branch_predictor::prediction_buffer::PredictionBuffer;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};


#[allow(unused_variables, non_snake_case)]
fn main() {
    // Collect all command line args
    let args: Vec<String> = env::args().collect();
    //let args = vec!["../curl.trace.out", "0", "128"];
    let input_file = &args[1];
    //let input_file = "cholesky64.trace.out";
    // Might want to error check this to ensure num_bits is between 0 and 3
    //let num_bits: &usize = &args[2].parse::<usize>().unwrap();
    //let num_bits: &u8 = &0;
    // Error check to ensure buffer_size is a multiple of two
    let prediction_buffer_size: &usize = &args[2].parse::<usize>().unwrap();
    /*let mut aligned_buffer_size: usize;
    if prediction_buffer_size % 2 != 0 {
        aligned_buffer_size = prediction_buffer_size + 1;
    } else {
        aligned_buffer_size = *prediction_buffer_size;
    } */
    /*if prediction_buffer_size % 2 != 0 {
        // buffer size not a multiple of two, lets increment it by 1, to ensure it's a legal size
        prediction_buffer_size += 1;
    } */
    //let prediction_buffer_size = 4096;
    let mut prediction_vector: Vec<(usize, usize)> = Vec::new();
    //let mut prediction_buffer = PredictionBuffer::new(*prediction_buffer_size, *num_bits as u8);

    // Read the output trace into a vector
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
    let total_branches: f32 = prediction_vector.len() as f32;
    let immutable_predict_vec = Arc::new(Mutex::new(prediction_vector));
    let zero_bit_immut_predict_vec = immutable_predict_vec.clone();
    let one_bit_immut_predict_vec = immutable_predict_vec.clone();
    let two_bit_immut_predict_vec = immutable_predict_vec.clone();
    let three_bit_immut_predict_vec = immutable_predict_vec.clone();
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
    // create several sets of these vars + prediction buffers
    let mut prediction_buffer_0_bit = PredictionBuffer::new(*prediction_buffer_size, 0);
    let mut prediction_buffer_1_bit = PredictionBuffer::new(*prediction_buffer_size, 1);
    let mut prediction_buffer_2_bit = PredictionBuffer::new(*prediction_buffer_size, 2);
    let mut prediction_buffer_3_bit = PredictionBuffer::new(*prediction_buffer_size, 3);
    let mut correct_predictions_0_bit: f32 = 0.0;
    let mut correct_predictions_1_bit: f32 = 0.0;
    let mut correct_predictions_2_bit: f32 = 0.0;
    let mut correct_predictions_3_bit: f32 = 0.0;
    let mut wrong_predictions_0_bit: f32 = 0.0;
    let mut wrong_predictions_1_bit: f32 = 0.0;
    let mut wrong_predictions_2_bit: f32 = 0.0;
    let mut wrong_predictions_3_bit: f32 = 0.0;

    let zero_bit_predict = thread::spawn( move|| {
        for branch in zero_bit_immut_predict_vec.lock().unwrap().iter() {
            let pred_result = prediction_buffer_0_bit.make_prediction(branch.0);
            if pred_result == branch.1 {
                correct_predictions_0_bit += 1.0
            } else {
                wrong_predictions_0_bit += 1.0;
            }
            if branch.1 == 0 {
                prediction_buffer_0_bit.update_prediction_bit(branch.0, false);
            } else {
                prediction_buffer_0_bit.update_prediction_bit(branch.0, true);
            }
        }
        println!("0 bit predictor:\n Percent Correct predictions: {:.3}\n Percent Incorrect predictions: {:.3}",
                 (correct_predictions_0_bit / total_branches) * 100.00,
                 (wrong_predictions_0_bit / total_branches) * 100.00);
    });

    let one_bit_predict = thread::spawn(move || {
        for branch in one_bit_immut_predict_vec.lock().unwrap().iter() {
            let pred_result = prediction_buffer_1_bit.make_prediction(branch.0);
            if pred_result == branch.1 {
                correct_predictions_1_bit += 1.0
            } else {
                wrong_predictions_1_bit += 1.0;
            }
            if branch.1 == 0 {
                prediction_buffer_1_bit.update_prediction_bit(branch.0, false);
            } else {
                prediction_buffer_1_bit.update_prediction_bit(branch.0, true);
            }
        }
        println!("1 bit predictor:\n Percent Correct predictions: {:.3}\n Percent Incorrect predictions: {:.3}",
                 (correct_predictions_1_bit / total_branches) * 100.00,
                 (wrong_predictions_1_bit / total_branches) * 100.00);
    });
    let two_bit_predict = thread::spawn(move || {
        for branch in two_bit_immut_predict_vec.lock().unwrap().iter() {
            let pred_result = prediction_buffer_2_bit.make_prediction(branch.0);
            if pred_result == branch.1 {
                correct_predictions_2_bit += 1.0
            } else {
                wrong_predictions_2_bit += 1.0;
            }
            if branch.1 == 0 {
                prediction_buffer_2_bit.update_prediction_bit(branch.0, false);
            } else {
                prediction_buffer_2_bit.update_prediction_bit(branch.0, true);
            }
        }
        println!("2 bit predictor:\n Percent Correct predictions: {:.3}\n Percent Incorrect predictions: {:.3}",
                 (correct_predictions_2_bit / total_branches) * 100.00,
                 (wrong_predictions_2_bit / total_branches) * 100.00);
    });
    let three_bit_predict = thread::spawn(move || {
        for branch in three_bit_immut_predict_vec.lock().unwrap().iter() {
            let pred_result = prediction_buffer_3_bit.make_prediction(branch.0);
            if pred_result == branch.1 {
                correct_predictions_3_bit += 1.0
            } else {
                wrong_predictions_3_bit += 1.0;
            }
            if branch.1 == 0 {
                prediction_buffer_3_bit.update_prediction_bit(branch.0, false);
            } else {
                prediction_buffer_3_bit.update_prediction_bit(branch.0, true);
            }
        }
        println!("3 bit predictor:\n Percent Correct predictions: {:.3}\n Percent Incorrect predictions: {:.3}",
                 (correct_predictions_3_bit / total_branches) * 100.00,
                 (wrong_predictions_3_bit / total_branches) * 100.00);
    });
    zero_bit_predict.join().unwrap();
    one_bit_predict.join().unwrap();
    two_bit_predict.join().unwrap();
    three_bit_predict.join().unwrap();



    // Outputting time!!
    /*
    The plan rn is to put all the results into a file or files, open those files in python, then
    graph the outputs
     */




}