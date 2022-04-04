#[derive(Copy, Clone)]
pub struct BranchPair {
    address: usize,
    prediction: u8,
    prediction_bit_size: u8,
}
#[allow(unused_comparisons)]
impl BranchPair {
    pub fn new(address: usize, prediction_bit_size: u8) -> BranchPair {
        BranchPair {
            address,
            prediction: 0,
            prediction_bit_size
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
    pub(crate) fn set_prediction(&mut self, prediction: bool) {
        /*
        Edge case where predictor is at the max size, we don't want to update it
         */
        match self.prediction_bit_size {
            0 => {}
            1 => {
                /*
                In a 1 bit predictor, only update the counter if the bit needs to flip
                 */
                if self.prediction == 0 && prediction {
                    self.prediction += 1;
                } else if self.prediction == 1 && !prediction {
                    self.prediction -= 1;
                }
            }
            2 => {
                /*
                Only update the prediction if a branch is taken and can be increased
                or if a branch wasn't taken and can be decreased
                 */
                if (self.prediction == 0 || self.prediction == 1 || self.prediction == 2) &&
                    prediction {
                    self.prediction += 1;
                } else if (self.prediction == 1 || self.prediction == 2 || self.prediction == 3) &&
                    !prediction {
                    self.prediction -= 1;
                }
            }
            3 => {
                /*
                Only increase the prediction if its in range 0-6 and the branch was taken
                Only decrease the prediction if it's in range 1-7 and branch wasn't taken
                 */
                if (self.prediction >= 0 && self.prediction < 7) && prediction {
                    self.prediction += 1;
                } else if (self.prediction >= 1 && self.prediction <= 7) && prediction {
                    self.prediction -= 1;
                }
            }
            _ => {}
        }
    }
}