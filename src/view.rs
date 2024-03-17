// I want to put wrappers here that will change the way different things are formatted to strings
use std::{fmt::Display, io, str::FromStr};

use crate::{cube::{Move, Cube, index}, cubelet::Rotation};

pub struct MovesList<'a>(pub &'a [Move]);

impl Display for MovesList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.0 {
            writeln!(f, "{}", m)?;
        }
        Ok(())
    }
}

pub struct DisplayCube(pub Cube);

impl Display for DisplayCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = &self.0.cubelets;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}",
            pad_right_to(&c[index::<0,2,2>()], 8),
            pad_right_to(&c[index::<0,1,2>()], 8),
            pad_right_to(&c[index::<0,0,2>()], 8),
            pad_right_to(&c[index::<1,0,2>()], 8),
            pad_right_to(&c[index::<2,0,2>()], 8),
            pad_right_to(&c[index::<2,1,2>()], 8),
            pad_right_to(&c[index::<2,2,2>()], 8),
            pad_right_to(&c[index::<1,2,2>()], 8),
        )?;
        writeln!(
            f,
            "{}O       {}G       {}R       {}B       ",
            pad_right_to(&c[index::<0,2,1>()], 8),
            pad_right_to(&c[index::<0,0,1>()], 8),
            pad_right_to(&c[index::<2,0,1>()], 8),
            pad_right_to(&c[index::<2,2,1>()], 8),
        )?;
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}",
            pad_right_to(&c[index::<0,2,0>()], 8),
            pad_right_to(&c[index::<0,1,0>()], 8),
            pad_right_to(&c[index::<0,0,0>()], 8),
            pad_right_to(&c[index::<1,0,0>()], 8),
            pad_right_to(&c[index::<2,0,0>()], 8),
            pad_right_to(&c[index::<2,1,0>()], 8),
            pad_right_to(&c[index::<2,2,0>()], 8),
            pad_right_to(&c[index::<1,2,0>()], 8),
        )?;
        Ok(())
    }
}


impl FromStr for Cube {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 20 {
            let chars: [u8; 20] = s.as_bytes().try_into().unwrap();
            let mut cubelets: [Rotation; 20] = Default::default();
            for (i, b) in chars.into_iter().enumerate() {
                let val = b - b'A';
                cubelets[i] = val.try_into().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, s))?;
            }
            Ok(Cube { cubelets })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, s))
        }
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
