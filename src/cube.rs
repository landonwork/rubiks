use std::{
    array, 
    fmt::Display, 
    marker::PhantomData,
};

use crate::{
    action::Move,
    cubelet::{Axis, Rotation}
};

/// A Rubiks' cube's state, represented by the orientation of the cubelets.
/// Cubelets are ordered by position or ID, meaning ordered by beginning position.
/// You can see this in the `coords` and `index` functions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Cube<T: SortBy> {
    pub cubelets: [Rotation; 20],
    _phantom: PhantomData<T>
}

pub(crate) trait SortBy {}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Position;
impl SortBy for Position {}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Id;
impl SortBy for Id {}

impl<T: SortBy> Display for Cube<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rot in self.cubelets.iter() {
            write!(f, "{}", ((*rot as u8) + b'A') as char)?;
        }
        Ok(())
    }
}

impl<T: SortBy> IntoIterator for Cube<T> {
    type IntoIter = array::IntoIter<Rotation, 20>;
    type Item = Rotation;
    fn into_iter(self) -> Self::IntoIter {
        self.cubelets.into_iter()
    }
}

#[allow(non_snake_case)]
#[inline]
pub const fn index([X, Y, Z]: [u8; 3]) -> usize {
    debug_assert!(X < 3);
    debug_assert!(Y < 3);
    debug_assert!(Z < 3);
    debug_assert!(!(X==1 && Y==1) && !(X==1 && Z==1) && !(Y==1 && Z==1));

    Z as usize + 3 * Y as usize + 9 * X as usize
    - (X==0 && Z==2 && Y==1 || Y==2 || X>0) as usize
    - (X==1 && (Z==2 || Y==2) || X==2) as usize
    - (X==1 && Y==2 || X==2) as usize * 3
    - (X==1 && Y==2 && Z==2 || X==2) as usize
    - (X==2 && (Z==2 && Y==1 || Y==2)) as usize
}

#[inline]
pub const fn coords(index: usize) -> [u8; 3] {
    const COORDS: [[u8; 3]; 20] = [
        [0, 0, 0],
        [0, 0, 1],
        [0, 0, 2],
        [0, 1, 0],
        [0, 1, 2],
        [0, 2, 0],
        [0, 2, 1],
        [0, 2, 2],
        [1, 0, 0],
        [1, 0, 2],
        [1, 2, 0],
        [1, 2, 2],
        [2, 0, 0],
        [2, 0, 1],
        [2, 0, 2],
        [2, 1, 0],
        [2, 1, 2],
        [2, 2, 0],
        [2, 2, 1],
        [2, 2, 2],
    ];
    COORDS[index]
}

#[allow(non_snake_case)]
#[inline]
const fn shift_coords_simple(coords: [u8; 3], (axis, turns): (Axis, u8)) -> [u8; 3] {
    let [X, Y, Z] = coords;
    match (axis, turns) {
        (_, 0) => coords,
        (Axis::X, n) => shift_coords_simple([X, Z, 2 - Y], (axis, n - 1)),
        (Axis::Y, n) => shift_coords_simple([Z, Y, 2 - X], (axis, n - 1)),
        (Axis::Z, n) => shift_coords_simple([Y, 2 - X, Z], (axis, n - 1)),
    }
}

#[inline]
const fn shift_coords(coords: [u8; 3], rot: Rotation) -> [u8; 3] {
    let [rot1, rot2] = rot.into_parts();
    shift_coords_simple(shift_coords_simple(coords, rot1), rot2)
}

fn turn_face_x<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([FACE,2,2])], c[index([FACE,0,2])], c[index([FACE,0,0])], c[index([FACE,2,0])]) =
        (c[index([FACE,2,0])].compose(rot), c[index([FACE,2,2])].compose(rot), c[index([FACE,0,2])].compose(rot), c[index([FACE,0,0])].compose(rot));
    // edges
    (c[index([FACE,1,2])], c[index([FACE,0,1])], c[index([FACE,1,0])], c[index([FACE,2,1])]) =
        (c[index([FACE,2,1])].compose(rot), c[index([FACE,1,2])].compose(rot), c[index([FACE,0,1])].compose(rot), c[index([FACE,1,0])].compose(rot));
    Cube::new(c)
}

fn turn_face_x2<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([FACE,2,2])], c[index([FACE,0,2])], c[index([FACE,0,0])], c[index([FACE,2,0])]) =
        (c[index([FACE,0,0])].compose(rot), c[index([FACE,2,0])].compose(rot), c[index([FACE,2,2])].compose(rot), c[index([FACE,0,2])].compose(rot));
    // edges
    (c[index([FACE,1,2])], c[index([FACE,0,1])], c[index([FACE,1,0])], c[index([FACE,2,1])]) =
        (c[index([FACE,1,0])].compose(rot), c[index([FACE,2,1])].compose(rot), c[index([FACE,1,2])].compose(rot), c[index([FACE,0,1])].compose(rot));
    Cube::new(c)
}

