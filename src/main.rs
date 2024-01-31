#![allow(dead_code)]
#![allow(unused_imports)]


pub mod cube {
    use my_stuff::stack::OneTwo;
    // Before we go any farther, let us figure out the algebra of a single cube.
    // We know there are 24 unique rotations of a cube. Let's make sure we can determine
    // a cube's rotation based on its faces.

    /// The faces of the Rubiks' cube
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// The symmetries of a cube (facelet colors are used to distinguish faces of the cube)
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

    /// The three axes of rotation
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Axis { X, Y, Z }

    /// The act of rotating a cube; the generator for the symmetries of
    /// a cube or cubelets faces.
    #[derive(Debug, Clone)]
    pub struct Rotate(Axis, u8);

    /// The 24 possible rotations for a cube in their reduced form
    #[repr(u8)]
    #[derive(Clone, Copy, Debug)]
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

    impl From<Rotate> for Rotation {
        fn from(Rotate(axis, turns): Rotate) -> Self {
            match (axis, turns % 4) {
                (_, 0) => Rotation::Neutral,
                (Axis::X, 1) => Rotation::X,
                (Axis::X, 2) => Rotation::X2,
                (Axis::X, 3) => Rotation::X3,
                (Axis::Y, 1) => Rotation::Y,
                (Axis::Y, 2) => Rotation::Y2,
                (Axis::Y, 3) => Rotation::Y3,
                (Axis::Z, 1) => Rotation::Z,
                (Axis::Z, 2) => Rotation::Z2,
                (Axis::Z, 3) => Rotation::Z3,
                _ => { unreachable!() }
            }
        }
    }

    #[derive(Debug)]
    pub enum Direction { Clockwise, Counterclockwise }
    type FacePair = (Face, Facelet);

    impl Rotation {
        pub const VALUES: [Rotation; 24] = [Self::Neutral, Self::X, Self::X2, Self::X3, Self::Y, Self::Y2, Self::Y3, Self::Z, Self::Z2, Self::Z3, Self::XY, Self::XY2, Self::XY3, Self::XZ, Self::XZ2, Self::XZ3, Self::X2Y, Self::X2Y3, Self::X2Z, Self::X2Z3, Self::X3Y, Self::X3Y3, Self::X3Z, Self::X3Z3];

        pub fn compose(self, other: Rotation) -> Self {
            let generators: OneTwo<Rotate> = other.into();
            generators.into_iter().fold(self, |acc, el| acc.rotate(el))
        }

        pub fn into_generators(self) -> OneTwo<Rotate> {
            self.into()
        }

        pub fn inverse(self) -> Self {
            self.into_generators().into_iter().rev().fold(Self::Neutral, |acc, el| acc.rotate(el))
        }

        pub fn difference(self, other: Rotation) -> Self {
            self.inverse().compose(other)
        }

