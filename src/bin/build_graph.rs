use std::{fs::File, io::{self, BufReader, BufRead, Write}, str::FromStr};

use petgraph::prelude::*;
use rubiks::{rubiks::{Cube, Move}, view::CompressedCube};

type Graph = GraphMap<RawCube, Move, Directed>;

const CHUNK_SIZE: usize = 8192;
const MAX_LINE_LENGTH: usize = 45;

fn main() {
    let mut cube = Cube::default();
    let mut graph: Graph = GraphMap::new();
    graph.add_node(RawCube::new(cube.clone()));
    for m in Move::ALL {
        let new_cube = cube.clone().make_move(m);
        graph.add_node(RawCube::new(new_cube.clone()));
        graph.add_edge(RawCube::new(cube.clone()), RawCube::new(new_cube.clone()), m);
        cube = new_cube;
    }
    // println!("{:?}", &graph);
    save_graph(graph, "test_graph.txt").unwrap();

    let graph = load_graph("test_graph.txt").unwrap();
    save_graph(graph, "test_graph2.txt").unwrap();
    // println!("{:?}", graph);
}

enum Line {
    Node(CompressedCube),
    Edge(CompressedCube, CompressedCube, Move)
}

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct RawCube([u8; 20]);

impl RawCube {
    fn new(cube: Cube) -> Self {
        unsafe { RawCube(std::mem::transmute(cube.cubelets)) }
    }

    fn into_inner(self) -> Cube {
        unsafe { Cube { cubelets: std::mem::transmute(self.0) } }
    }
}

impl FromStr for Line {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            // Range to account for the possible absence of a new line character
            20..=21 => {
                println!("{}", s.len());
                let node = Line::Node(s.trim_end().parse()?);
                Ok(node)
            }
            45..=46 => {
                println!("{}", s.len());
                let parts: [&str; 3] = s
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, s))?;
                let edge = Line::Edge(parts[0].parse()?, parts[1].parse()?, parts[2].trim_end().parse()?);
                Ok(edge)
            }
            _ => { Err(io::Error::new(io::ErrorKind::InvalidData, s)) }
        }
    }
}

fn load_graph(file_path: &str) -> Result<Graph, std::io::Error> {
    let mut graph = Graph::new();
    let mut reader = BufReader::with_capacity(CHUNK_SIZE, File::open(file_path).unwrap());
    let mut buf = String::with_capacity(MAX_LINE_LENGTH);

    while reader.read_line(&mut buf)? != 0 {
        let line: Line = buf.parse()?;
        match line {
            Line::Node(CompressedCube(cube)) => {
                graph.add_node(RawCube::new(cube));
            }
            Line::Edge(CompressedCube(src), CompressedCube(dest), m) => {
                graph.add_edge(RawCube::new(src), RawCube::new(dest), m);
            }
        }
        buf.clear();
    }

    Ok(graph)
}

fn save_graph(graph: Graph, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    for node in graph.nodes() {
        file.write(format!("{}\n", CompressedCube(node.into_inner())).as_bytes())?;
    }
    for (src, dest, m) in graph.all_edges() {
        file.write(format!(
            "{} {} {}\n",
            CompressedCube(src.into_inner()),
            CompressedCube(dest.into_inner()),
            m,
        ).as_bytes())?;
    }
    Ok(())
}
