use std::fmt;

pub type Row = Vec<u8>;
pub type Grid = Vec<Row>;
pub type Location = (usize, usize);

#[derive(Clone)]
pub struct Data {
    pub h: u16,
    pub current: Location,
    pub end: Location,
}

impl Data {
    pub fn new(h: u16, current: Location, end: Location) -> Self {
        Data {
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
            "\th:\t\t{}\n\tcurrent:\t{:?}\n\tend:\t\t{:?}\n\n",
            self.h, self.current, self.end
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub grid: Grid,
    pub cursor: Location,
    pub h: u16,
    pub parent: Option<&'a Node<'a>>,
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for u in row {
                match u {
                    0 => write!(f, "    ")?,
                    _ => write!(f, "{:4}", u)?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Puzzle {
    buf: String,
}