        pub fn rotate(self, Rotate(axis, turns): Rotate) -> Self {
            if turns == 0 {
                self
            } else {
                match (self, axis) {
                    (Self::Neutral, _) => { return Rotate(axis, turns).into(); }
                    (Self::X, Axis::X) => Self::X2,
                    (Self::X, Axis::Y) => Self::XY,
                    (Self::X, Axis::Z) => Self::XZ,
                    (Self::X2, Axis::X) => Self::X3,
                    (Self::X2, Axis::Y) => Self::X2Y,
                    (Self::X2, Axis::Z) => Self::X2Z,
                    (Self::X3, Axis::X) => Self::Neutral,
                    (Self::X3, Axis::Y) => Self::X3Y,
                    (Self::X3, Axis::Z) => Self::X3Z,
                    (Self::Y, Axis::X) => Self::XZ,
                    (Self::Y, Axis::Y) => Self::Y2,
                    (Self::Y, Axis::Z) => Self::X3Y,
                    (Self::Y2, Axis::X) => Self::XZ2,
                    (Self::Y2, Axis::Y) => Self::Y3,
                    (Self::Y2, Axis::Z) => Self::X2Z3,
                    (Self::Y3, Axis::X) => Self::XZ3,
                    (Self::Y3, Axis::Y) => Self::Neutral,
                    (Self::Y3, Axis::Z) => Self::XY3,
                    (Self::Z, Axis::X) => Self::XY3,
                    (Self::Z, Axis::Y) => Self::XZ,
                    (Self::Z, Axis::Z) => Self::Z2,
                    (Self::Z2, Axis::X) => Self::XY2,
                    (Self::Z2, Axis::Y) => Self::X2Y3,
                    (Self::Z2, Axis::Z) => Self::Z3,
                    (Self::Z3, Axis::X) => Self::XY,
                    (Self::Z3, Axis::Y) => Self::XZ3,
                    (Self::Z3, Axis::Z) => Self::Neutral,
                    (Self::XY, Axis::X) => Self::X2Z,
                    (Self::XY, Axis::Y) => Self::XY2,
                    (Self::XY, Axis::Z) => Self::Y,
                    (Self::XY2, Axis::X) => Self::Y2,
                    (Self::XY2, Axis::Y) => Self::XY3,
                    (Self::XY2, Axis::Z) => Self::X3Z3,
                    (Self::XY3, Axis::X) => Self::X2Z3,
                    (Self::XY3, Axis::Y) => Self::X,
                    (Self::XY3, Axis::Z) => Self::X2Y3,
                    (Self::XZ, Axis::X) => Self::X2Y3,
                    (Self::XZ, Axis::Y) => Self::X2Z,
                    (Self::XZ, Axis::Z) => Self::XZ2,
                    (Self::XZ2, Axis::X) => Self::Z2,
                    (Self::XZ2, Axis::Y) => Self::X3Y3,
                    (Self::XZ2, Axis::Z) => Self::XZ3,
                    (Self::XZ3, Axis::X) => Self::X2Y,
                    (Self::XZ3, Axis::Y) => Self::X2Z3,
                    (Self::XZ3, Axis::Z) => Self::X,
                    (Self::X2Y, Axis::X) => Self::X3Z,
                    (Self::X2Y, Axis::Y) => Self::Z2,
                    (Self::X2Y, Axis::Z) => Self::XY,
                    (Self::X2Y3, Axis::X) => Self::X3Z3,
                    (Self::X2Y3, Axis::Y) => Self::X2,
                    (Self::X2Y3, Axis::Z) => Self::X3Y3,
                    (Self::X2Z, Axis::X) => Self::X3Y3,
                    (Self::X2Z, Axis::Y) => Self::X3Z,
                    (Self::X2Z, Axis::Z) => Self::Y2,
                    (Self::X2Z3, Axis::X) => Self::X3Y,
                    (Self::X2Z3, Axis::Y) => Self::X3Z3,
                    (Self::X2Z3, Axis::Z) => Self::X2,
                    (Self::X3Y, Axis::X) => Self::Z,
                    (Self::X3Y, Axis::Y) => Self::XZ2,
                    (Self::X3Y, Axis::Z) => Self::X2Y,
                    (Self::X3Y3, Axis::X) => Self::Z3,
                    (Self::X3Y3, Axis::Y) => Self::X3,
                    (Self::X3Y3, Axis::Z) => Self::Y3,
                    (Self::X3Z, Axis::X) => Self::Y3,
                    (Self::X3Z, Axis::Y) => Self::Z,
                    (Self::X3Z, Axis::Z) => Self::XY2,
                    (Self::X3Z3, Axis::X) => Self::Y,
                    (Self::X3Z3, Axis::Y) => Self::Z3,
                    (Self::X3Z3, Axis::Z) => Self::X3,
                }
                .rotate(Rotate(axis, turns - 1))
            }
        }

        /// Find the rotation of a cube from two facelets
        pub fn from_two_facelets(pair1: &FacePair, pair2: &FacePair) -> Option<Self> {
            for (face_pairs, rotation) in CUBELET_PAIRS {
                if face_pairs.contains(pair1) && face_pairs.contains(pair2) {
                    return Some(rotation)
                }
            }
            None
        }
    }