fn turn_face_x3<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([FACE,2,2])], c[index([FACE,0,2])], c[index([FACE,0,0])], c[index([FACE,2,0])]) =
        (c[index([FACE,0,2])].compose(rot), c[index([FACE,0,0])].compose(rot), c[index([FACE,2,0])].compose(rot), c[index([FACE,2,2])].compose(rot));
    // edges
    (c[index([FACE,1,2])], c[index([FACE,0,1])], c[index([FACE,1,0])], c[index([FACE,2,1])]) =
        (c[index([FACE,0,1])].compose(rot), c[index([FACE,1,0])].compose(rot), c[index([FACE,2,1])].compose(rot), c[index([FACE,1,2])].compose(rot));
    Cube::new(c)
}

fn turn_face_y<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,FACE,2])], c[index([0,FACE,2])], c[index([0,FACE,0])], c[index([2,FACE,0])]) =
        (c[index([0,FACE,2])].compose(rot), c[index([0,FACE,0])].compose(rot), c[index([2,FACE,0])].compose(rot), c[index([2,FACE,2])].compose(rot));
    // edges
    (c[index([1,FACE,2])], c[index([0,FACE,1])], c[index([1,FACE,0])], c[index([2,FACE,1])]) =
        (c[index([0,FACE,1])].compose(rot), c[index([1,FACE,0])].compose(rot), c[index([2,FACE,1])].compose(rot), c[index([1,FACE,2])].compose(rot));
    Cube::new(c)
}

fn turn_face_y2<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,FACE,2])], c[index([0,FACE,2])], c[index([0,FACE,0])], c[index([2,FACE,0])]) =
        (c[index([0,FACE,0])].compose(rot), c[index([2,FACE,0])].compose(rot), c[index([2,FACE,2])].compose(rot), c[index([0,FACE,2])].compose(rot));
    // edges
    (c[index([1,FACE,2])], c[index([0,FACE,1])], c[index([1,FACE,0])], c[index([2,FACE,1])]) =
        (c[index([1,FACE,0])].compose(rot), c[index([2,FACE,1])].compose(rot), c[index([1,FACE,2])].compose(rot), c[index([0,FACE,1])].compose(rot));
    Cube::new(c)
}

fn turn_face_y3<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,FACE,2])], c[index([0,FACE,2])], c[index([0,FACE,0])], c[index([2,FACE,0])]) =
        (c[index([2,FACE,0])].compose(rot), c[index([2,FACE,2])].compose(rot), c[index([0,FACE,2])].compose(rot), c[index([0,FACE,0])].compose(rot));
    // edges
    (c[index([1,FACE,2])], c[index([0,FACE,1])], c[index([1,FACE,0])], c[index([2,FACE,1])]) =
        (c[index([2,FACE,1])].compose(rot), c[index([1,FACE,2])].compose(rot), c[index([0,FACE,1])].compose(rot), c[index([1,FACE,0])].compose(rot));
    Cube::new(c)
}

fn turn_face_z<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,2,FACE])], c[index([0,2,FACE])], c[index([0,0,FACE])], c[index([2,0,FACE])]) =
        (c[index([2,0,FACE])].compose(rot), c[index([2,2,FACE])].compose(rot), c[index([0,2,FACE])].compose(rot), c[index([0,0,FACE])].compose(rot));
    // edges
    (c[index([1,2,FACE])], c[index([0,1,FACE])], c[index([1,0,FACE])], c[index([2,1,FACE])]) =
        (c[index([2,1,FACE])].compose(rot), c[index([1,2,FACE])].compose(rot), c[index([0,1,FACE])].compose(rot), c[index([1,0,FACE])].compose(rot));
    Cube::new(c)
}

fn turn_face_z2<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,2,FACE])], c[index([0,2,FACE])], c[index([0,0,FACE])], c[index([2,0,FACE])]) =
        (c[index([0,0,FACE])].compose(rot), c[index([2,0,FACE])].compose(rot), c[index([2,2,FACE])].compose(rot), c[index([0,2,FACE])].compose(rot));
    // edges
    (c[index([1,2,FACE])], c[index([0,1,FACE])], c[index([1,0,FACE])], c[index([2,1,FACE])]) =
        (c[index([1,0,FACE])].compose(rot), c[index([2,1,FACE])].compose(rot), c[index([1,2,FACE])].compose(rot), c[index([0,1,FACE])].compose(rot));
    Cube::new(c)
}

