use n_puzzle::*;
use std::fmt;

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

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\ncurrent:\t{:?}\nend:\t\t{:?}\n",
            self.current, self.end
        )?;
        Ok(())
    }
}
