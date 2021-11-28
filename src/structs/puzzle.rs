use super::data::*;
use super::node::*;
use crate::input::*;
use n_puzzle::*;
use std::{
    cmp::min,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt,
    rc::Rc,
};

pub struct Puzzle {
    pub time_complexity: u64,
    pub size_complexity: u64,
    pub heuristic: fn((usize, usize), (usize, usize)) -> u16,
    pub data: HashMap<u8, Data>,
    pub fringe_db: HashMap<usize, Vec<u8>>,
    pub closed: HashSet<Grid>,
    pub open: VecDeque<Node>,
}

impl Puzzle {
    pub fn new(heuristic: fn((usize, usize), (usize, usize)) -> u16) -> Self {
        Puzzle {
            time_complexity: 1,
            size_complexity: 1,
            heuristic,
            data: HashMap::new(),
            fringe_db: HashMap::new(),
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
                if set.is_empty() {
                    break;
                }
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

        loop {
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
        for i in 1..(size * size) {
            let (x, y) = (*self.data.get_mut(&i).unwrap()).end;
            if !self.fringe_db.contains_key(min(&x, &y)) {
                self.fringe_db.insert(min(x, y), Vec::new());
            }
            (*self.fringe_db.get_mut(min(&x, &y)).unwrap()).push(i);
        }
        self.open.push_front(Node::new(
            grid,
            cursor,
            self.data
                .values()
                .map(|d| (self.heuristic)(d.current, d.end))
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
                parent.h + (self.heuristic)(parent.cursor, end) - (self.heuristic)(new_cursor, end),
                parent.g + 1,
                Some(Rc::clone(&parent)),
            ));
            self.time_complexity += 1;
            self.size_complexity += 1;
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
        self.closed.insert(parent.grid.clone());
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

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Traverse<'a> {
            r: &'a dyn Fn(&Traverse, &Node),
        }
        let traverse = Traverse {
            r: &|traverse, node| {
                if let Some(parent) = &node.parent {
                    (traverse.r)(&traverse, parent);
                }
                println!("{}", node);
            },
        };
        let node = &self.open[0];
        (traverse.r)(&traverse, node);
        write!(f, "Results\n{:_^24}\n\n", "")?;
        write!(f, "{:>16}{:>8}\n", "Time complexity:", self.time_complexity)?;
        write!(f, "{:>16}{:>8}\n", "Size complexity:", self.size_complexity)?;
        write!(f, "{:>16}{:>8}\n", "Total moves:", node.g)?;
        Ok(())
    }
}
