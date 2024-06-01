// I want to put wrappers here that will change the way different things are formatted to strings
use std::fmt::Display;

use crate::cube::{Cube, SortBy, index};

pub struct DisplayCube<T: SortBy>(pub Cube<T>);

impl<T: SortBy> Display for DisplayCube<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = &self.0.cubelets;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}",
            pad_right_to(&c[index([0,2,2])], 8),
            pad_right_to(&c[index([0,1,2])], 8),
            pad_right_to(&c[index([0,0,2])], 8),
            pad_right_to(&c[index([1,0,2])], 8),
            pad_right_to(&c[index([2,0,2])], 8),
            pad_right_to(&c[index([2,1,2])], 8),
            pad_right_to(&c[index([2,2,2])], 8),
            pad_right_to(&c[index([1,2,2])], 8),
        )?;
        writeln!(
            f,
            "{}O       {}G       {}R       {}B       ",
            pad_right_to(&c[index([0,2,1])], 8),
            pad_right_to(&c[index([0,0,1])], 8),
            pad_right_to(&c[index([2,0,1])], 8),
            pad_right_to(&c[index([2,2,1])], 8),
        )?;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}",
            pad_right_to(&c[index([0,2,0])], 8),
            pad_right_to(&c[index([0,1,0])], 8),
            pad_right_to(&c[index([0,0,0])], 8),
            pad_right_to(&c[index([1,0,0])], 8),
            pad_right_to(&c[index([2,0,0])], 8),
            pad_right_to(&c[index([2,1,0])], 8),
            pad_right_to(&c[index([2,2,0])], 8),
            pad_right_to(&c[index([1,2,0])], 8),
        )?;
        Ok(())
    }
}


// pub fn pad_outside(mut this: String, num: u8) -> String {
//     let mut s = " ".repeat(num as usize);
//     s.push_str(&format!("{:?}", this));
//     s.push_str(&" ".repeat(num as usize));
//     s
// }

pub fn pad_right_to<T: Display>(this: &T, num: u8) -> String {
    let mut this = format!("{}", this);
    let diff = (num as usize).saturating_sub(this.len());
    this.push_str(&" ".repeat(diff));
    this
}
