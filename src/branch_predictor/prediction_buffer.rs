use crate::branch_predictor::predictor_pair::BranchPair;

pub struct PredictionBuffer {
    buffer_size: usize,
    buffer: Vec<BranchPair>
}

impl PredictionBuffer {
    pub fn new(buffer_size: usize) -> PredictionBuffer {
        PredictionBuffer {
            buffer_size,
            buffer: Vec::with_capacity(buffer_size),
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
    pub fn set_branch_prediction(&mut self, index: usize, prediction: u8) {
        self.buffer[index].set_prediction(prediction);
    }

    pub fn set_branch_at_index(&mut self, index: usize, in_branch: BranchPair) {
        self.buffer[index] = in_branch;
    }
}
