mod input;
mod structs;

use std::error::Error;
use structs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new();
    puzzle.initialize_start()?;

    println!("{:?}", puzzle.open[0]);

    // let cursor = (*puzzle.data.get_mut(&0).unwrap()).current;
    // if start.cursor.0 != 0 {
    //     let new_cursor = (cursor.0 - 1, cursor.1);

    //     let new_grid: Grid = start
    //         .grid
    //         .iter()
    //         .cloned()
    //         .collect::<Grid>()
    //         .swap(cursor, new_cursor);

    //     let target = new_grid[cursor.0][cursor.1];
    //     let meta = &(*puzzle.data.get_mut(&target).unwrap());

    //     let new_h = manhattan(cursor, meta.end);

    //     let child_up = Node::new(new_grid, new_cursor, new_h, 1, Some(&start));

    //     println!("{:?}", child_up);
    //     println!("{:?}\n\nnew: {}\n\n", meta, new_h as i16 - meta.h as i16);
    // }

    // if start.cursor.0 + 1 < start.grid.len() {
    //     let new_cursor = (cursor.0 + 1, cursor.1);

    //     let new_grid: Grid = start
    //         .grid
    //         .iter()
    //         .cloned()
    //         .collect::<Grid>()
    //         .swap(cursor, new_cursor);

    //     let target = new_grid[cursor.0][cursor.1];
    //     let meta = &(*puzzle.data.get_mut(&target).unwrap());

    //     let new_h = manhattan(cursor, meta.end);

    //     let child_down = Node::new(new_grid, new_cursor, new_h, 1, Some(&start));

    //     println!("{:?}", child_down);
    //     println!("{:?}\n\nnew: {}\n\n", meta, new_h as i16 - meta.h as i16);
    // }

    // if start.cursor.0 + 1 < start.grid.len() {
    //     let target = start.grid[start.cursor.0 + 1][start.cursor.1];
    //     let meta = &(*data.get_mut(&target).unwrap());
    //     println!(
    //         "{:?}\t\n\tnew: {}\n\n",
    //         meta,
    //         manhattan(meta.current.1, meta.current.0 - 1, meta.end.1, meta.end.0) as i16
    //             - meta.h as i16
    //     );
    // }
    // if start.cursor.1 != 0 {
    //     let target = start.grid[start.cursor.0][start.cursor.1 - 1];
    //     let meta = &(*data.get_mut(&target).unwrap());
    //     println!(
    //         "{:?}\tnew: {}\n\n",
    //         meta,
    //         manhattan(meta.current.1 + 1, meta.current.0, meta.end.1, meta.end.0) as i16
    //             - meta.h as i16
    //     );
    // }
    // if start.cursor.1 + 1 < start.grid.len() {
    //     let target = start.grid[start.cursor.0][start.cursor.1 + 1];
    //     let meta = &(*data.get_mut(&target).unwrap());
    //     println!(
    //         "{:?}\tnew: {}\n\n",
    //         meta,
    //         manhattan(meta.current.1 - 1, meta.current.0, meta.end.1, meta.end.0) as i16
    //             - meta.h as i16
    //     );
    // }

    Ok(())
}
