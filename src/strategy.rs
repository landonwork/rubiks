// How necessary is it for me to figure out the iso/automorphisms in order to reduce my search
// space? Can I continue using a HashMap or do I need to actually choose a more formal data
// structure?

use std::{
    thread::{JoinHandle, self},
    // collections::HashMap
};

use crate::{
    action::Move,
    cube::{Cube, Position},
    cubelet::Axis,
    book::Book,
    // view::DisplayCube,
};

pub trait Strategy {
    fn explore(&self, book: &mut Book, cube: &Cube<Position>);
}

// Depth-first brute force search (slightly optimized)
#[derive(Clone)]
pub struct Tree {
    pub prev_move: Option<Move>,
    pub current_depth: u8,
    pub search_depth: u8,
}

impl Strategy for Tree {
    fn explore(&self, book: &mut Book, cube: &Cube<Position>) {
        // if self.current_depth >= self.search_depth {
        //     return
        // }

        // let iter: Box<dyn Iterator<Item = Move>> = match self.prev_move {
        //     None => Box::new(Move::ALL.into_iter()),
        //     Some(Move(_, _, Axis::X)) => Box::new(Move::Y.into_iter().chain(Move::Z)),
        //     Some(Move(_, _, Axis::Y)) => Box::new(Move::X.into_iter().chain(Move::Z)),
        //     Some(Move(_, _, Axis::Z)) => Box::new(Move::X.into_iter().chain(Move::Y)),
        // };

        // iter.for_each(|m| {
        //     let new_cube = cube.clone().make_move(m); 
        //     match book.get_mut(&new_cube) {
        //         Some(depth_ref_mut) if *depth_ref_mut > self.current_depth + 1 => {
        //             *depth_ref_mut = self.current_depth + 1;
        //             let new_strat = Tree {
        //                 prev_move: Some(m),
        //                 current_depth: self.current_depth + 1,
        //                 search_depth: self.search_depth
        //             };
        //             new_strat.explore(book, &new_cube);
        //         }
        //         Some(_) => {  }
        //         None => {
        //             book.insert(new_cube.clone(), self.current_depth + 1);
        //             let new_strat = Tree {
        //                 prev_move: Some(m),
        //                 current_depth: self.current_depth + 1,
        //                 search_depth: self.search_depth
        //             };
        //             new_strat.explore(book, &new_cube);
        //         }
        //     }
        // });
    }
}

// Thread pool, because scope was being incredibly slow
struct ThreadPool<T> {
    threads: Vec<Option<JoinHandle<T>>>
}

impl<T: Send + 'static> ThreadPool<T> {
    fn new() -> Self {
        let num_cpus = thread::available_parallelism().unwrap().get();
        let threads = (0..num_cpus).map(|_| None).collect();
        Self { threads }
    }

    fn spawn_all(&mut self, spawner: impl Iterator<Item=Box<dyn Send + FnOnce() -> T>>) -> Vec<T> {
        let mut results = Vec::new();
        let mut spawner = spawner.peekable();

        'outer: loop {
            if spawner.peek().is_some() {
                for i in 0..self.threads.len() {
                    match &self.threads[i] {
                        None => {
                            let closure = spawner.next().unwrap();
                            let parent_handle = thread::current();

                            let handle = thread::spawn(move || {
                                let result = closure();
                                parent_handle.unpark();
                                result
                            });
                            self.threads[i] = Some(handle);

                            continue 'outer
                        }
                        Some(handle) if handle.is_finished() => {
                            let closure = spawner.next().unwrap();
                            let parent_handle = thread::current();

                            let handle = thread::spawn(move || {
                                let result = closure();
                                parent_handle.unpark();
                                result
                            });
                            let finished_handle = std::mem::replace(&mut self.threads[i], Some(handle));
                            results.push(finished_handle.unwrap().join().unwrap());

                            continue 'outer
                        }
                        _ => {}
                    }
                }
            } else {
                let mut finished = 0;
                for i in 0..self.threads.len() {
                    match &self.threads[i] {
                        None => { finished += 1; }
                        Some(handle) if handle.is_finished() => {
                            let finished_handle = std::mem::replace(&mut self.threads[i], None);
                            results.push(finished_handle.unwrap().join().unwrap());
                            finished += 1;
                        }
                        _ => {}
                    }
                }
                if finished == self.threads.len() {
                    break 'outer
                }
            }

            thread::park();
        }

        results
    }
}

