use n_puzzle::*;
use std::fmt;

#[derive(Clone)]
pub struct Data {
    pub h: u16,
    pub current: Location,
    pub end: Location,
}

impl Data {
    pub fn new(h: u16, current: Location, end: Location) -> Self {
        Data { h, current, end }
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "h:\t\t{}\ncurrent:\t{:?}\nend:\t\t{:?}",
            self.h, self.current, self.end
        )?;
        Ok(())
    }
}
