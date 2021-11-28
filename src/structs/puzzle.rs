use super::data::*;
use super::node::*;
use crate::input::*;
use n_puzzle::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    rc::Rc,
};

fn manhattan((cy, cx): (usize, usize), (ey, ex): (usize, usize)) -> u16 {
    (if cy > ey { cy - ey } else { ey - cy } + if cx > ex { cx - ex } else { ex - cx }) as u16
}

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

    fn get_end_state(&mut self, size: u8) {
        let mut set: Vec<u8> = (1..(size.pow(2))).rev().collect();
        let (mut top, mut bottom, mut left, mut right): (isize, isize, isize, isize) =
            (0, (size - 1).into(), 0, (size - 1).into());

        macro_rules! fill {
            ($range:expr, $level:ident, $direction:literal, $increment:literal) => {
                for index in $range {
                    self.data.insert(
                        set.pop().unwrap(),
                        Data::new(
                            (0, 0),
                            match $direction {
                                "hor" => ($level as usize, index as usize),
                                _ => (index as usize, $level as usize),
                            },
                        ),
                    );
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
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        let (size, grid) = get_grid()?;
        let mut cursor = (0, 0);
        self.get_end_state(size);
        for (y, row) in grid.iter().enumerate() {
            for (x, u) in row.iter().enumerate() {
                if *u != 0 {
                    self.data.get_mut(u).unwrap().current = (y, x);
                } else {
                    cursor = (y, x);
                }
            }
        }
        self.open.push_front(Node::new(
            grid,
            cursor,
            self.data
                .values()
                .map(|d| manhattan(d.current, d.end))
                .sum(),
            0,
            None,
        ));
        Ok(())
    }

    fn add_child(&mut self, parent: &Rc<Node>, new_cursor: (usize, usize)) {
        let new_grid = parent.grid.clone().swap(parent.cursor, new_cursor);

        if self.closed.contains(&new_grid) == false {
            let target = new_grid[parent.cursor.0][parent.cursor.1];
            let end = (*self.data.get_mut(&target).unwrap()).end;
            self.open.push_back(Node::new(
                new_grid,
                new_cursor,
                parent.h + manhattan(parent.cursor, end) - manhattan(new_cursor, end),
                parent.g + 1,
                Some(Rc::clone(&parent)),
            ));
        }
    }

    fn add_children(&mut self, parent: Rc<Node>) {
        if parent.cursor.0 != 0 {
            self.add_child(&parent, (parent.cursor.0 - 1, parent.cursor.1));
        }
        if parent.cursor.0 + 1 < parent.grid.len() {
            self.add_child(&parent, (parent.cursor.0 + 1, parent.cursor.1));
        }
        if parent.cursor.1 != 0 {
            self.add_child(&parent, (parent.cursor.0, parent.cursor.1 - 1));
        }
        if parent.cursor.1 + 1 < parent.grid.len() {
            self.add_child(&parent, (parent.cursor.0, parent.cursor.1 + 1));
        }
        self.closed.insert(parent.grid.iter().cloned().collect());
    }

    pub fn solve(&mut self) {
        while self.open[0].h != 0 {
            let parent = Rc::new(self.open.pop_front().unwrap());

            self.add_children(parent);
            self.open
                .make_contiguous()
                // .sort_by(|a, b| (a.h + a.g).cmp(&(b.h + b.g)));
                .sort_by(|a, b| (a.h).cmp(&(b.h)));
        }
    }
}
