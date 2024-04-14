// Before we go any farther, let us figure out the algebra of a single cube.
// We know there are 24 unique rotations of a cube. Let's make sure we can determine
// a cube's rotation based on its faces.

use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis { X, Y, Z, }

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::Y => "Y",
                Self::Z => "Z",
            }
        )
    }
}

/// The 24 possible rotations for a cube in their reduced form
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rotation {
    // neutral
    Neutral,
    // generators
    X, X2, X3, Y, Y2, Y3, Z, Z2, Z3,
    // others
    XY, XY2, XY3, XZ, XZ2, XZ3,
    X2Y, X2Y3, X2Z, X2Z3,
    X3Y, X3Y3, X3Z, X3Z3
}

impl TryFrom<u8> for Rotation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 24 {
            Err(value)
        } else {
            Ok(unsafe { std::mem::transmute(value) })
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Neutral => "Neutral",
                Self::X => "X",
                Self::X2 => "X2",
                Self::X3 => "X3",
                Self::Y => "Y",
                Self::Y2 => "Y2",
                Self::Y3 => "Y3",
                Self::Z => "Z",
                Self::Z2 => "Z2",
                Self::Z3 => "Z3",
                Self::XY => "XY",
                Self::XY2 => "XY2",
                Self::XY3 => "XY3",
                Self::XZ => "XZ",
                Self::XZ2 => "XZ2",
                Self::XZ3 => "XZ3",
                Self::X2Y => "X2Y",
                Self::X2Y3 => "X2Y3",
                Self::X2Z => "X2Z",
                Self::X2Z3 => "X2Z3",
                Self::X3Y => "X3Y",
                Self::X3Y3 => "X3Y3",
                Self::X3Z => "X3Z",
                Self::X3Z3 => "X3Z3",
            }
        )
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Neutral
    }
}

impl From<(u8, Axis)> for Rotation {
    fn from((turns, axis): (u8, Axis)) -> Self {
        match (turns % 4, axis) {
            (0, _) => Rotation::Neutral,
            (1, Axis::X) => Rotation::X,
            (2, Axis::X) => Rotation::X2,
            (3, Axis::X) => Rotation::X3,
            (1, Axis::Y) => Rotation::Y,
            (2, Axis::Y) => Rotation::Y2,
            (3, Axis::Y) => Rotation::Y3,
            (1, Axis::Z) => Rotation::Z,
            (2, Axis::Z) => Rotation::Z2,
            (3, Axis::Z) => Rotation::Z3,
            _ => unreachable!()
        }
    }
}

impl Rotation {
    /// All members of the cube rotation group
    pub const VARIANTS: [Rotation; 24] = [Self::Neutral, Self::X, Self::X2, Self::X3, Self::Y, Self::Y2, Self::Y3, Self::Z, Self::Z2, Self::Z3, Self::XY, Self::XY2, Self::XY3, Self::XZ, Self::XZ2, Self::XZ3, Self::X2Y, Self::X2Y3, Self::X2Z, Self::X2Z3, Self::X3Y, Self::X3Y3, Self::X3Z, Self::X3Z3];
    /// This is a misnomer. "Generators" in this case is any rotation operand that can be caused by
    /// an action on a Rubiks' cube.
    pub const GENERATORS: [Rotation; 9] = [Self::X, Self::X2, Self::X3, Self::Y, Self::Y2, Self::Y3, Self::Z, Self::Z2, Self::Z3];

    #[inline]
    pub const fn into_usize(self) -> usize {
        unsafe { std::mem::transmute::<Self, u8>(self) as usize }
    }

