use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    str::FromStr,
    collections::HashMap,
};

use petgraph::{prelude::*, stable_graph::DefaultIx};

use crate::cube::{Cube, Info, Move};

const CHUNK_SIZE: usize = 8192;
const MAX_LINE_LENGTH: usize = 45;

pub struct CubeGraph {
    // Shouldn't need stable graph because I will never remove nodes or edges
    // DefaultIx may be too small
    pub graph: Graph<Info, Move, Directed, DefaultIx>,
    pub indices: HashMap<Cube, NodeIndex>
}

impl CubeGraph {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        let neutral_ind = graph.add_node(Info { depth: 0, parity: 0 });
        CubeGraph {
            graph,
            indices: HashMap::from([(Cube::default(), neutral_ind)])
        }
    }

    pub fn add_cube(&mut self, src: Cube, dest: Cube, info: Info, m: Move) {
        match (self.indices.get(&src), self.indices.get(&dest)) {
            (Some(&src_ind), Some(&dest_ind)) => {
                self.graph.add_edge(src_ind, dest_ind, m);
                let dest_info = self.graph.node_weight_mut(dest_ind).unwrap();
                dest_info.depth = std::cmp::min(dest_info.depth, info.depth);
                // If updating gets anymore complicated
                // dest_info.update(info);
            }
            (Some(&src_ind), None) => {
                let dest_ind = self.graph.add_node(info);
                self.graph.add_edge(src_ind, dest_ind, m);
                self.indices.insert(dest, dest_ind);
            }
            (None, _) => { panic!(); }
        }
    }

    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {
        let mut graph = Self::new();
        let mut reader = BufReader::with_capacity(CHUNK_SIZE, File::open(file_path).unwrap());
        let mut buf = String::with_capacity(MAX_LINE_LENGTH);

        while reader.read_line(&mut buf)? != 0 {
            let line: Line = buf.parse()?;
            match line {
                Line::Node(cube, depth, parity) => {
                    let ind = graph.graph.add_node(Info { depth, parity });
                    graph.indices.insert(cube, ind);
                }
                Line::Edge(src, dest, m) => {
                    match (graph.indices.get(&src), graph.indices.get(&dest)) {
                        (Some(src_ind), Some(dest_ind)) => {
                            graph.graph.add_edge(*src_ind, *dest_ind, m);
                        }
                        _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, buf)); }
                    }
                }
            }
            buf.clear();
        }

        Ok(graph)
    }

    pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, File::create(file_path)?);

        // Nodes
        for (cube, &ind) in self.indices.iter() {
            let info = self.graph.node_weight(ind).unwrap();
            writer.write(format!("{} {} {}\n", cube, info.depth, info.parity).as_bytes())?;
        }

        // Edges (It's a little funny to do it this way, but I like the current structure of CubeGraph
        // and I wasn't willing to change it.)
        for (src, &ind) in self.indices.iter() {
            for edge in self.graph.edges(ind) {
                let m = edge.weight();
                let dest = src.clone().make_move(*m);
                writer.write(format!("{} {} {}\n", src, dest, m).as_bytes())?;
            }
        }

        writer.flush()?;
        Ok(())
    }

    pub fn save_info(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, File::create(file_path)?);

        // Nodes
        for ind in self.indices.values() {
            let info = self.graph.node_weight(*ind).unwrap();
            writer.write(&[info.depth, info.parity])?;
        }

        writer.flush()?;
        Ok(())
    }
}

enum Line {
    Node(Cube, u8, u8),
    Edge(Cube, Cube, Move)
}

impl FromStr for Line {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            // Range to account for the 1- to 2-digit numbers
            24..=27 => {
                let parts: [&str; 3] = s
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, s))?;
                let node = Line::Node(
                    parts[0].trim().parse()?, 
                    parts[1].trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, s))?, 
                    parts[2].trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, s))?
                );

                Ok(node)
            }
            // Range to account for the possible absence of a new line character
            45..=46 => {
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

