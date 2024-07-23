#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    io::Write
};

use clap::Parser;
use rubiks::{
    cube::{Cube, self},
    cubelet::{Cubelet, Rotation, Axis},
    view::DisplayCube,
    book::Book
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    depth: u8
}

fn main() -> Result<(), std::io::Error> {

    Ok(())
}
