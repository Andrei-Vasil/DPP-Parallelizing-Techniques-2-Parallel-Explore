use crate::graph::Graph;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::thread::{self, JoinHandle};

pub fn find_cycle(graph: Graph) -> Vec<usize> {
    let graph_arc: Arc<Graph> = Arc::new(graph);
    let current: usize = 0;
    let mut visited_nodes: HashSet<usize> = HashSet::new();
    visited_nodes.insert(current);
    let mut found: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    
    thread::spawn(move || {
        parse_graph(graph_arc, current, visited_nodes, &mut found);
    }).join().unwrap();
    return vec![];
}

fn parse_graph(graph: Arc<Graph>, current: usize, visited_nodes: HashSet<usize>, found: &mut Arc<Mutex<bool>>) {
    let guard = found.lock().unwrap();
    if *guard == true {
        return;
    }
    drop(guard);

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for neighbour in graph.edges[current].iter() {
        if visited_nodes.len() == graph.no_of_nodes && *neighbour == 0 {
            println!("{:?}", visited_nodes);
            *found.lock().unwrap() = true;
            return;
        }
        
        if visited_nodes.get(neighbour) == None {    
            let graph_copy: Arc<Graph> = Arc::clone(&graph);
            let new_current: usize = *neighbour;
            let mut visited_nodes_plus: HashSet<usize> = visited_nodes.clone();
            visited_nodes_plus.insert(*neighbour);
            let mut found_copy: Arc<Mutex<bool>> = Arc::clone(&found);
            
            let handle = thread::spawn(move || {
                parse_graph(graph_copy, new_current, visited_nodes_plus, &mut found_copy);
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

}