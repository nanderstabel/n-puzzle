use n_puzzle::*;
use std::fmt;

#[derive(Clone)]
pub struct Data {
    pub value: u8,
    pub h: u16,
    pub current: Location,
    pub end: Location,
}

impl Data {
    pub fn new(value: u8, h: u16, current: Location, end: Location) -> Self {
        Data {
            value: value,
            h: h,
            current: current,
            end: end,
        }
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "value:\t\t{}\nh:\t\t{}\ncurrent:\t{:?}\nend:\t\t{:?}",
            self.value, self.h, self.current, self.end
        )?;
        Ok(())
    }
}
