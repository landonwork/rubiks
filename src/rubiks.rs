use std::fmt::Display;

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
//     as_cubelets: CubeletsArrangement,
// }

/// Number of turns on the most negative face, number of turns on the most positive face,
/// and the axis on which the turns happen
/// #[derive(Debug)]
pub struct Move(pub u8, pub u8, pub Axis);

/// A Rubiks' cube arrangement, represented by the rotation of
/// the cubelets relative to the solved arrangement. Each cubelet
/// is represented in the place where it is currently. Face centers
/// and the middle-middle-middle piece are never changed from Rotation::Neutral
/// This is probably the most practical memory layout.
#[derive(Debug, Default)]
pub struct CubeletsArrangement {
    cubelets: [[[Rotation; 3]; 3] ;3]
}

impl Display for CubeletsArrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = &self.cubelets;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[0][2][2], c[0][1][2], c[0][0][2], c[1][0][2], c[2][0][2], c[2][1][2], c[2][2][2], c[1][2][2],
        )?;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[0][2][1], "O", c[0][0][1], "G", c[2][0][1], "R", c[2][2][1], "B",
        )?;
        writeln!(
            f,
            "{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}\t{:?}",
            c[0][2][0], c[0][1][0], c[0][0][0], c[1][0][0], c[2][0][0], c[2][1][0], c[2][2][0], c[1][2][0],
        )?;
        Ok(())
    }
}

