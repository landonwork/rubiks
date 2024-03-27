use std::{
    collections::HashMap, fs::File, io::{self, BufRead, BufReader, BufWriter, Read, Write}, str::FromStr
};

use crate::cube::{Cube, Info, Move};

const CHUNK_SIZE: usize = 8192;
const MAX_LINE_LENGTH: usize = 45;

// Changing to simpler data structure to save space.
// The graph edges are too much and I can't think of a reason they would be useful,
// but I will try to keep track of how many neighbors have been visited.
pub struct CubeStore(HashMap<Cube, StoreInfo>);

pub struct StoreInfo {
    /// Have all of the cube's neighbors been visited/discovered
    /// Visited - the cube exists in the data structure
    /// Explored - all of the cube's neighbors have been visited
    explored: bool,
    depth: u8,
    parity: u8
}

impl TryFrom<&[u8]> for StoreInfo {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // TODO: debug asserts for depth and parity
        debug_assert_eq!(value.len(), 3);
        debug_assert!(value[0] == b'T' || value[1] == b'F');

        let explored = match value[0] {
            b'T' => true,
            b'F' => false,
            _ => { unreachable!() }
        };
        let depth = value[1];
        let parity = value[2];
        Ok(StoreInfo { explored, depth, parity })
    }
}

impl CubeStore {
    pub fn with_capacity(cap: usize) -> Self {
        let mut hashmap = HashMap::with_capacity(cap);
        hashmap.insert(Cube::default(), StoreInfo { explored: false, depth: 0, parity: 0 });
        CubeStore(hashmap)
    }

    /// Add a new, unexplored cube to the store and update its info if it already exists.
    pub fn new_cube(&mut self, cube: Cube, depth: u8) {
        match self.0.get_mut(&cube) {
            Some(info) => {
                info.depth = std::cmp::min(info.depth, depth);
            }
            None => {
                self.0.insert(cube, StoreInfo { depth, parity: cube.parity(), explored: false });
            }
        }
    }

    pub fn load(file_path: &str) -> Result<Self, std::io::Error> {
        let mut reader = BufReader::with_capacity(CHUNK_SIZE, File::open(file_path).unwrap());
        let mut store = CubeStore::with_capacity(file_path.len() / 23);
        let mut buf = [0u8; 23];

        // TODO: Check and see if it is the correct error when it terminates
        // (`ErrorKind::UnexpectedEof`)
        while reader.read_exact(&mut buf).is_ok() {
            let cube: Cube = std::str::from_utf8(&buf[..20])
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, ""))?
                .parse()?;
            let info: StoreInfo = (&buf[20..]).try_into()?;
            store.0.insert(cube, info);
        }

        Ok(store)
    }

    pub fn save(&self, file_path: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::with_capacity(CHUNK_SIZE, File::create(file_path)?);

        // Nodes
        for (cube, info) in self.0.iter() {
            writer.write(format!("{} {} {}\n", cube, info.explored, info.depth, info.parity).as_bytes())?;
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

