pub mod action;
pub mod book;
pub mod cube;
pub mod cubelet;
pub mod strategy;
pub mod view;
// TODO: make compatible with laion/strategic_game_cube dataset to use as a benchmark
// and to get results comparable with anyone else's for this very niche problem.
// pub mod strategic_game_cube;
//

#[macro_export]
macro_rules! as_bytes {
    ($reference:expr) => {
        let ptr: *const _ = $reference;
        unsafe { std::slice::from_raw_parts(ptr.cast(), std::mem::size_of()) }
    }
}
