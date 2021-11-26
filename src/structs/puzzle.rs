use super::data::*;
use super::node::*;
use crate::input::*;
use n_puzzle::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::rc::Rc;

static END: (
    [[u8; 1]; 1],
    [[u8; 2]; 2],
    [[u8; 3]; 3],
    [[u8; 4]; 4],
    [[u8; 5]; 5],
    [[u8; 6]; 6],
) = (
    [[0]],
    [[1, 2], [0, 3]],
    [[1, 2, 3], [8, 0, 4], [7, 6, 5]],
    [[1, 2, 3, 4], [12, 13, 14, 5], [11, 0, 15, 6], [10, 9, 8, 7]],
    [
        [1, 2, 3, 4, 5],
        [16, 17, 18, 19, 6],
        [15, 24, 0, 20, 7],
        [14, 23, 22, 21, 8],
        [13, 12, 11, 10, 9],
    ],
    [
        [1, 2, 3, 4, 5, 6],
        [20, 21, 22, 23, 24, 7],
        [19, 32, 33, 34, 25, 8],
        [18, 31, 0, 35, 26, 9],
        [17, 30, 29, 28, 27, 10],
        [16, 15, 14, 13, 12, 11],
    ],
);

fn manhattan(a: (usize, usize), b: (usize, usize)) -> u16 {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 } + if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
        as u16
}

fn get_data(u: u8, y: usize, x: usize) -> Data {
    for (j, row) in END.3.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if u == *v {
                return Data::new(
                    u,
                    if u != 0 { manhattan((y, x), (j, i)) } else { 0 },
                    (y, x),
                    (j, i),
                );
            }
        }
    }
    Data::new(0, 0, (y, x), (y, x))
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

    pub fn initialize_start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut start = Node::new(get_grid()?, (0, 0), 0, 0, None);
        for (y, row) in start.grid.iter().enumerate() {
            for (x, u) in row.iter().enumerate() {
                self.data.insert(*u, get_data(*u, y, x));
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
