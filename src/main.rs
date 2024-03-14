#![allow(dead_code)]
#![allow(unused_imports)]

use std::borrow::BorrowMut;
use std::{
    collections::HashMap,
    io::Write
};

use rubiks::rubiks::{Cube, Move, index};
use rubiks::cube::{Cubelet, Rotation, Axis};

const END: u8 = 5;

fn main() -> Result<(), std::io::Error> {
    let my_cube = Cube::default()
        .make_move(Move(0, 1, Axis::X))
        .make_move(Move(0, 1, Axis::Y))
        .make_move(Move(0, 1, Axis::Z));
    println!("{}", my_cube);

    // let mut cubes = HashMap::from([
    //     (Cube::default(), Info { depth: 0, parity: 0 })
    // ]);
    // cubes.reserve(37_000_000);

    // // depth-first search
    // dfs(&mut cubes, Cube::default(), None, 0, END);

    // let mut summary: HashMap<u8, (usize, usize)> = HashMap::new();
    // for (_, info) in cubes {
    //     let Info { parity, depth } = info;
    //     let val = summary.entry(depth).or_insert((0, 0));
    //     val.0 += 1;
    //     val.1 += parity as usize;
    // }

    // println!("Size of CubeletsArrangement: {} bytes", std::mem::size_of::<Cube>());
    // for (k, v) in summary {
    //     println!("{}: Cubes={}, Avg. parity={}", k, v.0, v.1 as f32 / v.0 as f32);
    // }

    println!("index::<0,0,0>() = {}", index::<0,0,0>());
    println!("index::<0,0,1>() = {}", index::<0,0,1>());
    println!("index::<0,0,2>() = {}", index::<0,0,2>());
    println!("index::<0,1,0>() = {}", index::<0,1,0>());
    println!("index::<0,1,2>() = {}", index::<0,1,2>());
    println!("index::<0,2,0>() = {}", index::<0,2,0>());
    println!("index::<0,2,1>() = {}", index::<0,2,1>());
    println!("index::<0,2,2>() = {}", index::<0,2,2>());
                                                   
    println!("index::<1,0,0>() = {}", index::<1,0,0>());
    println!("index::<1,0,2>() = {}", index::<1,0,2>());
    println!("index::<1,2,0>() = {}", index::<1,2,0>());
    println!("index::<1,2,2>() = {}", index::<1,2,2>());
                                                   
    println!("index::<2,0,0>() = {}", index::<2,0,0>());
    println!("index::<2,0,1>() = {}", index::<2,0,1>());
    println!("index::<2,0,2>() = {}", index::<2,0,2>());
    println!("index::<2,1,0>() = {}", index::<2,1,0>());
    println!("index::<2,1,2>() = {}", index::<2,1,2>());
    println!("index::<2,2,0>() = {}", index::<2,2,0>());
    println!("index::<2,2,1>() = {}", index::<2,2,1>());
    println!("index::<2,2,2>() = {}", index::<2,2,2>());

    Ok(())
}

fn dfs(
    cubes: &mut HashMap<Cube, Info>,
    cube: Cube,
    last_move: Option<Move>,
    depth: u8,
    max_depth: u8
) {
    if depth < max_depth {
        let iter: Box<dyn Iterator<Item=Move>> = match last_move {
            Some(Move(_, _, Axis::X)) => Box::new(Move::Y.into_iter().chain(Move::Z.into_iter())),
            Some(Move(_, _, Axis::Y)) => Box::new(Move::X.into_iter().chain(Move::Z.into_iter())),
            Some(Move(_, _, Axis::Z)) => Box::new(Move::X.into_iter().chain(Move::Y.into_iter())),
            None => Box::new(Move::ALL.into_iter()),
        };
        let new_depth = depth + 1;
        for m in iter {
            let new = cube.clone().make_move(m);
            let info = cubes.entry(new.clone()).or_insert(Info {
                depth: new_depth,
                parity: new.parity()
            });

            // Dijkstra bc depth-first gets things wrong
            // if info.depth < new_depth {
            //     println!("{}", &new);
            // }
            info.depth = std::cmp::min(new_depth, info.depth);

            dfs(cubes, new, Some(m), new_depth, max_depth);
        }
    }
}

#[derive(Debug)]
struct Info {
    depth: u8,
    parity: u8,
    // distance: usize // TODO
}

