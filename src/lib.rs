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
