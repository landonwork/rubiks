//! The actions provided for generating the Rubik's cube group. [`Move`] is the primary construct
//! and there will be an analagous type for familiar notation mapped to it. I can put a description
//! of how that works later. I hope to make it so you can pick and choose your set of generators.

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

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        (self.1 == 0 && self.2 == 0 && other.1 == 0 && other.2 == 0)
            || (self.0 == other.0 && self.1 == other.1 && self.2 == other.2)
    }
}
impl Eq for Move {}

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Turn {
    L, L2, L3,
    R, R2, R3,
    F, F2, F3,
    B, B2, B3,
    D, D2, D3,
    U, U2, U3,
}

impl From<Turn> for Move {
    fn from(value: Turn) -> Self {
        match value {
            Turn::L => Move(Axis::X, 1, 0),
            Turn::L2 => Move(Axis::X, 2, 0),
            Turn::L3 => Move(Axis::X, 3, 0),
            Turn::R => Move(Axis::X, 0, 3),
            Turn::R2 => Move(Axis::X, 0, 2),
            Turn::R3 => Move(Axis::X, 0, 1),
            Turn::F => Move(Axis::Y, 1, 0),
            Turn::F2 => Move(Axis::Y, 2, 0),
            Turn::F3 => Move(Axis::Y, 3, 0),
            Turn::B => Move(Axis::Y, 0, 3),
            Turn::B2 => Move(Axis::Y, 0, 2),
            Turn::B3 => Move(Axis::Y, 0, 1),
            Turn::D => Move(Axis::Z, 1, 0),
            Turn::D2 => Move(Axis::Z, 2, 0),
            Turn::D3 => Move(Axis::Z, 3, 0),
            Turn::U => Move(Axis::Z, 0, 3),
            Turn::U2 => Move(Axis::Z, 0, 2),
            Turn::U3 => Move(Axis::Z, 0, 1),
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::L => "L",
                Self::L2 => "L2",
                Self::L3 => "L'",
                Self::R => "R",
                Self::R2 => "R2",
                Self::R3 => "R'",
                Self::U => "U",
                Self::U2 => "U2",
                Self::U3 => "U'",
                Self::D => "D",
                Self::D2 => "D2",
                Self::D3 => "D'",
                Self::F => "F",
                Self::F2 => "F2",
                Self::F3 => "F'",
                Self::B => "B",
                Self::B2 => "B2",
                Self::B3 => "B'",
            }
        )
    }
}

impl Turn {
    const ALL: [Self; 18] = [Self::L, Self::L2, Self::L3, Self::R, Self::R2, Self::R3, Self::U, Self::U2, Self::U3, Self::D, Self::D2, Self::D3, Self::F, Self::F2, Self::F3, Self::B, Self::B2, Self::B3];
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum QuarterTurn {
    L, L3,
    R, R3,
    U, U3,
    D, D3,
    F, F3,
    B, B3,
}

impl From<QuarterTurn> for Move {
    fn from(value: QuarterTurn) -> Self {
        match value {
            QuarterTurn::L => Move(Axis::X, 1, 0),
            QuarterTurn::L3 => Move(Axis::X, 3, 0),
            QuarterTurn::R => Move(Axis::X, 0, 3),
            QuarterTurn::R3 => Move(Axis::X, 0, 1),
            QuarterTurn::F => Move(Axis::Y, 1, 0),
            QuarterTurn::F3 => Move(Axis::Y, 3, 0),
            QuarterTurn::B => Move(Axis::Y, 0, 3),
            QuarterTurn::B3 => Move(Axis::Y, 0, 1),
            QuarterTurn::D => Move(Axis::Z, 1, 0),
            QuarterTurn::D3 => Move(Axis::Z, 3, 0),
            QuarterTurn::U => Move(Axis::Z, 0, 3),
            QuarterTurn::U3 => Move(Axis::Z, 0, 1),
        }
    }
}

impl Display for QuarterTurn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::L => "L",
                Self::L3 => "L'",
                Self::R => "R",
                Self::R3 => "R'",
                Self::U => "U",
                Self::U3 => "U'",
                Self::D => "D",
                Self::D3 => "D'",
                Self::F => "F",
                Self::F3 => "F'",
                Self::B => "B",
                Self::B3 => "B'",
            }
        )
    }
}

impl QuarterTurn {
    const ALL: [Self; 12] = [Self::L, Self::L3, Self::R, Self::R3, Self::U, Self::U3, Self::D, Self::D3, Self::F, Self::F3, Self::B, Self::B3];
}

struct Word<T> {
    seq: Vec<T>
}
