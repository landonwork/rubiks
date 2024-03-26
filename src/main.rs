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
    let my_cube = Cube::default()
        .make_move(Move(0, 1, Axis::X))
        .make_move(Move(0, 1, Axis::Y))
        .make_move(Move(0, 1, Axis::Z));
    println!("{}", &my_cube);
    println!("{}", DisplayCube(my_cube));

    let mut graph = CubeGraph::new();

    // depth-first search
    bfs(&mut graph, Cube::default(), None, 0, END);
    graph.save_info("depth5.info")?;

    let mut summary: HashMap<u8, (usize, usize)> = HashMap::new();
    for info in graph.graph.node_weights() {
        let Info { parity, depth } = info;
        let val = summary.entry(*depth).or_insert((0, 0));
        val.0 += 1;
        val.1 += *parity as usize;
    }

    println!("Size of Cube: {} bytes", std::mem::size_of::<Cube>());
    for (k, v) in summary {
        println!("{}: Cubes={}, Avg. parity={}", k, v.0, v.1 as f32 / v.0 as f32);
    }

    Ok(())
}

fn bfs(
    graph: &mut CubeGraph,
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
            let info = Info {
                depth: new_depth,
                parity: new.parity()
            };

            graph.add_cube(cube.clone(), new.clone(), info, m);

            bfs(graph, new, Some(m), new_depth, max_depth);
        }
    }
}