    const fn simple_rotation(left: Self, axis: Axis, turns: u8) -> Self {
        if turns == 0 { left } else {
            let new = match (left, axis) {
                (Self::Neutral, Axis::X) => Self::X,
                (Self::X,    Axis::X) => Self::X2,
                (Self::X2,   Axis::X) => Self::X3,
                (Self::X3,   Axis::X) => Self::Neutral,
                (Self::Y,    Axis::X) => Self::XZ,
                (Self::Y2,   Axis::X) => Self::XZ2,
                (Self::Y3,   Axis::X) => Self::XZ3,
                (Self::Z,    Axis::X) => Self::XY3,
                (Self::Z2,   Axis::X) => Self::XY2,
                (Self::Z3,   Axis::X) => Self::XY,
                (Self::XY,   Axis::X) => Self::X2Z,
                (Self::XY2,  Axis::X) => Self::Y2,
                (Self::XY3,  Axis::X) => Self::X2Z3,
                (Self::XZ,   Axis::X) => Self::X2Y3,
                (Self::XZ2,  Axis::X) => Self::Z2,
                (Self::XZ3,  Axis::X) => Self::X2Y,
                (Self::X2Y,  Axis::X) => Self::X3Z,
                (Self::X2Y3, Axis::X) => Self::X3Z3,
                (Self::X2Z,  Axis::X) => Self::X3Y3,
                (Self::X2Z3, Axis::X) => Self::X3Y,
                (Self::X3Y,  Axis::X) => Self::Z,
                (Self::X3Y3, Axis::X) => Self::Z3,
                (Self::X3Z,  Axis::X) => Self::Y3,
                (Self::X3Z3, Axis::X) => Self::Y,

                (Self::Neutral, Axis::Y) => Self::Y,
                (Self::X,    Axis::Y) => Self::XY,
                (Self::X2,   Axis::Y) => Self::X2Y,
                (Self::X3,   Axis::Y) => Self::X3Y,
                (Self::Y,    Axis::Y) => Self::Y2,
                (Self::Y2,   Axis::Y) => Self::Y3,
                (Self::Y3,   Axis::Y) => Self::Neutral,
                (Self::Z,    Axis::Y) => Self::XZ,
                (Self::Z2,   Axis::Y) => Self::X2Y3,
                (Self::Z3,   Axis::Y) => Self::X3Z3,
                (Self::XY,   Axis::Y) => Self::XY2,
                (Self::XY2,  Axis::Y) => Self::XY3,
                (Self::XY3,  Axis::Y) => Self::X,
                (Self::XZ,   Axis::Y) => Self::X2Z,
                (Self::XZ2,  Axis::Y) => Self::X3Y3,
                (Self::XZ3,  Axis::Y) => Self::Z3,
                (Self::X2Y,  Axis::Y) => Self::Z2,
                (Self::X2Y3, Axis::Y) => Self::X2,
                (Self::X2Z,  Axis::Y) => Self::X3Z,
                (Self::X2Z3, Axis::Y) => Self::XZ3,
                (Self::X3Y,  Axis::Y) => Self::XZ2,
                (Self::X3Y3, Axis::Y) => Self::X3,
                (Self::X3Z,  Axis::Y) => Self::Z,
                (Self::X3Z3, Axis::Y) => Self::X2Z3,

                (Self::Neutral, Axis::Z) => Self::Z,
                (Self::X,    Axis::Z) => Self::XZ,
                (Self::X2,   Axis::Z) => Self::X2Z,
                (Self::X3,   Axis::Z) => Self::X3Z,
                (Self::Y,    Axis::Z) => Self::X3Y,
                (Self::Y2,   Axis::Z) => Self::X2Z3,
                (Self::Y3,   Axis::Z) => Self::XY3,
                (Self::Z,    Axis::Z) => Self::Z2,
                (Self::Z2,   Axis::Z) => Self::Z3,
                (Self::Z3,   Axis::Z) => Self::Neutral,
                (Self::XY,   Axis::Z) => Self::Y,
                (Self::XY2,  Axis::Z) => Self::X3Z3,
                (Self::XY3,  Axis::Z) => Self::X2Y3,
                (Self::XZ,   Axis::Z) => Self::XZ2,
                (Self::XZ2,  Axis::Z) => Self::XZ3,
                (Self::XZ3,  Axis::Z) => Self::X,
                (Self::X2Y,  Axis::Z) => Self::XY,
                (Self::X2Y3, Axis::Z) => Self::X3Y3,
                (Self::X2Z,  Axis::Z) => Self::Y2,
                (Self::X2Z3, Axis::Z) => Self::X2,
                (Self::X3Y,  Axis::Z) => Self::X2Y,
                (Self::X3Y3, Axis::Z) => Self::Y3,
                (Self::X3Z,  Axis::Z) => Self::XY2,
                (Self::X3Z3, Axis::Z) => Self::X3,
            };

            Self::simple_rotation(new, axis, turns - 1)
        }
    }

