#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    io::Write
};

use clap::Parser;
use rubiks_lab_rs::prelude::*;
use rubiks_lab_rs::view::DisplayCube;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    depth: u8,
    // #[arg(short, long, default_value_t = false)]
    // random: usize,
    // #[arg(short, long)]
    // n_scrambles: Option<usize>,
}

fn main() -> Result<(), std::io::Error> {
    // let book = Book

    Ok(())
}
