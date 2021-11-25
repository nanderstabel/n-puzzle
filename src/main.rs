/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/25 20:11:22 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use input::*;
use n_puzzle::*;
use std::{collections::HashMap, error::Error};

static END: (
    [[u8; 1]; 1],
    [[u8; 2]; 2],
    [[u8; 3]; 3],
    [[u8; 4]; 4],
    [[u8; 5]; 5],
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

fn main() -> Result<(), Box<dyn Error>> {
    // let puzzle = Puzzle::default();
    let mut data = HashMap::new();
    let mut start = Node::new(get_grid()?, (0, 0), 0, None);
    for (y, row) in start.grid.iter().enumerate() {
        for (x, u) in row.iter().enumerate() {
            data.insert(u, get_data(*u, y, x));
        }
    }
    start.h = data.values().map(|d| d.h).sum();
    start.cursor = (*data.get_mut(&0).unwrap()).current;

    println!("{:?}", start);
    // println!("{:?}", data);

    let cursor = (*data.get_mut(&0).unwrap()).current;
    if start.cursor.0 != 0 {
        let new_cursor = (cursor.0 - 1, cursor.1);

        let new_grid: Grid = start
            .grid
            .iter()
            .cloned()
            .collect::<Grid>()
            .swap(cursor, new_cursor);

        let target = new_grid[cursor.0][cursor.1];
        let meta = &(*data.get_mut(&target).unwrap());

        let new_h = manhattan(cursor, meta.end);

        let child = Node::new(new_grid, new_cursor, new_h, None);

        println!("{:?}", child);
        println!("{:?}\tnew: {}\n\n", meta, new_h as i16 - meta.h as i16);
    }

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
