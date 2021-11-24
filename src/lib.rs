use std::fmt;

pub type Row = Vec<u8>;
pub type Grid = Vec<Row>;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub grid: Grid,
    pub parent: Option<&'a Node<'a>>
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            write!(f, "{:2?}\n", row)?;
        }
        Ok(())
    }
}


#[derive(Debug, Default)]
pub struct Puzzle {
    buf: String
}