// Parallelized depth-first search
// Creates partial trees from `jobs` and then spawns multiple `Tree` search strategies
pub struct MultiTree {
    pub search_depth: u8,
    pub jobs: Vec<Vec<Move>>
}

impl Strategy for MultiTree {
    fn explore(&self, book: &mut Book, cube: &Cube<Position>) {
        // // Getting things set up
        // let starts: Vec<_> = self.jobs.iter()
        //     .map(|moves| {
        //         let last_move = moves.last().clone();
        //         let (cube, depth) = moves.into_iter().fold(
        //             (cube.clone(), 0),
        //             |(acc, d), &m| {
        //                 let new = acc.make_move(m);
        //                 book.insert(new.clone(), d + 1);
        //                 (new, d + 1)
        //             });
        //         (cube, depth, last_move)
        //     })
        //     .collect();


        // // Spawn all those tasks
        // let mut pool = ThreadPool::new();
        // let spawner = starts.into_iter()
        //     .map(|(start, depth, last_move)| {
        //         let cap = book.capacity() / self.jobs.len();
        //         let mut inner = HashMap::with_capacity(cap);
        //         inner.extend(book.iter().map(|(a, b)| (a.clone(), b.clone())));
        //         let mut local_book = Book(inner);
        //         let tree = Tree {
        //             prev_move: last_move.copied(),
        //             current_depth: depth,
        //             search_depth: self.search_depth
        //         };
        //         let closure: Box<dyn Send + FnOnce() -> Book> = Box::new(move || {
        //             local_book.expand(tree, vec![Box::new(move |cube, _depth| cube == &start)]);
        //             local_book
        //         });
        //         closure
        //     });
        // let books = pool.spawn_all(spawner);

        // for other in books {
        //     book.extend_from_book(other);
        // }
    }
}

// Could this be useful? Is there ever time I might equally want an Axis or a Move?
pub enum BranchType {
    Axis(Axis),
    Move(Move),
}

// A "pruned" `Tree` strategy which only considers paths that follow the order of a sequence
// of axes.
pub struct PartialTree {
    pub axes: Vec<Axis>,
    pub current_depth: u8,
}

impl Strategy for PartialTree {
    fn explore(&self, book: &mut Book, cube: &Cube<Position>) {
        // if self.axes.is_empty() {
        //     return
        // }

        // let iter = match self.axes.first().unwrap() {
        //     Axis::X => Move::X,
        //     Axis::Y => Move::Y,
        //     Axis::Z => Move::Z,
        // };

        // iter.into_iter().for_each(|m| {
        //     let new_cube = cube.clone().make_move(m); 
        //     match book.get_mut(&new_cube) {
        //         Some(depth_ref_mut) if *depth_ref_mut > self.current_depth + 1 => {
        //             *depth_ref_mut = self.current_depth + 1;
        //             let new_strat = PartialTree {
        //                 axes: self.axes[1..].to_vec(),
        //                 current_depth: self.current_depth + 1,
        //             };
        //             new_strat.explore(book, &new_cube);
        //         }
        //         Some(_) => {  }
        //         None => {
        //             book.insert(new_cube.clone(), self.current_depth + 1);
        //             let new_strat = PartialTree {
        //                 axes: self.axes[1..].to_vec(),
        //                 current_depth: self.current_depth + 1,
        //             };
        //             new_strat.explore(book, &new_cube);
        //         }
        //     }
        // });
    }
}

// Repeats a sequence of `Move`s until returning to a solved state
pub struct Cycle {
    pub moves: Vec<Move>
}