    const fn into_parts(self) -> [(Axis, u8); 2] {
        match self {
            Self::Neutral => [(Axis::X, 0), (Axis::X, 0)],
            Self::X => [(Axis::X, 1), (Axis::X, 0)],
            Self::X2 => [(Axis::X, 2), (Axis::X, 0)],
            Self::X3 => [(Axis::X, 3), (Axis::X, 0)],
            Self::Y => [(Axis::Y, 1), (Axis::Y, 0)],
            Self::Y2 => [(Axis::Y, 2), (Axis::Y, 0)],
            Self::Y3 => [(Axis::Y, 3), (Axis::Y, 0)],
            Self::Z => [(Axis::Z, 1), (Axis::Z, 0)],
            Self::Z2 => [(Axis::Z, 2), (Axis::Z, 0)],
            Self::Z3 => [(Axis::Z, 3), (Axis::Z, 0)],
            Self::XY => [(Axis::X, 1), (Axis::Y, 1)],
            Self::XY2 => [(Axis::X, 1), (Axis::Y, 2)],
            Self::XY3 => [(Axis::X, 1), (Axis::Y, 3)],
            Self::XZ => [(Axis::X, 1), (Axis::Z, 1)],
            Self::XZ2 => [(Axis::X, 1), (Axis::Z, 2)],
            Self::XZ3 => [(Axis::X, 1), (Axis::Z, 3)],
            Self::X2Y => [(Axis::X, 2), (Axis::Y, 1)],
            Self::X2Y3 => [(Axis::X, 2), (Axis::Y, 3)],
            Self::X2Z => [(Axis::X, 2), (Axis::Z, 1)],
            Self::X2Z3 => [(Axis::X, 2), (Axis::Z, 3)],
            Self::X3Y => [(Axis::X, 3), (Axis::Y, 1)],
            Self::X3Y3 => [(Axis::X, 3), (Axis::Y, 3)],
            Self::X3Z => [(Axis::X, 3), (Axis::Z, 1)],
            Self::X3Z3 => [(Axis::X, 3), (Axis::Z, 3)],
        }
    }

    const MULT_TABLE: [[Self; 24]; 24] = {
        let mut i = 0;
        let mut table = [[Self::Neutral; 24]; 24];

        while i < 24 {
            let left = Self::VARIANTS[i];
            let mut j = 0;

            while j < 24 {
                let mut new = left;
                let right = Self::VARIANTS[j].into_parts();

                let (axis, turns) = right[0];
                new = Self::simple_rotation(new, axis, turns);
                let (axis, turns) = right[1];
                new = Self::simple_rotation(new, axis, turns);

                table[i][j] = new;
                j += 1;
            }

            i += 1;
        }

        table
    };

    pub const fn compose(self, other: Self) -> Self {
        Self::MULT_TABLE[self.into_usize()][other.into_usize()]
    }

    const INVERSES: [Self; 24] = {
        let mut i = 0;
        let mut arr = [Self::Neutral; 24];
        while i < 24 {
            let a = Self::VARIANTS[i];
            let mut j = 0;

            while j < 24 {
                match a.compose(Self::VARIANTS[j]) {
                    Self::Neutral => {
                        arr[i] = Self::VARIANTS[j];
                        break;
                    }
                    _ => {}
                }
                j += 24;
            }

            i += 1;
        }

        arr
    };

    pub const fn inverse(self) -> Self {
        Self::INVERSES[self.into_usize()]
    }

    // const DIFFERENCES: [[Rotation; 24]; 24] = {
    // };

    /// Number of Rubiks' cube actions it would take to get from Neutral to that rotation.
    /// TODO: This is too simple for the edge cubelets for which we would need to know position and
    /// not just rotation.
    pub const fn len(&self) -> u8 {
        match self {
            Self::Neutral => 0,
            Self::X | Self::X2 | Self::X3 | Self::Y | Self::Y2 | Self::Y3 | Self::Z | Self::Z2 | Self::Z3 => 1,
            _ => 2,
        }
    }

    // /// Find the rotation that when right-multiplied with the left operand,
    // /// you obtain the right operand.
    // /// "How to get from rotation A to rotation B?"
    // /// A * x = B
    // /// x = A^-1 * B
    // /// These have been precomputed now
    // pub fn difference(self, other: Rotation) -> Self {
    //     todo!()
    // }

