/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/24 17:17:13 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use std::{
    error::Error,
    collections::HashMap
};

fn main() -> Result<(), Box<dyn Error>> {
    // let puzzle = Puzzle::default();
    // let mut test = HashMap::new();


    let grid = input::get_grid()?;
    println!("{:#?}", grid);
    Ok(())
}
