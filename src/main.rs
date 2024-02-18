#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    collections::HashMap,
    io::Write
};


use rubiks::rubiks::{CubeletsArrangement, Move};
use rubiks::cube::{Cubelet, Rotation, Axis};

fn main() -> Result<(), std::io::Error> {
    let mut cube = CubeletsArrangement::default();
    cube.make_move(Move(1, 0, Axis::X));
    // cube.make_move(Move(1, 0, Axis::Y));
    // cube.make_move(Move(0, 3, Axis::Z));
    println!("{}", &cube);
    println!("{:?}", info(&cube));
    Ok(())
}

#[derive(Debug)]
struct Info {
    parity: u8,
    // distance: usize // TODO
}

fn info(cube: &CubeletsArrangement) -> Box<dyn std::fmt::Debug> {
    Box::new(Info { parity: cube.iter().map(|c| c.len() ).sum() })
}
