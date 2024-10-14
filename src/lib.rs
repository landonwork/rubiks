pub mod action;
pub use action::{Action, Move, Turn, QuarterTurn};
pub mod book;
pub mod cube;
pub use cube::{Cube, Position};
pub mod cubelet;
pub use cubelet::{Rotation, Axis};
pub mod strategy;
pub mod view;
pub mod word;
pub use word::Word;
// TODO: make compatible with laion/strategic_game_cube dataset to use as a benchmark
// and to get results comparable with anyone else's for this very niche problem.
// pub mod strategic_game_cube;
//

