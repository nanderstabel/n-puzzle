use std::collections::HashMap;
use std::fmt;

pub type Row = Vec<u8>;
pub type Location = (usize, usize);
pub type Grid = Vec<Row>;

pub trait Swap {
    fn swap(&mut self, i: Location, j: Location) -> Self;
}

impl Swap for Grid {
    fn swap(&mut self, i: Location, j: Location) -> Self {
        let tmp = self[j.0 as usize][j.1 as usize];
        self[j.0 as usize][j.1 as usize] = self[i.0 as usize][i.1 as usize];
        self[i.0 as usize][i.1 as usize] = tmp;
        self.to_vec()
    }
}

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

#[derive(Debug)]
pub struct Puzzle<'a> {
    pub data: HashMap<u8, Data>,
    pub closed: HashMap<Grid, u16>,
    pub open: Vec<&'a mut Node<'a>>,
}

impl<'a> Puzzle<'a> {
    pub fn new() -> Self {
        Puzzle {
            data: HashMap::new(),
            closed: HashMap::new(),
            open: Vec::new(),
        }
    }
}
