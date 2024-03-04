use std::{fmt::Display, array};

use crate::cube::{Axis, Rotation};

// My own way of representing the arrangement of a Rubiks' cube
// relying on minimum number of moves from the solved arrangement.
// Hopefully, we can reduce the search space by using it to easily
// identify isomorphic arrangements.
// TODO: This is significantly complicated. I think I will need
// some sort of adjacency matrix that keeps track of which faces,
// axes, and directions are distinct at any point in the move
// sequence.
// #[derive(Debug)]
// pub struct MovesArrangement {
//     sequence: Vec<Move>,
//     as_cubelets: Cube,
// }

/// Number of turns on the most negative face, number of turns on the most positive face,
/// and the axis on which the turns happen
#[derive(Clone, Copy, Debug)]
pub struct Move(pub u8, pub u8, pub Axis);

impl Move {
    pub const ALL: [Move; 45] = {
        let mut i = 0;
        let mut res = [Move(0, 0, Axis::X); 45];
        while i < 4 {
            let mut j = 0;
            while j < 4 {
                if !(i == 0 && j == 0) {
                    res[i*12 + j*3 - 3] = Move(i as u8, j as u8, Axis::X);
                    res[i*12 + j*3 - 2] = Move(i as u8, j as u8, Axis::Y);
                    res[i*12 + j*3 - 1] = Move(i as u8, j as u8, Axis::Z);
                }
                j += 1;
            }
            i += 1;
        }
        res
    };

    pub const X: [Move; 15] = {
        let mut i = 0;
        let mut res = [Move(0, 0, Axis::X); 15];
        while i < 4 {
            let mut j = 0;
            while j < 4 {
                if !(i == 0 && j == 0) {
                    res[i*4 + j - 1] = Move(i as u8, j as u8, Axis::X);
                }
                j += 1;
            }
            i += 1;
        }
        res
    };

    pub const Y: [Move; 15] = {
        let mut i = 0;
        let mut res = [Move(0, 0, Axis::Y); 15];
        while i < 4 {
            let mut j = 0;
            while j < 4 {
                if !(i == 0 && j == 0) {
                    res[i*4 + j - 1] = Move(i as u8, j as u8, Axis::Y);
                }
                j += 1;
            }
            i += 1;
        }
        res
    };

    pub const Z: [Move; 15] = {
        let mut i = 0;
        let mut res = [Move(0, 0, Axis::Z); 15];
        while i < 4 {
            let mut j = 0;
            while j < 4 {
                if !(i == 0 && j == 0) {
                    res[i*4 + j - 1] = Move(i as u8, j as u8, Axis::Z);
                }
                j += 1;
            }
            i += 1;
        }
        res
    };
}

pub const fn index<const X: usize, const Y: usize, const Z: usize>() -> usize {
    assert!(X < 3);
    assert!(Y < 3);
    assert!(Z < 3);

    Z + 3 * Y + 9 * X
    - (X==0 && Z==2 && Y==1 || Y==2 || X>0) as usize
    - (X==1 && (Z==2 || Y==2) || X==2) as usize
    - (X==1 && Y==2 || X==2) as usize * 3
    - (X==1 && Y==2 && Z==2 || X==2) as usize
    - (X==2 && (Z==2 && Y==1 || Y==2 || X>0)) as usize
}

/// A Rubiks' cube arrangement, represented by the rotation of
/// the cubelets relative to the solved arrangement. Each cubelet
/// is represented in the place where it is currently. Face centers
/// and the middle-middle-middle piece are never changed from Rotation::Neutral
/// This is probably the most practical memory layout.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cube {
    pub cubelets: [Rotation; 20]
}

pub struct CubeletsIter {
    inner: array::IntoIter<Rotation, 20>
}

impl Iterator for CubeletsIter {
    type Item = Rotation;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = &self.cubelets;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[index::<0,2,2>()], c[index::<0,1,2>()], c[index::<0,0,2>()], c[index::<1,0,2>()], c[index::<2,0,2>()], c[index::<2,1,2>()], c[index::<2,2,2>()], c[index::<1,2,2>()],
        )?;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[index::<0,2,1>()], "O", c[index::<0,0,1>()], "G", c[index::<2,0,1>()], "R", c[index::<2,2,1>()], "B",
        )?;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[index::<0,2,0>()], c[index::<0,1,0>()], c[index::<0,0,0>()], c[index::<1,0,0>()], c[index::<2,0,0>()], c[index::<2,1,0>()], c[index::<2,2,0>()], c[index::<1,2,0>()],
        )?;
        Ok(())
    }
}

