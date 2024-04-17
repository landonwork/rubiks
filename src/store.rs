use std::{
    fs,
    collections::HashMap,
    io::{self, BufReader, BufWriter, Read, Write},
    ops::{Deref, DerefMut},
    os::windows::fs::MetadataExt,
};

use crate::{
    cube::Cube,
    strategy::{Update, Strategy}
};

// `DepthMap` would probably be more accurate
#[derive(Clone)]
pub struct Store(pub HashMap<Cube, u8>);

impl Deref for Store {
    type Target = HashMap<Cube, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Store {
    pub fn new() -> Self {
        let map = HashMap::from([(Cube::default(), 0)]);
        Self(map)
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut map = HashMap::with_capacity(cap);
        map.insert(Cube::default(), 0);
        Self(map)
    }

    pub fn expand(&mut self, strategy: impl Strategy, criteria: Vec<Box<dyn Fn(&Cube, u8) -> bool>>) {
        let cubes: Vec<_> = self.iter()
            .filter_map(|(cube, &depth)| criteria.iter().all(|crit| crit(cube, depth)).then_some(cube.clone()))
            .collect();
        let closure = |cube| strategy.explore(self, &cube);
        cubes.into_iter().for_each(closure);
    }

    pub fn extend_from_store(&mut self, other: Store) {
        let mut cubes_to_update = Vec::new();
        other.0.into_iter()
            .for_each(|(cube, depth)| {
                self.entry(cube.clone())
                    .and_modify(|depth_mut_ref| {
                        if depth < *depth_mut_ref {
                            *depth_mut_ref = depth;
                            cubes_to_update.push(cube);
                        }
                    })
                    .or_insert(depth);
            });
        cubes_to_update.into_iter().for_each(|cube| Update.explore(self, &cube));
    }

    pub fn load(file_name: &str) -> io::Result<Self> {
        let file = fs::File::open(file_name)?;
        let size = file.metadata()?.file_size();
        let mut reader = BufReader::new(file);
        let mut buffer: [u8; 21] = [0; 21];
        let mut store = HashMap::with_capacity(size as usize / 21);

        loop {
            match reader.read_exact(&mut buffer) {
                Ok(()) => {
                    let cubelets: [u8; 20] = buffer[..20].try_into().unwrap();
                    let cube = Cube { cubelets: unsafe { std::mem::transmute(cubelets) } };
                    store.insert(cube, buffer[20]);
                }
                Err(error) if matches!(error.kind(), io::ErrorKind::UnexpectedEof) => {
                    break
                }
                error => { error?; }
            }
        }

        Ok(Store(store))
    }

    pub fn save(&self, file_name: &str) -> io::Result<()> {
        let file = fs::File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        for (k, v) in self.iter() {
            writer.write_all(k.as_bytes())?;
            writer.write_all([*v].as_slice())?;
        }
        writer.flush()?;
        Ok(())
    }
}

// Something with an index; maybe it could be parallelized and use swap memory;
// maybe a wrapper on a database, like Redis, which gets backed up automatically?
// It would be nice if I didn't have to hold the whole thing in memory at once.
// pub struct Table {
//     
// }

// Cubes mapped to its depth and the best move towards the solved state
// pub struct BackwardTrie {
//     inner: HashMap<Cube, (u8, Move)>
// }
