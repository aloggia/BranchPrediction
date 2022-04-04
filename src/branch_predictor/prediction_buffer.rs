use crate::branch_predictor::predictor_pair::BranchPair;

pub struct PredictionBuffer {
    buffer_size: usize,
    buffer: Vec<BranchPair>,
    prediction_bit_size: u8,
}

#[allow(unused_comparisons)]
impl PredictionBuffer {
    pub fn new(buffer_size: usize, prediction_bit_size: u8) -> PredictionBuffer {
        let mut pred_vec: Vec<BranchPair> = Vec::with_capacity(buffer_size);
        for _ in 0..buffer_size {
            pred_vec.push(BranchPair::new(0, prediction_bit_size));
        }
        PredictionBuffer {
            buffer_size,
            prediction_bit_size,
            buffer: pred_vec,
            // Might need to fill this vec
        }
    }

    pub fn get_branch_at_index(&self, index: usize) -> &BranchPair {
        &self.buffer[index]
    }
    pub fn get_branch_address(&self, index: usize) -> usize {
        self.buffer[index].get_address()
    }
    pub fn get_branch_prediction(&self, index: usize) -> u8 {
        self.buffer[index].get_prediction()
    }
    pub fn set_branch_address(&mut self, index: usize, address: usize) {
        self.buffer[index].set_address(address)
    }
    pub(crate) fn set_branch_prediction(&mut self, index: usize, prediction: bool) {
        self.buffer[index].set_prediction(prediction);
    }

    pub fn set_branch_at_index(&mut self, index: usize, in_branch: BranchPair) {
        self.buffer[index] = in_branch;
    }

    pub fn make_prediction(&mut self, addr: usize) -> bool {
        // check if this branch instr is in our prediction buffer
        if self.get_branch_address(addr & (self.buffer_size-1)) != addr {
            // if this instr is not in the buffer then move it in
            self.set_branch_at_index(addr & (self.buffer_size - 1),
                                     BranchPair::new(addr, self.prediction_bit_size));
        }
        // Do variable things depending on the prediction bit size
        match self.prediction_bit_size {
            0 => {
                // 0 bit predictor
                // Always predict branch is taken
                true
            }
            1 => {
                // 1 bit predictor
                // If the prediction bit is 0, then predict not taken
                if self.get_branch_prediction(addr & (self.buffer_size - 1)) == 0 {
                    false
                } else {
                    // otherwise predict taken
                    true
                }
            }
            2 => {
                // 2 bit predictor
                // If prediction is 0 or 1, then predict not taken
                // else predict taken
                if self.get_branch_prediction(addr & (self.buffer_size - 1)) == 0 ||
                    self.get_branch_prediction(addr & (self.buffer_size - 1)) == 1 {
                    false
                } else {
                    true
                }
            }
            3 => {
                // 3 bit predictor
                // if prediction bit is between 0 & 3 inclusive predict not taken
                // else predict taken
                if self.get_branch_prediction(addr & (self.buffer_size - 1)) >= 0 &&
                    self.get_branch_prediction(addr & (self.buffer_size - 1)) <= 3 {
                    false
                } else {
                    true
                }
            }
            _ => {
                // Catch all safety base case
                false
            }
        }
    }
    // If the branch was taken, then increase the prediction bit by 1,
    // if the branch wasn't taken decrease it by one
    pub fn update_prediction_bit(&mut self, address: usize, branch_taken: bool) {
        if branch_taken {
            self.set_branch_prediction(address & (self.buffer_size - 1), true);
        }
        else {
            self.set_branch_prediction(address & (self. buffer_size - 1), false)
        }
    }
}