fn turn_face_x<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<FACE,2,2>()], c[index::<FACE,0,2>()], c[index::<FACE,0,0>()], c[index::<FACE,2,0>()]) =
        (c[index::<FACE,2,0>()].compose(rot), c[index::<FACE,2,2>()].compose(rot), c[index::<FACE,0,2>()].compose(rot), c[index::<FACE,0,0>()].compose(rot));
    // edges
    (c[index::<FACE,1,2>()], c[index::<FACE,0,1>()], c[index::<FACE,1,0>()], c[index::<FACE,2,1>()]) =
        (c[index::<FACE,2,1>()].compose(rot), c[index::<FACE,1,2>()].compose(rot), c[index::<FACE,0,1>()].compose(rot), c[index::<FACE,1,0>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_x2<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<FACE,2,2>()], c[index::<FACE,0,2>()], c[index::<FACE,0,0>()], c[index::<FACE,2,0>()]) =
        (c[index::<FACE,0,0>()].compose(rot), c[index::<FACE,2,0>()].compose(rot), c[index::<FACE,2,2>()].compose(rot), c[index::<FACE,0,2>()].compose(rot));
    // edges
    (c[index::<FACE,1,2>()], c[index::<FACE,0,1>()], c[index::<FACE,1,0>()], c[index::<FACE,2,1>()]) =
        (c[index::<FACE,1,0>()].compose(rot), c[index::<FACE,2,1>()].compose(rot), c[index::<FACE,1,2>()].compose(rot), c[index::<FACE,0,1>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_x3<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<FACE,2,2>()], c[index::<FACE,0,2>()], c[index::<FACE,0,0>()], c[index::<FACE,2,0>()]) =
        (c[index::<FACE,0,2>()].compose(rot), c[index::<FACE,0,0>()].compose(rot), c[index::<FACE,2,0>()].compose(rot), c[index::<FACE,2,2>()].compose(rot));
    // edges
    (c[index::<FACE,1,2>()], c[index::<FACE,0,1>()], c[index::<FACE,1,0>()], c[index::<FACE,2,1>()]) =
        (c[index::<FACE,0,1>()].compose(rot), c[index::<FACE,1,2>()].compose(rot), c[index::<FACE,2,1>()].compose(rot), c[index::<FACE,1,0>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_y<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,FACE,2>()], c[index::<0,FACE,2>()], c[index::<0,FACE,0>()], c[index::<2,FACE,0>()]) =
        (c[index::<0,FACE,2>()].compose(rot), c[index::<0,FACE,0>()].compose(rot), c[index::<2,FACE,0>()].compose(rot), c[index::<2,FACE,2>()].compose(rot));
    // edges
    (c[index::<1,FACE,2>()], c[index::<0,FACE,1>()], c[index::<1,FACE,0>()], c[index::<2,FACE,1>()]) =
        (c[index::<0,FACE,1>()].compose(rot), c[index::<1,FACE,0>()].compose(rot), c[index::<2,FACE,1>()].compose(rot), c[index::<1,FACE,2>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_y2<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,FACE,2>()], c[index::<0,FACE,2>()], c[index::<0,FACE,0>()], c[index::<2,FACE,0>()]) =
        (c[index::<0,FACE,0>()].compose(rot), c[index::<2,FACE,0>()].compose(rot), c[index::<2,FACE,2>()].compose(rot), c[index::<0,FACE,2>()].compose(rot));
    // edges
    (c[index::<1,FACE,2>()], c[index::<0,FACE,1>()], c[index::<1,FACE,0>()], c[index::<2,FACE,1>()]) =
        (c[index::<1,FACE,0>()].compose(rot), c[index::<2,FACE,1>()].compose(rot), c[index::<1,FACE,2>()].compose(rot), c[index::<0,FACE,1>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_y3<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,FACE,2>()], c[index::<0,FACE,2>()], c[index::<0,FACE,0>()], c[index::<2,FACE,0>()]) =
        (c[index::<2,FACE,0>()].compose(rot), c[index::<2,FACE,2>()].compose(rot), c[index::<0,FACE,2>()].compose(rot), c[index::<0,FACE,0>()].compose(rot));
    // edges
    (c[index::<1,FACE,2>()], c[index::<0,FACE,1>()], c[index::<1,FACE,0>()], c[index::<2,FACE,1>()]) =
        (c[index::<2,FACE,1>()].compose(rot), c[index::<1,FACE,2>()].compose(rot), c[index::<0,FACE,1>()].compose(rot), c[index::<1,FACE,0>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_z<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,2,FACE>()], c[index::<0,2,FACE>()], c[index::<0,0,FACE>()], c[index::<2,0,FACE>()]) =
        (c[index::<2,0,FACE>()].compose(rot), c[index::<2,2,FACE>()].compose(rot), c[index::<0,2,FACE>()].compose(rot), c[index::<0,0,FACE>()].compose(rot));
    // edges
    (c[index::<1,2,FACE>()], c[index::<0,1,FACE>()], c[index::<1,0,FACE>()], c[index::<2,1,FACE>()]) =
        (c[index::<2,1,FACE>()].compose(rot), c[index::<1,2,FACE>()].compose(rot), c[index::<0,1,FACE>()].compose(rot), c[index::<1,0,FACE>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_z2<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,2,FACE>()], c[index::<0,2,FACE>()], c[index::<0,0,FACE>()], c[index::<2,0,FACE>()]) =
        (c[index::<0,0,FACE>()].compose(rot), c[index::<2,0,FACE>()].compose(rot), c[index::<2,2,FACE>()].compose(rot), c[index::<0,2,FACE>()].compose(rot));
    // edges
    (c[index::<1,2,FACE>()], c[index::<0,1,FACE>()], c[index::<1,0,FACE>()], c[index::<2,1,FACE>()]) =
        (c[index::<1,0,FACE>()].compose(rot), c[index::<2,1,FACE>()].compose(rot), c[index::<1,2,FACE>()].compose(rot), c[index::<0,1,FACE>()].compose(rot));
    Cube { cubelets: c }
}

