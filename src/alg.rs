use crate::graph::Graph;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::sync::mpsc::{channel, Sender};
use std::collections::HashSet;

pub fn find_cycle(graph: Graph) -> Option<Vec<usize>> {
    let pool: Arc<Mutex<ThreadPool>> = Arc::new(Mutex::new(ThreadPool::new(2)));
    let (tx, rx) = channel();

    let graph_arc: Arc<Graph> = Arc::new(graph);
    let current: usize = 0;
    let path: Vec<usize> = Vec::from([current]);
    let visited_nodes: HashSet<usize> = HashSet::from([current]);
    let mut found: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let pool_arc: Arc<Mutex<ThreadPool>> = Arc::clone(&pool);
    let tx_arc: Arc<Mutex<Sender<Option<Vec<usize>>>>> = Arc::new(Mutex::new(tx)); 
    
    let pool_guard = pool.lock().unwrap();
    pool_guard.execute(move || {
        parse_graph(graph_arc, current, path, visited_nodes, &mut found, pool_arc, tx_arc);
    });
    drop(pool_guard);
    return rx.recv().unwrap();
}

fn parse_graph(graph: Arc<Graph>, current: usize, path: Vec<usize>, visited_nodes: HashSet<usize>, found: &mut Arc<Mutex<bool>>, pool: Arc<Mutex<ThreadPool>>, tx: Arc<Mutex<Sender<Option<Vec<usize>>>>>) {
    let guard = found.lock().unwrap();
    if *guard == true {
        return;
    }
    drop(guard);

    if visited_nodes.len() == graph.no_of_nodes {
        if graph.edges[current].iter().find(|&&x| x == 0) != None {
            let mut path_plus: Vec<usize> = path.clone();
            path_plus.push(0);
            *found.lock().unwrap() = true;
            tx.lock().unwrap().send(Option::from(path_plus)).unwrap();
        }
        return;
    }

    for neighbour in graph.edges[current].iter() {       
        if visited_nodes.get(neighbour) == None { 
            let graph_copy: Arc<Graph> = Arc::clone(&graph);
            let new_current: usize = *neighbour;
            let mut path_plus: Vec<usize> = path.clone();
            path_plus.push(*neighbour);
            let mut visited_nodes_plus: HashSet<usize> = visited_nodes.clone();
            visited_nodes_plus.insert(*neighbour);
            let mut found_copy: Arc<Mutex<bool>> = Arc::clone(&found);
            let pool_copy: Arc<Mutex<ThreadPool>> = Arc::clone(&pool);
            let tx_copy: Arc<Mutex<Sender<Option<Vec<usize>>>>> = Arc::clone(&tx); 
            
            let pool_guard = pool.lock().unwrap();
            pool_guard.execute(move || {
                parse_graph(graph_copy, new_current, path_plus, visited_nodes_plus, &mut found_copy, pool_copy, tx_copy);
            });
            drop(pool_guard);
        }
    }
}
