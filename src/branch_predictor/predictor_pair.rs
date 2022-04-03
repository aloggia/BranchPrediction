pub struct BranchPair {
    address: usize,
    prediction: u8,
}

impl BranchPair {
    pub fn new(address: usize) -> BranchPair {
        BranchPair {
            address,
            prediction: 0,
        }
    }
    pub(crate) fn get_address(&self) -> usize {
        self.address
    }
    pub(crate) fn get_prediction(&self) -> u8 {
        self.prediction
    }
    pub(crate) fn set_address(&mut self, address: usize) {
        self.address = address;
    }
    pub(crate) fn set_prediction(&mut self, prediction: u8) {
        self.prediction = prediction;
    }
}