#[macro_use]
extern crate log;
extern crate env_logger;
extern crate failure;
extern crate programming_assignments;
extern crate regex;

use std::collections::{HashSet,HashMap};
    use std::iter::FromIterator;

fn main() {
    run_main().unwrap();
}
fn run_main() -> Result<(), failure::Error> {
    env_logger::init();

    let file = programming_assignments::get_argv1()?;
    let file = std::path::Path::new(&file);
    let mut file = std::fs::File::open(file)?;
    let mut string = String::new();

    use std::io::Read;
    info!("Reading file {:?}", file);
    file.read_to_string(&mut string)?;
    let string: Vec<Vec<String>> = string
        .replace(" ", "")
        .split("\n")
        .map(|l| l.split(",").map(|s| s.to_string()).collect())
        .collect();

    let mut nodes_map = HashMap::new();
    let mut nodes_map_rev = HashMap::new();
    let mut nodes = HashMap::new();
    for (idx,symbol) in string[0].iter().enumerate() {
        nodes_map_rev.insert(idx,symbol);
        nodes_map.insert(symbol,idx);
        nodes.insert(idx, mut HashSet::new());
    }
    let (n_start, n_end) = (nodes_map[&string[1][0]], nodes_map[&string[1][1]]);

    for line in string[2..].iter() {
        use std::str::FromStr;
        let (src, dst, weight) = (
            nodes_map[&line[0]],
            nodes_map[&line[1]],
            i32::from_str(&line[2])?,
        );
        nodes[&src].insert((dst,weight));
        nodes[&dst].insert((src,weight));
    }
    let (distance, path) = dijkstra(nodes, n_start, n_end)?;
    let path = path
        .iter()
        .map(|l|nodes_map_rev[l])
        .fold(String::new(), |acc, x| acc + ", " + x)[2..].to_string();
    println!("{}", distance);
    println!("{}", path);
    Ok(())
}

fn dijkstra(
    nodes: HashMap<usize, HashSet<(usize,i32)>>,
    start: usize,
    end: usize,
) -> Result<(i32, Vec<usize>), failure::Error> {
    let mut distances = HashMap::new();
    for node in nodes.keys(){
        distances.insert(node, i32::max_value());
    }
    distances[&start] = 0;
    let mut visited_nodes = HashSet::new();
    let mut queue: HashSet<&usize> = HashSet::from_iter(nodes.keys());

    while queue.len()>0 {
        if let Some((min_key,min_val)) = queue.iter().map(|n|(n,distances[n])).min(){
            let node = &nodes[min_key];
            visited_nodes.insert(min_key);
            for (neighbour,weight) in node{
                if distances[min_key] + weight < distances[&neighbour]{
                    distances[&neighbour] += weight;
                }
            }
        }
    }
    //Find the node with the shortest distance
    // Add it to visited_nodes
    //iter over its neighbours
        // if distance to the neighbour > 

    Ok((0, vec![0, 1, 2]))
}