fn turn_face_z3<const FACE: u8>(cube: Cube<Position>, rot: Rotation) -> Cube<Position> {
    let mut c = cube.cubelets;
    // corners
    (c[index([2,2,FACE])], c[index([0,2,FACE])], c[index([0,0,FACE])], c[index([2,0,FACE])]) =
        (c[index([0,2,FACE])].compose(rot), c[index([0,0,FACE])].compose(rot), c[index([2,0,FACE])].compose(rot), c[index([2,2,FACE])].compose(rot));
    // edges
    (c[index([1,2,FACE])], c[index([0,1,FACE])], c[index([1,0,FACE])], c[index([2,1,FACE])]) =
        (c[index([0,1,FACE])].compose(rot), c[index([1,0,FACE])].compose(rot), c[index([2,1,FACE])].compose(rot), c[index([1,2,FACE])].compose(rot));
    Cube::new(c)
}

// Using function pointers in an array instead of a 9-arm match statement
// Is it faster? Idk.
static TURN_CLOSE_FACES: [fn(Cube<Position>, Rotation) -> Cube<Position>; 9] = [
    turn_face_x::<0>, turn_face_x2::<0>, turn_face_x3::<0>,
    turn_face_y::<0>, turn_face_y2::<0>, turn_face_y3::<0>,
    turn_face_z::<0>, turn_face_z2::<0>, turn_face_z3::<0>,
];

static TURN_FAR_FACES: [fn(Cube<Position>, Rotation) -> Cube<Position>; 9] = [
    turn_face_x::<2>, turn_face_x2::<2>, turn_face_x3::<2>,
    turn_face_y::<2>, turn_face_y2::<2>, turn_face_y3::<2>,
    turn_face_z::<2>, turn_face_z2::<2>, turn_face_z3::<2>,
];

pub fn shift_forward(cubelets: &[Rotation; 20], mutations: &[Rotation; 20]) -> [Rotation; 20] {
    mutations.into_iter()
        .enumerate()
        .map(|(i, rot)| index(shift_coords(coords(i), *rot)))
        .map(|ind| cubelets[ind])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn shift_backward(cubelets: &[Rotation; 20], mutations: &[Rotation; 20]) -> [Rotation; 20] {
    mutations.into_iter()
        .enumerate()
        .map(|(i, rot)| index(shift_coords(coords(i), rot.inverse())))
        .map(|ind| cubelets[ind])
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

impl Cube<Position> {
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

    pub fn make_move(self, Move(axis, rot1, rot2): Move) -> Self {
        let rot1: Rotation = (rot1, axis).into();
        let rot2: Rotation = (rot2, axis).into();

        self.turn_face::<0>(rot1).turn_face::<2>(rot2)
    }

    pub fn by_id(&self, mutations: &[Rotation; 20]) -> Cube<Id> {
        Cube::new(mutations.into_iter()
            .enumerate()
            .map(|(i, rot)| index(shift_coords(coords(i), rot.inverse())))
            .map(|ind| self.cubelets[ind])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        )
    }

    // pub fn difference(&self, other: &Self) -> Self {
    //     // FP stands for fixed-position notation
    //     let fp_self = self.shift(&self.cubelets).cubelets;
    //     let diff = Cube::<>::new(fp_self.iter()
    //         .zip(other.shift(&other.cubelets).cubelets.iter())
    //         .map(|(a, &b)| a.difference(b))
    //         .collect::<Vec<_>>()
    //         .try_into()
    //         .unwrap()
    //     );
    //     let fp_x = diff.shift(&fp_self);
    //     fp_x.shift(&fp_x.cubelets)
    // }
}

impl Cube<Id> {
    pub fn by_position(&self, mutations: &[Rotation; 20]) -> Cube<Position> {
        Cube::new(mutations.into_iter()
            .enumerate()
            .map(|(i, &rot)| index(shift_coords(coords(i), rot)))
            .map(|ind| self.cubelets[ind])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
        )
    }
}

impl<T: SortBy> Cube<T> {
    #[inline]
    pub fn new(cubelets: [Rotation; 20]) -> Self {
        Self { cubelets, _phantom: PhantomData::default() }
    }

    pub fn parity(&self) -> u8 {
        self.cubelets.iter().map(|r| r.len()).sum()
    }
}

// Add all feasible paths to the Vec `paths`
// struct CubePath
// to be replaced with a Word

#[cfg(test)]
mod tests {
    #[test]
    fn test_move_back_and_forth() {
        use rand::{thread_rng, Rng};
        use super::*;

        let mut cube = Cube::default();
        let cube = &mut cube;
        let mut thread = thread_rng();
        let moves: Vec<usize> = (0..20).map(|_| thread.gen_range(0..45)).collect();
        moves.into_iter().for_each(|ind| { *cube = cube.clone().make_move(Move::ALL[ind]); });

        for m in Move::ALL {
            let inverse = m.inverse();
            assert_eq!(cube.clone().make_move(m).make_move(inverse), *cube);
        }
    }
}
