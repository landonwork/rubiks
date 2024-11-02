pub mod action;
pub mod book;
pub mod cube;
pub mod cubelet;
pub mod strategy;
pub mod view;
pub mod word;
pub mod prelude;

// TODO: make compatible with laion/strategic_game_cube dataset to use as a benchmark
// and to get results comparable with anyone else's for this very niche problem.
// pub mod strategic_game_cube;


use pyo3::prelude::*;

#[pymodule]
fn rubik_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<book::PyBook>()?;
    Ok(())
}
