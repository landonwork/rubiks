// I want to put wrappers here that will change the way different things are formatted to strings
use std::fmt::Display;

use crate::rubiks::Move;

pub struct MovesList<'a>(pub &'a [Move]);

impl Display for MovesList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0 {
            writeln!(f, "{}{}{}", m.2, m.0, m.1)?;
        }
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
