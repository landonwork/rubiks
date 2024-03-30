#![allow(dead_code)]
#![allow(unused_imports)]

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::Write
};

use rubiks::{
    cube::{Cube, Move, Info, index},
    cubelet::{Cubelet, Rotation, Axis},
    graph::CubeGraph,
    view::DisplayCube
};

const END: u8 = 5;

fn main() -> Result<(), std::io::Error> {
    // let my_cube = Cube::default()
    //     .make_move(Move(0, 1, Axis::X))
    //     .make_move(Move(0, 1, Axis::Y))
    //     .make_move(Move(0, 1, Axis::Z));
    // println!("{}", &my_cube);
    // println!("{}", DisplayCube(my_cube));

    // let mut graph = CubeGraph::new();

    let strategy = BasicStrategy { last_move: None, search_depth: 5 };
    let cubes = strategy.explore([Cube::default()].as_slice());
    let mut cubes_map = HashMap::with_capacity(cubes.len() + 1);
    cubes_map.insert(Cube::default(), 0);
    cubes.into_iter()
        .for_each(|(cube, depth)| match cubes_map.get_mut(&cube) {
            None => { cubes_map.insert(cube, depth); }
            Some(depth_mut) => { *depth_mut = std::cmp::min(depth, *depth_mut); }
        });

    let mut summary: HashMap<u8, usize> = HashMap::new();
    for depth in cubes_map.values() {
        let val = summary.entry(*depth).or_insert(0);
        *val += 1;
    }

    // println!("Size of Cube: {} bytes", std::mem::size_of::<Cube>());
    for (k, v) in summary {
        println!("{}: Cubes={}", k, v);
    }

    Ok(())
}

pub trait Strategy {
    fn explore(&self, to_explore: &[Cube]) -> Vec<(Cube, u8)>;
}

struct BasicStrategy {
    pub search_depth: u8,
    pub last_move: Option<Move>,
}

fn basic_explore(
    cube: &Cube,
    last_move: Option<Move>,
    depth: u8,
    max_depth: u8,
) -> Vec<(Cube, u8)> {
    // if depth == max_depth { vec![(cube.clone(), depth)] } else {
    if depth == max_depth { vec![] } else {
        let moves: Box<dyn Iterator<Item=Move>> = match last_move {
            None => Box::new(Move::ALL.into_iter()),
            Some(Move(_, _, Axis::X)) => Box::new(Move::Y.into_iter().chain(Move::Z.into_iter())),
            Some(Move(_, _, Axis::Y)) => Box::new(Move::X.into_iter().chain(Move::Z.into_iter())),
            Some(Move(_, _, Axis::Z)) => Box::new(Move::X.into_iter().chain(Move::Y.into_iter())),
        };

        moves.flat_map(|m| {
                let mut new_cubes = vec![(cube.clone().make_move(m), depth + 1)];
                new_cubes.extend(basic_explore(&new_cubes[0].0, Some(m), depth + 1, max_depth));
                new_cubes
            })
            .collect()
    }
}

impl Strategy for BasicStrategy {
    fn explore(&self, to_explore: &[Cube]) -> Vec<(Cube, u8)> {
        to_explore.iter()
            .flat_map(|cube| basic_explore(cube, self.last_move, 0, self.search_depth))
            .collect()
    }
}