// TODO: Some cycles are too large to use a u8 for depth!
impl Strategy for Cycle {
    fn explore(&self, book: &mut Book, start_cube: &Cube<Position>) {
        // let mut depth = *book.get(start_cube).unwrap();
        // let mut cube = start_cube.clone();

        // for &m in self.moves.iter().cycle() {
        //     cube = cube.make_move(m);
        //     depth += 1;

        //     match book.get_mut(&cube) {
        //         // If we find we have better depth information than the
        //         // our current collected knowledge
        //         Some(depth_mut_ref) if *depth_mut_ref > depth => {
        //             // Update our knowledge to match our depth estimate
        //             *depth_mut_ref = depth;
        //             // Update that cube's neighbors
        //             Update.explore(book, &cube);
        //         }
        //         // If we are overestimating our current depth
        //         Some(depth_mut_ref) if *depth_mut_ref < depth => {
        //             println!("Whoops! We looped back on ourselves. Let's get our depth back on track!");
        //             // Update the depth to match our collected knowledge
        //             depth = *depth_mut_ref;
        //             if depth == 0 {
        //                 println!("Depth 0?\n{}", DisplayCube(cube.clone()));
        //             }
        //             // Update the path we just came from
        //             Update.explore(book, &cube);
        //         }
        //         // Nothing needs to be done
        //         Some(_) => {  }
        //         // Insert the cube
        //         None => {
        //             book.insert(cube.clone(), depth);
        //         }
        //     }

        //     // If we have returned to our original state (and updated all of our known info),
        //     // then we can stop. Otherwise, we just keep on going.
        //     if &cube == start_cube {
        //         break
        //     }
        // }
    }
}

// Adds all cubes along a single path
pub struct NewPath {
    pub path: Vec<Move>
}

// `Rivulet` in this case means "random walk with one of two stopping conditions"
// Search a random path with random chance to stop after each move
pub struct RivuletProb {
    pub p_stop: f32,
}

// Search a random path and stop at `max_depth`
pub struct RivuletMax {
    pub max_depth: u8,
}

// A `Rivulet`
pub enum Rivulet {
    Prob(RivuletProb),
    Max(RivuletMax)
}

// Spawn many `Rivulet` strategies in parallel
pub struct Rivulets {
    pub rivulets: Vec<Rivulets>
}

// Breadth-first search
pub struct Flood {
    pub search_depth: u8
}

// Updates all nearby cubes with more appropriate depth values
pub struct Update;

impl Strategy for Update {
    fn explore(&self, _book: &mut Book, _cube: &Cube<Position>) {
        // let depth = *book.get(cube).unwrap();
        // Move::ALL.into_iter()
        //     .for_each(|m| {
        //         let new_cube = cube.clone().make_move(m);
        //         match book.get_mut(&new_cube)  {
        //             Some(depth_mut_ref) if *depth_mut_ref > depth + 1 => {
        //                 *depth_mut_ref = depth + 1;
        //                 Update.explore(book, &new_cube);
        //             }
        //             // We might learn that even though we thought we were updating our current
        //             // location with the best information, there is actually better information out
        //             // there that we should be using
        //             Some(depth_mut_ref) if *depth_mut_ref < depth - 1 => {
        //                 let current_depth = *depth_mut_ref + 1;
        //                 book.insert(cube.clone(), current_depth);
        //                 Update.explore(book, cube);
        //             }
        //             _ => {  }
        //         }
        //     })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    // TODO: I wrote this to work on my personal laptop.
    // There's a way to write the logic for this for any number of cores,
    // I'm just not in the mood to figure it out
    #[test]
    fn test_pool() {
        let mut pool = ThreadPool::new();
        let spawner: Vec<_> = (0..10).map(|i| {
            let closure: Box<dyn Send + FnOnce() -> u64> = Box::new(
                move || {
                    thread::sleep(Duration::from_secs(i));
                    i
                }
            );
            closure
        }).collect();

        let begin = Instant::now();
        pool.spawn_all(spawner.into_iter());
        let duration = Instant::now().duration_since(begin);

        let expected_min = Duration::from_secs(15);
        assert!(expected_min < duration, "{duration:?}");
    }
}
