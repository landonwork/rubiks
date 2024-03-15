
use petgraph::{prelude::*, stable_graph::DefaultIx};
use rubiks::rubiks::{Cube, CubePath, Move};

type Graph = petgraph::graph::Graph<Cube, Move, Directed, DefaultIx>;

fn main() {

}

fn load_graph(file_path: &str) -> Result<Graph, std::io::Error> {
    let mut graph = Graph::new();
}

fn save_graph(graph: Graph, file_path: &str) -> Result<(), std::io::Error> {

    Ok(())
}
