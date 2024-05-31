use std::{
    fmt::Display, 
    io, 
    str::FromStr,
};

use crate::cubelet::Axis;

/// Number of turns on the most negative face, number of turns on the most positive face,
/// and the axis on which the turns happen
#[derive(Clone, Copy, Debug, Hash)]
pub struct Move(pub Axis, pub u8, pub u8);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl FromStr for Move {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 {
            let [axis, rot1, rot2] = s.as_bytes().try_into().unwrap();
            let axis = match axis {
                b'X' => Axis::X,
                b'Y' => Axis::Y,
                b'Z' => Axis::Z,
                _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, s)) }
            };
            let rot1 = match rot1 {
                b'0' | b'1' | b'2' | b'3' => rot1 - b'0',
                _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, s)) }
            };
            let rot2 = match rot2 {
                b'0' | b'1' | b'2' | b'3' => rot2 - b'0',
                _ => { return Err(io::Error::new(io::ErrorKind::InvalidData, s)) }
            };

            Ok(Move(axis, rot1, rot2))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, s))
        }
    }
}

impl Move {
    pub const ALL: [Move; 45] = {
        let mut i = 0;
        let mut res = [Move(Axis::X, 0, 0); 45];
        while i < 4 {
            let mut j = 0;
            while j < 4 {
                if !(i == 0 && j == 0) {
                    res[i*12 + j*3 - 3] = Move(Axis::X, i as u8, j as u8);
                    res[i*12 + j*3 - 2] = Move(Axis::Y, i as u8, j as u8);
                    res[i*12 + j*3 - 1] = Move(Axis::Z, i as u8, j as u8);
                }
                j += 1;
            }
            i += 1;
        }
        res
    };

    const fn axis_moves(axis: Axis) -> [Move; 15] {
        let mut i = 1;
        let mut res = [Move(axis, 0, 0); 15];
        while i < 16 {
            res[i - 1].1 = i as u8 / 4;
            res[i - 1].2 = i as u8 % 4;
            i += 1;
        }
        res
    }

    pub const X: [Move; 15] = Self::axis_moves(Axis::X);
    pub const Y: [Move; 15] = Self::axis_moves(Axis::Y);
    pub const Z: [Move; 15] = Self::axis_moves(Axis::Z);

    pub fn inverse(self) -> Self {
        let Move(axis, rot1, rot2) = self;
        Move(axis, (4 - rot1) % 4, (4 - rot2) % 4)
    }
}

