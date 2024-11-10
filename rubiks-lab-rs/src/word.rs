use std::{fmt::Display, marker::PhantomData};
use crate::prelude::*;

// 
#[derive(Clone, Debug)]
pub struct Word<T> {
    pub actions: Vec<Move>,
    pub cube: Cube<Position>,
    _phantom: PhantomData<T>,
}

impl<T: Action> Display for Word<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actions = self.as_actions();
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

impl<T: Action> Default for Word<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> From<T> for Word<A>
where
    T: IntoIterator<Item = A>,
    A: Action
{
    fn from(value: T) -> Self {
        let mut word = Self::new();
        for a in value {
            word.make_move(a);
        }
        word
    }
}

impl<T: Action> Word<T> {
    pub fn new() -> Self {
        Self {
            actions: vec![],
            cube: Cube::default(),
            _phantom: PhantomData
        }
    }

    pub fn as_actions(&self) -> Vec<T> {
        self.actions.iter()
            .flat_map(|&m| T::from_move(m))
            .collect()
    }

    pub fn from_parts_unchecked(cube: Cube<Position>, actions: Vec<Move>) -> Self {
        Self { cube, actions, ..Default::default() }
    }

    pub fn make_move(&mut self, action: T) {
        self.actions.push(action.into());
        self.cube = self.cube.clone().make_move(action.into());
    }

    // We'll keep the word expanded until the user says otherwise
    pub fn normal_form(self) -> Self {
        let mut actions = vec![];
        let cube = self.cube;
        let mut old = self.actions.into_iter();

        let mut first = old.next();
        let mut second = old.next();
        loop {
            match (first, second) {
                (None, _) => { break }
                (Some(left), None) => { actions.push(left); break }
                (Some(left), Some(right)) => {
                    match Move::reduce(left, right) {
                        (None, None) => {
                            first = old.next();
                            second = old.next();
                        }
                        (Some(new_left), None) => {
                            first = Some(new_left);
                            second = old.next();
                        }
                        (Some(same_left), Some(same_right)) => {
                            actions.push(same_left);
                            first = Some(same_right);
                            second = old.next();
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }


        Self::from_parts_unchecked(cube, actions)
    }

    pub fn cube(&self) -> &Cube<Position> {
        &self.cube
    }
}

impl<A: Action> Extend<A> for Word<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter().for_each(|action| {
            let m = action.into();
            self.cube = self.cube.clone().make_move(m);
            self.actions.push(m);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Axis, Move};

    #[test]
    fn test_word_stuff() {

        let mut word = Word::new();
        word.make_move(Move(Axis::X, 0, 1));
        word.make_move(Move(Axis::X, 0, 3));
        word.make_move(Move(Axis::Y, 0, 1));
        word.make_move(Move(Axis::Y, 0, 1));
        word.make_move(Move(Axis::Z, 0, 1));
        word.make_move(Move(Axis::Z, 0, 1));
        word.make_move(Move(Axis::Z, 0, 1));
        word.make_move(Move(Axis::Z, 0, 1));
        word.make_move(Move(Axis::X, 0, 3));
        word.make_move(Move(Axis::X, 1, 3));
        word.make_move(Move(Axis::X, 1, 3));
        assert_eq!(word.actions.len(), 11);

        let Word { cube, actions, .. } = word.normal_form();
        let test_cube = Cube::default()
            .make_move(Move(Axis::Y, 0, 2))
            .make_move(Move(Axis::X, 2, 1));
        let test_actions = vec![
            Move(Axis::Y, 0, 2),
            Move(Axis::X, 2, 1)
        ];

        assert_eq!(cube, test_cube);
        assert_eq!(actions, test_actions);
    }
}
