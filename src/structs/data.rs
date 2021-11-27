use n_puzzle::*;

#[derive(Clone)]
pub struct Data {
    pub current: Location,
    pub end: Location,
}

impl Data {
    pub fn new(current: Location, end: Location) -> Self {
        Data { current, end }
    }
}