    impl From<Rotation> for OneTwo<Rotate> {
        fn from(value: Rotation) -> Self {
            match value {
                Rotation::Neutral => OneTwo::One(Rotate(Axis::X, 0)),
                Rotation::X    => OneTwo::One(Rotate(Axis::X, 1)),
                Rotation::X2   => OneTwo::One(Rotate(Axis::X, 2)),
                Rotation::X3   => OneTwo::One(Rotate(Axis::X, 3)),
                Rotation::Y    => OneTwo::One(Rotate(Axis::Y, 1)),
                Rotation::Y2   => OneTwo::One(Rotate(Axis::Y, 2)),
                Rotation::Y3   => OneTwo::One(Rotate(Axis::Y, 3)),
                Rotation::Z    => OneTwo::One(Rotate(Axis::Z, 1)),
                Rotation::Z2   => OneTwo::One(Rotate(Axis::Z, 2)),
                Rotation::Z3   => OneTwo::One(Rotate(Axis::Z, 3)),
                Rotation::XY   => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Y, 1)),
                Rotation::XY2  => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Y, 2)),
                Rotation::XY3  => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Y, 3)),
                Rotation::XZ   => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Z, 1)),
                Rotation::XZ2  => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Z, 2)),
                Rotation::XZ3  => OneTwo::Two(Rotate(Axis::X, 1), Rotate(Axis::Z, 3)),
                Rotation::X2Y  => OneTwo::Two(Rotate(Axis::X, 2), Rotate(Axis::Y, 1)),
                Rotation::X2Y3 => OneTwo::Two(Rotate(Axis::X, 2), Rotate(Axis::Y, 3)),
                Rotation::X2Z  => OneTwo::Two(Rotate(Axis::X, 2), Rotate(Axis::Z, 1)),
                Rotation::X2Z3 => OneTwo::Two(Rotate(Axis::X, 2), Rotate(Axis::Z, 3)),
                Rotation::X3Y  => OneTwo::Two(Rotate(Axis::X, 3), Rotate(Axis::Y, 1)),
                Rotation::X3Y3 => OneTwo::Two(Rotate(Axis::X, 3), Rotate(Axis::Y, 3)),
                Rotation::X3Z  => OneTwo::Two(Rotate(Axis::X, 3), Rotate(Axis::Z, 1)),
                Rotation::X3Z3 => OneTwo::Two(Rotate(Axis::X, 3), Rotate(Axis::Z, 3)),
            }
        }
    }

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
        pub fn rotate(self, Rotate(axis, turns): Rotate) -> Self {
            if turns == 0 {
                self
            } else {
                match axis {
                    Axis::X => Cubelet {
                        up: self.back,
                        front: self.up,
                        left: self.left,
                        back: self.down,
                        right: self.right,
                        down: self.front,
                    },
                    Axis::Y => Cubelet {
                        up: self.left,
                        front: self.front,
                        left: self.down,
                        back: self.back,
                        right: self.up,
                        down: self.right,
                    },
                    Axis::Z => Cubelet {
                        up: self.up,
                        front: self.left,
                        left: self.back,
                        back: self.right,
                        right: self.front,
                        down: self.down,
                    },
                }.rotate(Rotate(axis, turns - 1))
            }
        }
    }
}

pub mod rubiks {
    use crate::cube::{Axis, Rotation, Cubelet, Facelet};

    /// My own way of representing the arrangement of a Rubiks' cube
    /// relying on minimum number of moves from the solved arrangement.
    /// Hopefully, we can reduce the search space by using it to easily
    /// identify isomorphic arrangements.
    #[derive(Debug)]
    pub struct MovesArrangement {
        sequence: Vec<Move>,
        as_cubelets: CubeletsArrangement,
    }

    /// Number of turns on the most negative face, number of turns on the most positive face,
    /// and the axis on which the turns happen
    #[derive(Debug)]
    pub struct Move(u8, u8, Axis);

    /// A Rubiks' cube arrangement, represented by the rotation of
    /// the cubelets relative to the solved arrangement
    #[derive(Debug)]
    pub struct CubeletsArrangement {
        cubelets: [[[Rotation; 3]; 3] ;3]
    }

    /// A Rubiks' cube arrangement, represented by the position of the facelets
    #[derive(Debug)]
    pub struct FaceletsArrangement {
        facelets: [[[Facelet; 6]; 3]; 3]
    }
}

use std::io::Write;
use cube::{Cubelet, Rotation};

fn main() -> Result<(), std::io::Error> {
    let cubelet = Cubelet::default();
    for rotation in Rotation::VALUES.iter() {
        println!("{:?}: {:?}", rotation, rotation.into_generators().into_iter().fold(cubelet.clone(), |acc, el| acc.rotate(el)));
    }
    Ok(())
}