fn turn_face_z3<const FACE: usize>(cube: Cube, rot: Rotation) -> Cube {
    let mut c = cube.cubelets;
    // corners
    (c[index::<2,2,FACE>()], c[index::<0,2,FACE>()], c[index::<0,0,FACE>()], c[index::<2,0,FACE>()]) =
        (c[index::<0,2,FACE>()].compose(rot), c[index::<0,0,FACE>()].compose(rot), c[index::<2,0,FACE>()].compose(rot), c[index::<2,2,FACE>()].compose(rot));
    // edges
    (c[index::<1,2,FACE>()], c[index::<0,1,FACE>()], c[index::<1,0,FACE>()], c[index::<2,1,FACE>()]) =
        (c[index::<0,1,FACE>()].compose(rot), c[index::<1,0,FACE>()].compose(rot), c[index::<2,1,FACE>()].compose(rot), c[index::<1,2,FACE>()].compose(rot));
    Cube { cubelets: c }
}

// Using function pointers instead of a 9-arm match statement shaves about a second per 10 million
// cubes searched.
static TURN_CLOSE_FACES: [fn(Cube, Rotation) -> Cube; 9] = [
    turn_face_x::<0>, turn_face_x2::<0>, turn_face_x3::<0>,
    turn_face_y::<0>, turn_face_y2::<0>, turn_face_y3::<0>,
    turn_face_z::<0>, turn_face_z2::<0>, turn_face_z3::<0>,
];

static TURN_FAR_FACES: [fn(Cube, Rotation) -> Cube; 9] = [
    turn_face_x::<2>, turn_face_x2::<2>, turn_face_x3::<2>,
    turn_face_y::<2>, turn_face_y2::<2>, turn_face_y3::<2>,
    turn_face_z::<2>, turn_face_z2::<2>, turn_face_z3::<2>,
];

impl Cube {
    fn turn_face<const FACE: usize>(self, rot: Rotation) -> Self {
        if rot == Rotation::Neutral { self } else {
            debug_assert!(FACE == 0 || FACE == 2, "{FACE}");
            let ind = (rot as u8) as usize - 1;
            debug_assert!(ind < 9, "{ind}");

            let f = if FACE == 0 {
                TURN_CLOSE_FACES[ind]
            } else {
                TURN_FAR_FACES[ind]
            };

            f(self, rot)
        }
    }

    pub fn make_move(self, Move(rot1, rot2, axis): Move) -> Self {
        let rot1: Rotation = (rot1, axis).into();
        let rot2: Rotation = (rot2, axis).into();

        self.turn_face::<0>(rot1).turn_face::<2>(rot2)
    }

    pub fn iter(&self) -> CubeletsIter {
        CubeletsIter { inner: self.cubelets.into_iter() }
    }

    pub fn parity(&self) -> u8 {
        self.iter().map(|r| r.len()).sum()
    }
}

