use std::collections::HashSet;
use rand::Rng;

#[derive(Debug)]
pub struct Graph {
    pub edges: Vec<Vec<usize>>,
    pub no_of_nodes: usize
}

impl Graph {
    pub fn generate(no_of_nodes: usize) -> Graph {
        let mut graph: Graph = Graph{
            no_of_nodes: no_of_nodes,
            edges: vec![]
        };

        for i in 0..no_of_nodes {
            graph.edges.push(vec![]);

            let mut rng = rand::thread_rng();
            let edges_no: usize = rng.gen_range(1..no_of_nodes);

            let mut neighbours: HashSet<usize> = HashSet::new();
            for _ in 0..edges_no {
                let mut neighbour: usize = rng.gen_range(0..no_of_nodes);
                while neighbours.get(&neighbour) != None || neighbour == i {
                    neighbour = rng.gen_range(0..no_of_nodes);
                }
                graph.edges[i].push(neighbour);
                neighbours.insert(neighbour);
            }
        }
        
        return graph;
    }
}