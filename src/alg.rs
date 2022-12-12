use crate::graph::Graph;
use std::sync::{Arc, Mutex, self};
use threadpool::ThreadPool;
use std::sync::mpsc::{channel, Sender};

pub fn find_cycle(graph: Graph) -> Option<Vec<usize>> {
    let pool: Arc<Mutex<ThreadPool>> = Arc::new(Mutex::new(ThreadPool::new(2)));
    let (tx, rx) = channel();

    let graph_arc: Arc<Graph> = Arc::new(graph);
    let current: usize = 0;
    let visited_nodes: Vec<usize> = Vec::from([current]);
    let mut found: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let pool_arc: Arc<Mutex<ThreadPool>> = Arc::clone(&pool);
    let tx_arc: Arc<Mutex<Sender<Option<Vec<usize>>>>> = Arc::new(Mutex::new(tx)); 

    let pool_guard = pool.lock().unwrap();
    pool_guard.execute(move || {
        parse_graph(graph_arc, current, visited_nodes, &mut found, pool_arc, tx_arc);
    });
    drop(pool_guard);
    // pool.lock().unwrap().join();
    return rx.recv().unwrap();
}

fn parse_graph(graph: Arc<Graph>, current: usize, visited_nodes: Vec<usize>, found: &mut Arc<Mutex<bool>>, pool: Arc<Mutex<ThreadPool>>, tx: Arc<Mutex<Sender<Option<Vec<usize>>>>>) {
    let guard = found.lock().unwrap();
    if *guard == true {
        return;
    }
    drop(guard);

    for neighbour in graph.edges[current].iter() {       
        if visited_nodes.len() == graph.no_of_nodes && *neighbour == 0 {
            let mut visited_nodes_plus: Vec<usize> = visited_nodes.clone();
            visited_nodes_plus.push(0);
            *found.lock().unwrap() = true;
            println!("plm");
            tx.lock().unwrap().send(Option::from(visited_nodes_plus)).unwrap();
            return;
        }

        if visited_nodes.iter().find(|&&x| x == *neighbour) == None {    
            let graph_copy: Arc<Graph> = Arc::clone(&graph);
            let new_current: usize = *neighbour;
            let mut visited_nodes_plus: Vec<usize> = visited_nodes.clone();
            visited_nodes_plus.push(*neighbour);
            let mut found_copy: Arc<Mutex<bool>> = Arc::clone(&found);
            let pool_copy: Arc<Mutex<ThreadPool>> = Arc::clone(&pool);
            let tx_copy: Arc<Mutex<Sender<Option<Vec<usize>>>>> = Arc::clone(&tx); 
            
            let pool_guard = pool.lock().unwrap();
            pool_guard.execute(move || {
                parse_graph(graph_copy, new_current, visited_nodes_plus, &mut found_copy, pool_copy, tx_copy);
            });
            drop(pool_guard);
        }
    }
}
