use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
    str::FromStr,
};

use crate::cube::{Cube, Move};

const CHUNK_SIZE: usize = 8192;
// const MAX_LINE_LENGTH: usize = 45;

pub trait Strategy {
    fn next_moves(&self) -> Box<dyn Iterator<Item = Move>>;
    fn descend(&mut self, m: Move);
    fn ascend(&mut self);
}

// Changing to simpler data structure to save space.
// The graph edges are too much and I can't think of a reason they would be useful,
// but I will try to keep track of how many neighbors have been visited.
// key: cube state, value: depth
pub struct CubeStore(HashMap<Cube, u8>);

impl CubeStore {
    pub fn with_capacity(cap: usize) -> Self {
        let mut hashmap = HashMap::with_capacity(cap);
        hashmap.insert(Cube::default(), 0);
        CubeStore(hashmap)
    }

    /// Add a new, unexplored cube to the store and update its info if it already exists.
    pub fn new_cube(&mut self, cube: Cube, depth: u8) {
        self.0
            .entry(cube)
            .and_modify(|d| { *d = std::cmp::min(*d, depth); })
            .or_insert(depth);
    }

    // Explore all neighbors of an existing node
    pub fn explore_node<S: Strategy>(&self, cube: &Cube, strat: &S) -> Vec<Cube> {
        debug_assert!(self.0.contains_key(cube));

        let mut cubes = Vec::with_capacity(45);
        cubes.extend(strat.next_moves().map(|m| cube.clone().make_move(m)));
        cubes
    }

    pub fn explore<S: Strategy>(&mut self, strat: &mut S) {
    }

    ///////// I/O /////////

    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {
        let mut reader = BufReader::with_capacity(CHUNK_SIZE, File::open(file_path).unwrap());
        let mut store = CubeStore::with_capacity(file_path.len() / 23);
        let mut buf = [0u8; 21];

        // TODO: Check and see if it is the correct error when it terminates
        // (`ErrorKind::UnexpectedEof`)
        loop {
            match reader.read_exact(&mut buf) {
                Ok(_) => {
                    let cube: Cube = std::str::from_utf8(&buf[..20])
                        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ""))?
                        .parse()?;
                    let depth = buf[20];
                    store.0.insert(cube, depth);
                }
                Err(e) if matches!(e.kind(), io::ErrorKind::UnexpectedEof) => { break; }
                error @ Err(_) => { error?; }
            }
        }

        Ok(store)
    }

    pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, File::create(file_path)?);

        for (cube, depth) in self.0.iter() {
            writer.write(cube.as_bytes())?;
            writer.write(&[*depth])?;
        }

        writer.flush()?;
        Ok(())
    }

    // pub fn save_info(&self, file_path: &str) -> Result<(), std::io::Error> {
    //     let mut writer = BufWriter::with_capacity(CHUNK_SIZE, File::create(file_path)?);

    //     for (cube, depth) in self.0.iter() {
    //         let info = cube.info();
    //         writer.write(info.as_bytes())?;
    //         writer.write(&[*depth])?;
    //     }

    //     writer.flush()?;
    //     Ok(())
    // }
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