    /// Find the rotation of a cube from two facelets
    /// TODO: Assert that the facelets cannot be poles
    pub fn from_two_facelets(pair1: &FacePair, pair2: &FacePair) -> Option<Self> {
        for (face_pairs, rotation) in CUBELET_PAIRS {
            if face_pairs.contains(pair1) && face_pairs.contains(pair2) {
                return Some(rotation)
            }
        }
        None
    }
}

/// The faces of the Rubiks' cube
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    /// The negative X face
    Left,
    /// The positive X face
    Right,
    /// The negative Y face
    Front,
    /// The positive Y face
    Back,
    /// The negative Z face
    Down,
    /// The positive Z face
    Up,
}

/// The unique facelet colors
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facelet {
    /// The up face
    White,
    /// The front face
    Green,
    /// The left face
    Orange,
    /// The back face
    Blue,
    /// The right face
    Red,
    /// The down face
    Yellow
}

/// The symmetries of a cube.
/// Facelet colors are used to designate the positions of the faces.
/// TODO: add transformation to S4? [u8; 4]?
#[derive(Clone, Debug)]
pub struct Cubelet {
    up: Facelet,
    front: Facelet,
    left: Facelet,
    back: Facelet,
    right: Facelet,
    down: Facelet,
}

impl Default for Cubelet {
    fn default() -> Self {
        Self {
            up: Facelet::White,
            front: Facelet::Green,
            left: Facelet::Orange,
            back: Facelet::Blue,
            right: Facelet::Red,
            down: Facelet::Yellow,
        }
    }
}

#[derive(Debug)]
pub enum Direction { Clockwise, Counterclockwise }
type FacePair = (Face, Facelet);

