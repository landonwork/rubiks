#![allow(dead_code)]
#![allow(unused_imports)]

use rubiks::rubiks::{CubeletsArrangement, Move};

use std::io::Write;
use rubiks::cube::{Cubelet, Rotation, Axis};

fn main() -> Result<(), std::io::Error> {
    let mut cube = CubeletsArrangement::default();
    cube.make_move(Move(1, 0, Axis::X));
    // cube.make_move(Move(1, 0, Axis::Y));
    cube.make_move(Move(0, 3, Axis::Z));
    println!("{}", cube);
    Ok(())
}

