use std::{fmt::Display, marker::PhantomData};
use crate::{Action, Move, Cube, Position};

#[derive(Clone, Debug)]
pub struct Word<T> {
    pub actions: Vec<Move>,
    pub cube: Cube<Position>,
    _phantom: PhantomData<T>,
}

impl<T: Action> Display for Word<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions = self.actions.iter()
            .flat_map(|&m| T::from_move(m))
            .collect::<Vec<_>>();
        write!(f, "(")?;
        for (i, a) in actions.into_iter().enumerate() {
            if i == 0 {
                write!(f, "{}", a)?;
            } else {
                write!(f, " {}", a)?;
            }
        }
        write!(f, ")")
    }
}

impl<T: Action> Word<T> {
    fn multiply(&mut self, action: T) {
        let m = action.into();
        if let Some(last) = self.actions.last_mut() {
            if last.0 == m.0 {
                let new = Move(last.0, (last.1 + m.1) % 4, (last.2 + m.2) % 4);
                if new.1 == 0 && new.2 == 0 {
                    self.actions.pop();
                } else {
                    *last = new;
                }
            } else {
                self.actions.push(m);
            }
        } else {
            self.actions.push(action.into());
        }
        self.cube = self.cube.clone().make_move(action.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_stuff() {
        todo!()
    }
}
