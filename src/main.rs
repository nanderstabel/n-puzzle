/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/24 21:18:08 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use input::*;
use std::{
    error::Error,
    collections::HashMap
};
use n_puzzle::*;

fn main() -> Result<(), Box<dyn Error>> {
    // let puzzle = Puzzle::default();
    // let mut test = HashMap::new();

    let start = Node {
        grid: get_grid()?,
        parent: None
    };
    println!("{}", start);
    Ok(())
}
