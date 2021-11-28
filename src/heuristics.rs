use n_puzzle::*;

pub fn manhattan((cy, cx): (usize, usize), (ey, ex): (usize, usize), grid: Grid) -> u16 {
    (if cy > ey { cy - ey } else { ey - cy } + if cx > ex { cx - ex } else { ex - cx }) as u16
}

pub fn hamming((cy, cx): (usize, usize), (ey, ex): (usize, usize), grid: Grid) -> u16 {
    if (cy, cx) == (ey, ex) {
        0
    } else {
        1
    }
}
