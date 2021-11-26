use n_puzzle::*;
use std::fmt;

#[derive(Clone)]
pub struct Node<'a> {
    pub grid: Grid,
    pub cursor: Location,
    pub h: u16,
    pub g: u16,
    pub parent: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(grid: Grid, cursor: Location, h: u16, g: u16, parent: Option<&'a Node<'a>>) -> Self {
        Node {
            grid: grid,
            cursor: cursor,
            h: h,
            g: g,
            parent: parent,
        }
    }
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

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "     h:\t{}\n", self.h)?;
        write!(f, "     g:\t{}\n", self.g)?;
        write!(f, "cursor:\t{:?}\n", self.cursor)?;
        write!(f, "{}", self)
    }
}
