use crate::graph::Graph;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub fn find_cycle(graph: Graph) -> Option<Vec<usize>> {
    let graph_arc: Arc<Graph> = Arc::new(graph);
    let current: usize = 0;
    let visited_nodes: Vec<usize> = Vec::from([current]);
    let mut found: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    
    return thread::spawn(move || {
        return parse_graph(graph_arc, current, visited_nodes, &mut found);
    }).join().unwrap();
}

fn parse_graph(graph: Arc<Graph>, current: usize, visited_nodes: Vec<usize>, found: &mut Arc<Mutex<bool>>) -> Option<Vec<usize>> {
    let guard = found.lock().unwrap();
    if *guard == true {
        return None;
    }
    drop(guard);

    let mut handles: Vec<JoinHandle<Option<Vec<usize>>>> = vec![];

    for neighbour in graph.edges[current].iter() {
        if visited_nodes.len() == graph.no_of_nodes && *neighbour == 0 {
            let mut visited_nodes_plus: Vec<usize> = visited_nodes.clone();
            visited_nodes_plus.push(0);
            *found.lock().unwrap() = true;
            return Option::from(visited_nodes_plus);
        }
        
        if visited_nodes.iter().find(|&&x| x == *neighbour) == None {    
            let graph_copy: Arc<Graph> = Arc::clone(&graph);
            let new_current: usize = *neighbour;
            let mut visited_nodes_plus: Vec<usize> = visited_nodes.clone();
            visited_nodes_plus.push(*neighbour);
            let mut found_copy: Arc<Mutex<bool>> = Arc::clone(&found);
            
            let handle = thread::spawn(move || {
                return parse_graph(graph_copy, new_current, visited_nodes_plus, &mut found_copy);
            });
            handles.push(handle);
        }
    }

    let mut result: Option<Vec<usize>> = None; 
    for handle in handles {
        let answer = handle.join().unwrap();
        if answer != None {
            result = answer;
        }
    }

    return result;
}