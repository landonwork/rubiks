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
pub struct Book(pub Db);