/// Face-Facelet positions of a cube depending on its rotation
const CUBELET_PAIRS: [([FacePair; 6], Rotation); 24] = [
    ([ (Face::Up, Facelet::Green), (Face::Front, Facelet::Red), (Face::Left, Facelet::Yellow), (Face::Back, Facelet::Orange), (Face::Right, Facelet::White), (Face::Down, Facelet::Blue) ], Rotation::X3Z3),
    ([ (Face::Up, Facelet::Green), (Face::Front, Facelet::Orange), (Face::Left, Facelet::White), (Face::Back, Facelet::Red), (Face::Right, Facelet::Yellow), (Face::Down, Facelet::Blue) ], Rotation::X3Z),
    ([ (Face::Up, Facelet::Red), (Face::Front, Facelet::Yellow), (Face::Left, Facelet::Green), (Face::Back, Facelet::White), (Face::Right, Facelet::Blue), (Face::Down, Facelet::Orange) ], Rotation::X3Y3),
    ([ (Face::Up, Facelet::Orange), (Face::Front, Facelet::Yellow), (Face::Left, Facelet::Blue), (Face::Back, Facelet::White), (Face::Right, Facelet::Green), (Face::Down, Facelet::Red) ], Rotation::X3Y),
    ([ (Face::Up, Facelet::Yellow), (Face::Front, Facelet::Red), (Face::Left, Facelet::Blue), (Face::Back, Facelet::Orange), (Face::Right, Facelet::Green), (Face::Down, Facelet::White) ], Rotation::X2Z3),
    ([ (Face::Up, Facelet::Yellow), (Face::Front, Facelet::Orange), (Face::Left, Facelet::Green), (Face::Back, Facelet::Red), (Face::Right, Facelet::Blue), (Face::Down, Facelet::White) ], Rotation::X2Z),
    ([ (Face::Up, Facelet::Red), (Face::Front, Facelet::Blue), (Face::Left, Facelet::Yellow), (Face::Back, Facelet::Green), (Face::Right, Facelet::White), (Face::Down, Facelet::Orange) ], Rotation::X2Y3),
    ([ (Face::Up, Facelet::Orange), (Face::Front, Facelet::Blue), (Face::Left, Facelet::White), (Face::Back, Facelet::Green), (Face::Right, Facelet::Yellow), (Face::Down, Facelet::Red) ], Rotation::X2Y),
    ([ (Face::Up, Facelet::Blue), (Face::Front, Facelet::Red), (Face::Left, Facelet::White), (Face::Back, Facelet::Orange), (Face::Right, Facelet::Yellow), (Face::Down, Facelet::Green) ], Rotation::XZ3),
    ([ (Face::Up, Facelet::Blue), (Face::Front, Facelet::Yellow), (Face::Left, Facelet::Red), (Face::Back, Facelet::White), (Face::Right, Facelet::Orange), (Face::Down, Facelet::Green) ], Rotation::XZ2),
    ([ (Face::Up, Facelet::Blue), (Face::Front, Facelet::Orange), (Face::Left, Facelet::Yellow), (Face::Back, Facelet::Red), (Face::Right, Facelet::White), (Face::Down, Facelet::Green) ], Rotation::XZ),
    ([ (Face::Up, Facelet::Red), (Face::Front, Facelet::White), (Face::Left, Facelet::Blue), (Face::Back, Facelet::Yellow), (Face::Right, Facelet::Green), (Face::Down, Facelet::Orange) ], Rotation::XY3),
    ([ (Face::Up, Facelet::Green), (Face::Front, Facelet::White), (Face::Left, Facelet::Red), (Face::Back, Facelet::Yellow), (Face::Right, Facelet::Orange), (Face::Down, Facelet::Blue) ], Rotation::XY2),
    ([ (Face::Up, Facelet::Orange), (Face::Front, Facelet::White), (Face::Left, Facelet::Green), (Face::Back, Facelet::Yellow), (Face::Right, Facelet::Blue), (Face::Down, Facelet::Red) ], Rotation::XY),
    ([ (Face::Up, Facelet::White), (Face::Front, Facelet::Red), (Face::Left, Facelet::Green), (Face::Back, Facelet::Orange), (Face::Right, Facelet::Blue), (Face::Down, Facelet::Yellow) ], Rotation::Z3),
    ([ (Face::Up, Facelet::White), (Face::Front, Facelet::Red), (Face::Left, Facelet::Green), (Face::Back, Facelet::Orange), (Face::Right, Facelet::Blue), (Face::Down, Facelet::Yellow) ], Rotation::Z2),
    ([ (Face::Up, Facelet::White), (Face::Front, Facelet::Orange), (Face::Left, Facelet::Blue), (Face::Back, Facelet::Red), (Face::Right, Facelet::Green), (Face::Down, Facelet::Yellow) ], Rotation::Z),
    ([ (Face::Up, Facelet::Red), (Face::Front, Facelet::Green), (Face::Left, Facelet::White), (Face::Back, Facelet::Blue), (Face::Right, Facelet::Yellow), (Face::Down, Facelet::Orange) ], Rotation::Y3),
    ([ (Face::Up, Facelet::Yellow), (Face::Front, Facelet::Green), (Face::Left, Facelet::Red), (Face::Back, Facelet::Blue), (Face::Right, Facelet::Orange), (Face::Down, Facelet::White) ], Rotation::Y2),
    ([ (Face::Up, Facelet::Orange), (Face::Front, Facelet::Green), (Face::Left, Facelet::Yellow), (Face::Back, Facelet::Blue), (Face::Right, Facelet::White), (Face::Down, Facelet::Red) ], Rotation::Y),
    ([ (Face::Up, Facelet::Green), (Face::Front, Facelet::Yellow), (Face::Left, Facelet::Orange), (Face::Back, Facelet::White), (Face::Right, Facelet::Red), (Face::Down, Facelet::Blue) ], Rotation::X3),
    ([ (Face::Up, Facelet::Yellow), (Face::Front, Facelet::Blue), (Face::Left, Facelet::Orange), (Face::Back, Facelet::Green), (Face::Right, Facelet::Red), (Face::Down, Facelet::White) ], Rotation::X2),
    ([ (Face::Up, Facelet::Blue), (Face::Front, Facelet::White), (Face::Left, Facelet::Orange), (Face::Back, Facelet::Yellow), (Face::Right, Facelet::Red), (Face::Down, Facelet::Green) ], Rotation::X),
    ([ (Face::Up, Facelet::White), (Face::Front, Facelet::Green), (Face::Left, Facelet::Orange), (Face::Back, Facelet::Blue), (Face::Right, Facelet::Red), (Face::Down, Facelet::Yellow) ], Rotation::Neutral),
];

