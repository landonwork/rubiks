// use std::{fs::File, io::{self, BufReader, BufRead, Write}, str::FromStr};

// use clap::Parser;
// use petgraph::prelude::*;
use rubiks::{cube::{Cube, Info, Move}, graph::CubeGraph};

// Maybe I should create many commands?
// build, join, extend?
// I should really keep Info at every node
// Yes, I've decided this will be for the command line
// I will build everything out in a graph module

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Options {
//     /// Type of search: breadth-first, cycle
//     #[arg(short, long)]
//     search: String,
//     /// Depth to search
//     #[arg(short, long, default_value_t = 5)]
//     depth: u8
// }

fn main() {
    let mut cube = Cube::default();
    let mut graph = CubeGraph::new();

    for m in Move::ALL {
        let new_cube = cube.clone().make_move(m);
        let info = Info { depth: 1, parity: new_cube.parity() };
        graph.add_cube(cube.clone(), new_cube.clone(), info, m);
    }
    // println!("{:?}", &graph);
    graph.save("test_graph.txt").unwrap();

    let graph = CubeGraph::load("test_graph.txt").unwrap();
    graph.save("test_graph2.txt").unwrap();
    // println!("{:?}", graph);
}

