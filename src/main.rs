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
    strategy::{MultiTree, PartialTree, Tree, Strategy, Cycle},
    book::Book
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3)]
    depth: u8
}

fn main() -> Result<(), std::io::Error> {
    // let args = Args::parse();

    // let strategy = Tree { prev_move: None, current_depth: 0, search_depth: args.depth };
    // let strategy = Cycle { moves: vec![ Move(1, 0, Axis::X), Move(0, 1, Axis::Y) ] };
    // let strategy = PartialTree { axes: vec![Axis::X, Axis::Y, Axis::X, Axis::Z, Axis::Y], current_depth: 0 };
    // let strategy = MultiTree {
    //     search_depth: args.depth,
    //     jobs: Move::ALL.into_iter().map(|m| vec![m]).collect()
    // };
    // let mut book = Book::with_capacity(34_000_000);
    // book.expand(strategy, vec![]);

    // let mut summary: HashMap<u8, usize> = HashMap::new();
    // for (_cube, depth) in book.iter() {
    //     let val = summary.entry(*depth).or_insert(0);
    //     *val += 1;
    // }
    // let mut summary: Vec<_> = summary.into_iter().collect();
    // summary.sort();
    // book.save(&format!("depth_{}.book", args.depth))?;
    // book.save(&format!("cycle_xy.book"))?;

    // for (k, v) in summary {
    //     println!("{}: Cubes={}", k, v);
    // }

    // let not_in_depth3 = Book::load("test/not_in_depth3.book")?;
    // let mut search_cube = CubePath::default();
    // for target_cube in not_in_depth3.keys() {
    //     println!("\n=========================================================\n{}", DisplayCube(target_cube.clone()));
    //     let mut paths = HashSet::new();
    //     cube::search(&mut paths, &mut search_cube, &target_cube, 0, 3);
    //     for path in paths {
    //         println!("{:?}", path);
    //     }
    // }

    Ok(())
}