impl Cubelet {
    /// Get a whole cubelet using two facelet pairs
    /// TODO: Assert that the facelets cannot be opposing sides
    pub fn from_two_facelets(pair1: FacePair, pair2: FacePair) -> Self {
        let mut ind = 0;
        let mut iter = CUBELET_PAIRS.iter().enumerate();
        while let Some((i, cubelet_pairs)) = iter.next() {
        // for (i, cubelet_pairs) in CUBELET_PAIRS.iter().enumerate() {
           if cubelet_pairs.0.contains(&pair1) && cubelet_pairs.0.contains(&pair2) {
                ind = i;
                break;
            }
        }

        Self {
            up:    CUBELET_PAIRS[ind].0[0].1,
            front: CUBELET_PAIRS[ind].0[1].1,
            left:  CUBELET_PAIRS[ind].0[2].1,
            back:  CUBELET_PAIRS[ind].0[3].1,
            right: CUBELET_PAIRS[ind].0[4].1,
            down:  CUBELET_PAIRS[ind].0[5].1,
        }
    }

    pub const fn compose(self, rot: Rotation) -> Self {
        match rot {
            Rotation::Neutral => Cubelet { up: self.up, front: self.front, left: self.left, back: self.back, right: self.right, down: self.down, },
            Rotation::X    => Cubelet { up: self.back, front: self.up, left: self.left, back: self.down, right: self.right, down: self.front, },
            Rotation::X2   => Cubelet { up: self.down, front: self.back, left: self.left, back: self.front, right: self.right, down: self.up, },
            Rotation::X3   => Cubelet { up: self.front, front: self.down, left: self.left, back: self.up, right: self.right, down: self.back, },
            Rotation::Y    => Cubelet { up: self.left, front: self.front, left: self.down, back: self.back, right: self.up, down: self.right, },
            Rotation::Y2   => Cubelet { up: self.down, front: self.front, left: self.right, back: self.back, right: self.left, down: self.up, },
            Rotation::Y3   => Cubelet { up: self.right, front: self.front, left: self.up, back: self.back, right: self.down, down: self.left, },
            Rotation::Z    => Cubelet { up: self.up, front: self.left, left: self.back, back: self.right, right: self.front, down: self.down, },
            Rotation::Z2   => Cubelet { up: self.up, front: self.right, left: self.front, back: self.left, right: self.back, down: self.down, },
            Rotation::Z3   => Cubelet { up: self.up, front: self.right, left: self.front, back: self.left, right: self.back, down: self.down, },
            Rotation::XY   => Cubelet { up: self.left, front: self.up, left: self.front, back: self.down, right: self.back, down: self.right, },
            Rotation::XY2  => Cubelet { up: self.front, front: self.up, left: self.right, back: self.down, right: self.left, down: self.back, },
            Rotation::XY3  => Cubelet { up: self.right, front: self.up, left: self.back, back: self.down, right: self.front, down: self.left, },
            Rotation::XZ   => Cubelet { up: self.back, front: self.left, left: self.down, back: self.right, right: self.up, down: self.front, },
            Rotation::XZ2  => Cubelet { up: self.back, front: self.down, left: self.right, back: self.up, right: self.left, down: self.front, },
            Rotation::XZ3  => Cubelet { up: self.back, front: self.right, left: self.up, back: self.left, right: self.down, down: self.front, },
            Rotation::X2Y  => Cubelet { up: self.left, front: self.back, left: self.up, back: self.front, right: self.down, down: self.right, },
            Rotation::X2Y3 => Cubelet { up: self.right, front: self.back, left: self.down, back: self.front, right: self.up, down: self.left, },
            Rotation::X2Z  => Cubelet { up: self.down, front: self.left, left: self.front, back: self.right, right: self.back, down: self.up, },
            Rotation::X2Z3 => Cubelet { up: self.down, front: self.right, left: self.back, back: self.left, right: self.front, down: self.up, },
            Rotation::X3Y  => Cubelet { up: self.left, front: self.down, left: self.back, back: self.up, right: self.front, down: self.right, },
            Rotation::X3Y3 => Cubelet { up: self.right, front: self.down, left: self.front, back: self.up, right: self.back, down: self.left, },
            Rotation::X3Z  => Cubelet { up: self.front, front: self.left, left: self.up, back: self.right, right: self.down, down: self.back, },
            Rotation::X3Z3 => Cubelet { up: self.front, front: self.right, left: self.down, back: self.left, right: self.up, down: self.back, },
        }
    }
}

