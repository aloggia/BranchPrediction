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
    fn get_address(&self) -> usize {
        self.address
    }
    fn get_prediction(&self) -> u8 {
        self.prediction
    }
    fn set_addrress(&mut self, address: usize) {
        self.address = address;
    }
    fn set_prediction(&mut self, prediction: u8) {
        self.prediction = prediction;
    }
}