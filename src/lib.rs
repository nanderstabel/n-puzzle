
pub type Grid = Vec<Vec<u8>>;

#[derive(Debug, Clone)]
pub struct Node<'a> {
    grid: Grid,
    parent: &'a Node<'a>
}

#[derive(Debug, Default)]
pub struct Puzzle {
    buf: String
}
