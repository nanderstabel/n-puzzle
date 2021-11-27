use super::data::*;
use super::node::*;
use crate::input::*;
use n_puzzle::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    rc::Rc,
};

fn manhattan(a: (usize, usize), b: (usize, usize)) -> u16 {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 } + if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
        as u16
}

#[derive(Debug)]
pub struct Puzzle {
    pub data: HashMap<u8, Data>,
    pub closed: HashSet<Grid>,
    pub open: VecDeque<Node>,
}

impl Puzzle {
    pub fn new() -> Self {
        Puzzle {
            data: HashMap::new(),
            closed: HashSet::new(),
            open: VecDeque::new(),
        }
    }

    fn get_end_state(size: u8) -> Grid {
        let mut grid: Grid = vec![vec![0; size.into()]; size.into()];
        let mut set: Vec<u8> = (1..(size.pow(2))).rev().collect();
        let (mut top, mut bottom, mut left, mut right): (isize, isize, isize, isize) =
            (0, (size - 1).into(), 0, (size - 1).into());

        macro_rules! fill {
            ($range:expr, $level:ident, $direction:literal, $increment:literal) => {
                for index in $range {
                    match $direction {
                        "hor" => grid[$level as usize][index as usize] = set.pop().unwrap(),
                        _ => grid[index as usize][$level as usize] = set.pop().unwrap(),
                    }
                }
                $level += $increment;
            };
        }

        while !set.is_empty() {
            fill!(left..=right, top, "hor", 1);
            fill!(top..=bottom, right, "ver", -1);
            fill!((left..=right).rev(), bottom, "hor", -1);
            fill!((top..=bottom).rev(), left, "ver", 1);
        }
        grid
    }

    fn get_data(u: u8, y: usize, x: usize, size: u8) -> Data {
        for (j, row) in Self::get_end_state(size).iter().enumerate() {
            for (i, v) in row.iter().enumerate() {
                if u == *v {
                    return Data::new(
                        if u != 0 { manhattan((y, x), (j, i)) } else { 0 },
                        (y, x),
                        (j, i),
                    );
                }
            }
        }
        Data::new(0, (y, x), (y, x))
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        let (size, grid) = get_grid()?;
        let mut start = Node::new(grid, (0, 0), 0, 0, None);
        for (y, row) in start.grid.iter().enumerate() {
            for (x, u) in row.iter().enumerate() {
                self.data.insert(*u, Self::get_data(*u, y, x, size));
            }
        }
        start.h = self.data.values().map(|d| d.h).sum();
        start.cursor = (*self.data.get_mut(&0).unwrap()).current;
        self.open.push_front(start);
        Ok(())
    }

    fn add_child(&mut self, parent: &Rc<Node>, cursor: (usize, usize), new_cursor: (usize, usize)) {
        let new_grid: Grid = parent
            .grid
            .iter()
            .cloned()
            .collect::<Grid>()
            .swap(cursor, new_cursor);
        if self.closed.contains(&new_grid) == false {
            let target = new_grid[cursor.0][cursor.1];
            let meta = &(*self.data.get_mut(&target).unwrap());
            let new_h = manhattan(cursor, meta.end) as i16 - manhattan(new_cursor, meta.end) as i16;
            self.open.push_back(Node::new(
                new_grid,
                new_cursor,
                (parent.h as i16 + new_h) as u16,
                parent.g + 1,
                Some(Rc::clone(&parent)),
            ));
        }
    }

    fn add_children(&mut self) {
        let parent = Rc::new(self.open.pop_front().unwrap());
        let cursor = parent.cursor;
        self.closed.insert(parent.grid.iter().cloned().collect());
        if parent.cursor.0 != 0 {
            self.add_child(&parent, cursor, (cursor.0 - 1, cursor.1));
        }
        if parent.cursor.0 + 1 < parent.grid.len() {
            self.add_child(&parent, cursor, (cursor.0 + 1, cursor.1));
        }
        if parent.cursor.1 != 0 {
            self.add_child(&parent, cursor, (cursor.0, cursor.1 - 1));
        }
        if parent.cursor.1 + 1 < parent.grid.len() {
            self.add_child(&parent, cursor, (cursor.0, cursor.1 + 1));
        }
    }

    pub fn solve(&mut self) {
        while self.open[0].h != 0 {
            self.add_children();
            self.open
                .make_contiguous()
                // .sort_by(|a, b| (a.h + a.g).cmp(&(b.h + b.g)));
                .sort_by(|a, b| (a.h).cmp(&(b.h)));
        }
    }
}
