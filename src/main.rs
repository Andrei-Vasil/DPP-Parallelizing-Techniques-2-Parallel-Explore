mod graph;
mod alg;
use graph::Graph;
use alg::find_cycle;

fn main() {
    // let x: Graph = Graph {
    //     no_of_nodes: 5,
    //     edges: Vec::from([
    //         Vec::from([2, 3, 4]),
    //         Vec::from([0, 3]),
    //         Vec::from([1, 3]),
    //         Vec::from([0, 1, 4]),
    //         Vec::from([0, 2])
    //     ])
    // };
    let x: Graph = Graph::generate(10);
    
    let result: Option<Vec<usize>> = find_cycle(x);
    if result == None {
        println!("No hamiltonian cycle.");
    } else {
        println!("Hamiltonian cycle detected: {:?}", result.unwrap());
    }
}