impl CubeletsArrangement {
    pub fn turn_face<const NEG: bool>(&mut self, rot: Rotation) {
        let c = &mut self.cubelets;
        let sign = if NEG { 0 } else { 2 };
        match rot {
            Rotation::Neutral => {}
            Rotation::X => {
                // corners
                (c[sign][2][2], c[sign][0][2], c[sign][0][0], c[sign][2][0]) = (
                    c[sign][2][0].compose(rot),
                    c[sign][2][2].compose(rot),
                    c[sign][0][2].compose(rot),
                    c[sign][0][0].compose(rot)
                );
                // edges
                (c[sign][1][2], c[sign][0][1], c[sign][1][0], c[sign][2][1]) = (
                    c[sign][2][1].compose(rot),
                    c[sign][1][2].compose(rot),
                    c[sign][0][1].compose(rot),
                    c[sign][1][0].compose(rot)
                );
            }
            Rotation::X2 => {
                // corners
                (c[sign][2][2], c[sign][0][2], c[sign][0][0], c[sign][2][0]) = (
                    c[sign][0][0].compose(rot),
                    c[sign][2][0].compose(rot),
                    c[sign][2][2].compose(rot),
                    c[sign][0][2].compose(rot)
                );
                // edges
                (c[sign][1][2], c[sign][0][1], c[sign][1][0], c[sign][2][1]) = (
                    c[sign][1][0].compose(rot),
                    c[sign][2][1].compose(rot),
                    c[sign][1][2].compose(rot),
                    c[sign][0][1].compose(rot)
                );
            }
            Rotation::X3 => {
                // corners
                (c[sign][2][2], c[sign][0][2], c[sign][0][0], c[sign][2][0]) = (
                    c[sign][0][2].compose(rot),
                    c[sign][0][0].compose(rot),
                    c[sign][2][0].compose(rot),
                    c[sign][2][2].compose(rot)
                );
                // edges
                (c[sign][1][2], c[sign][0][1], c[sign][1][0], c[sign][2][1]) = (
                    c[sign][0][1].compose(rot),
                    c[sign][1][2].compose(rot),
                    c[sign][2][1].compose(rot),
                    c[sign][1][0].compose(rot)
                );
            }
            Rotation::Y => {
                // corners
                (c[2][sign][2], c[0][sign][2], c[0][sign][0], c[2][sign][0]) = (
                    c[0][sign][2].compose(rot),
                    c[0][sign][0].compose(rot),
                    c[2][sign][0].compose(rot),
                    c[2][sign][2].compose(rot)
                );
                // edges
                (c[1][sign][2], c[0][sign][1], c[1][sign][0], c[2][sign][1]) = (
                    c[0][sign][1].compose(rot),
                    c[1][sign][0].compose(rot),
                    c[2][sign][1].compose(rot),
                    c[1][sign][2].compose(rot)
                );
            }
            Rotation::Y2 => {
                // corners
                (c[2][sign][2], c[0][sign][2], c[0][sign][0], c[2][sign][0]) = (
                    c[0][sign][0].compose(rot),
                    c[2][sign][0].compose(rot),
                    c[2][sign][2].compose(rot),
                    c[0][sign][2].compose(rot)
                );
                // edges
                (c[1][sign][2], c[0][sign][1], c[1][sign][0], c[2][sign][1]) = (
                    c[1][sign][0].compose(rot),
                    c[2][sign][1].compose(rot),
                    c[1][sign][2].compose(rot),
                    c[0][sign][1].compose(rot)
                );
            }
            Rotation::Y3 => {
                // corners
                (c[2][sign][2], c[0][sign][2], c[0][sign][0], c[2][sign][0]) = (
                    c[2][sign][0].compose(rot),
                    c[2][sign][2].compose(rot),
                    c[0][sign][2].compose(rot),
                    c[0][sign][0].compose(rot)
                );
                // edges
                (c[1][sign][2], c[0][sign][1], c[1][sign][0], c[2][sign][1]) = (
                    c[2][sign][1].compose(rot),
                    c[1][sign][2].compose(rot),
                    c[0][sign][1].compose(rot),
                    c[1][sign][0].compose(rot)
                );
            }
            Rotation::Z => {
                // corners
                (c[2][2][sign], c[0][2][sign], c[0][0][sign], c[2][0][sign]) = (
                    c[2][0][sign].compose(rot),
                    c[2][2][sign].compose(rot),
                    c[0][2][sign].compose(rot),
                    c[0][0][sign].compose(rot)
                );
                // edges
                (c[1][2][sign], c[0][1][sign], c[1][0][sign], c[2][1][sign]) = (
                    c[2][1][sign].compose(rot),
                    c[1][2][sign].compose(rot),
                    c[0][1][sign].compose(rot),
                    c[1][0][sign].compose(rot)
                );
            }
            Rotation::Z2 => {
                // corners
                (c[2][2][sign], c[0][2][sign], c[0][0][sign], c[2][0][sign]) = (
                    c[0][0][sign].compose(rot),
                    c[2][0][sign].compose(rot),
                    c[2][2][sign].compose(rot),
                    c[0][2][sign].compose(rot)
                );
                // edges
                (c[1][2][sign], c[0][1][sign], c[1][0][sign], c[2][1][sign]) = (
                    c[1][0][sign].compose(rot),
                    c[2][1][sign].compose(rot),
                    c[1][2][sign].compose(rot),
                    c[0][1][sign].compose(rot)
                );
            }
            Rotation::Z3 => {
                // corners
                (c[2][2][sign], c[0][2][sign], c[0][0][sign], c[2][0][sign]) = (
                    c[0][2][sign].compose(rot),
                    c[0][0][sign].compose(rot),
                    c[2][0][sign].compose(rot),
                    c[2][2][sign].compose(rot)
                );
                // edges
                (c[1][2][sign], c[0][1][sign], c[1][0][sign], c[2][1][sign]) = (
                    c[0][1][sign].compose(rot),
                    c[1][0][sign].compose(rot),
                    c[2][1][sign].compose(rot),
                    c[1][2][sign].compose(rot)
                );
            }
            _ => unreachable!()
        }
    }

    pub fn make_move(&mut self, Move(rot1, rot2, axis): Move) {
        let rot1: Rotation = (rot1, axis).into();
        let rot2: Rotation = (rot2, axis).into();

        self.turn_face::<true>(rot1);
        self.turn_face::<false>(rot2);
    }
}

